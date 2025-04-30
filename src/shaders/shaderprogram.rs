use gl::types::GLuint;

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
}
