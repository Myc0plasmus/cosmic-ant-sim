use gl::types::GLuint;
use gl::types::GLenum;
use gl::types::GLchar;
use gl::types::GLint;
use std::fs;
use std::fs::read;
use std::io::{self, Read};
use std::path::Path;
use std::ptr;
use std::ffi::{CStr, CString};

pub struct ShaderProgram {
   shader_program : GLuint,
   vertex_shader: GLuint,
   geometry_shader: Option<GLuint>,
   fragment_shader: GLuint,
}

impl ShaderProgram {
    

    pub fn new(vertex_path: &str, geometry_path: Option<&str>, fragment_path: &str) -> ShaderProgram {
        let vertex_shader = ShaderProgram::loadShader( gl::VERTEX_SHADER, vertex_path);
        let geometry_shader = geometry_path.map(|path| ShaderProgram::loadShader(gl::GEOMETRY_SHADER, path));
        let fragment_shader = ShaderProgram::loadShader(gl::FRAGMENT_SHADER, fragment_path);

        let shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            if let Some(gs) = geometry_shader {
                gl::AttachShader(shader_program, gs);
            }
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            // Check for linking errors
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetProgramInfoLog(shader_program, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Program linking failed: {}", error.to_string_lossy());
            }
        }

        ShaderProgram {
            shader_program,
            vertex_shader,
            geometry_shader,
            fragment_shader,
        }
    }

    fn loadShader(shader_type : GLenum, file_name: &str) -> GLuint {
        let shader_source = read_file(file_name);
        let shader : GLuint;

        unsafe {
            shader = gl::CreateShader(shader_type);
            let c_str = CString::new(shader_source).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr() , ptr::null() );
            gl::CompileShader(shader);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Shader compilation failed ({}): {}", file_name, error.to_string_lossy());
            }
        }
        shader
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> GLint {
        let c_name = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.shader_program, c_name.as_ptr()) }
    }

    pub fn get_attrib_location(&self, name: &str) -> GLint {
        let c_name = CString::new(name).unwrap();
        unsafe { gl::GetAttribLocation(self.shader_program, c_name.as_ptr()) }
    }


}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DetachShader(self.shader_program, self.vertex_shader);
            gl::DeleteShader(self.vertex_shader);
            if let Some(gs) = self.geometry_shader {
                gl::DetachShader(self.shader_program, gs);
                gl::DeleteShader(gs);
            }
            gl::DetachShader(self.shader_program, self.fragment_shader);
            gl::DeleteShader(self.fragment_shader);
            gl::DeleteProgram(self.shader_program);
        }
    }
}

fn read_file(file_path: &str) -> String {
    let bytes = read(Path::new(file_path)).expect(&format!("Failed to read file: {}", file_path));
    String::from_utf8_lossy(&bytes).to_string()
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Allocate buffer of correct size
    let buffer: Vec<u8> = vec![b' '; len];
    // Convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
