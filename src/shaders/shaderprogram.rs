use gl::types::GLuint;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct ShaderProgram {
   shader_program : GLuint,
   vertex_shader: GLuint,
   geometry_shader: GLuint,
   fragment_shader: GLuint,
}

impl ShaderProgram {
    fn read_file_bytes(file_path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn loadShader(shader_type : GLenum, file_name: &str) -> GLuint {
        unsafe {
            let mut shader : GLuint = gl::CreateShader(shader_type);
            let shader_source : GLchar = read_file_bytes(file_name);
            gl::ShaderSource(shader, 1, shader_source, null() );
        }
    }
}
