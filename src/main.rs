extern crate glfw;
extern crate gl;
extern crate nalgebra_glm;

use std::ptr;

use glfw::{Action, Context, Glfw, Key};
use gl::{types::*, ClearColor};

mod shader;
use shader::shaderprogram::ShaderProgram;
mod models;
use models::sphere::Sphere;
mod utils;


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

    let spLambert = ShaderProgram::new(
        "assets/shaders/v_lambert.glsl", 
        None,
        "assets/shaders/f_lambert.glsl",
    );
    let spConstant = ShaderProgram::new(
        "assets/shaders/v_constant.glsl", 
        None,
        "assets/shaders/f_constant.glsl",
    );
    let spColored = ShaderProgram::new(
        "assets/shaders/v_colored.glsl", 
        None,
        "assets/shaders/f_colored.glsl",
    );
    let spTextured = ShaderProgram::new(
        "assets/shaders/v_textured.glsl", 
        None,
        "assets/shaders/f_textured.glsl",
    );
    let spLambertTextured = ShaderProgram::new(
        "assets/shaders/v_lamberttextured.glsl", 
        None,
        "assets/shaders/f_lamberttextured.glsl",
    );

    let mySphere : Sphere = Sphere::new(None, None, None);


    // Loop until the user closes the window
    while !window.should_close() {
        spLambert.use_program();
        mySphere.draw_solid();

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
