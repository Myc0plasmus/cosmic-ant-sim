extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Glfw, Key};
use gl::{types::*, ClearColor};


type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] =
  [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;

  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;


fn main() {
   use glfw::fail_on_errors;
let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a windowed mode window and its OpenGL context
    let name = "C.A.S";
    let (mut window, events) = glfw.create_window(1900, 1100, name,
glfw::WindowMode::Windowed)         .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s));

    unsafe {
        ClearColor(0.2, 0.3, 0.3, 1.0);
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
          gl::ARRAY_BUFFER,
          size_of_val(&VERTICES) as isize,
          VERTICES.as_ptr().cast(),
          gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
          0,
          3,
          gl::FLOAT,
          gl::FALSE,
          size_of::<Vertex>().try_into().unwrap(),
          0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        gl::ShaderSource(
          vertex_shader,
          1,
          &(VERT_SHADER.as_bytes().as_ptr().cast()),
          &(VERT_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(vertex_shader);
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
          let mut v: Vec<u8> = Vec::with_capacity(1024);
          let mut log_len = 0_i32;
          gl::GetShaderInfoLog(
            vertex_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
          );
          v.set_len(log_len.try_into().unwrap());
          panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        gl::ShaderSource(
          fragment_shader,
          1,
          &(FRAG_SHADER.as_bytes().as_ptr().cast()),
          &(FRAG_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(fragment_shader);
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
          let mut v: Vec<u8> = Vec::with_capacity(1024);
          let mut log_len = 0_i32;
          gl::GetShaderInfoLog(
            fragment_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
          );
          v.set_len(log_len.try_into().unwrap());
          panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let shader_program = gl::CreateProgram();
        assert_ne!(shader_program, 0);
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
          let mut v: Vec<u8> = Vec::with_capacity(1024);
          let mut log_len = 0_i32;
          gl::GetProgramInfoLog(
            shader_program,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
          );
          v.set_len(log_len.try_into().unwrap());
          panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
        gl::DeleteShader(fragment_shader);

        gl::UseProgram(shader_program);
    }

    // Loop until the user closes the window
    while !window.should_close() {
        unsafe {
          gl::Clear(gl::COLOR_BUFFER_BIT);
          gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}
