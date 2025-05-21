extern crate glfw;
extern crate gl;
extern crate nalgebra_glm;

use std::ffi::CString;
use std::ptr;

use glfw::{Action, Context, Glfw, Key};
use gl::{types::*, ClearColor};

mod shader;
use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use shader::shaderprogram::ShaderProgram;
mod models;
use models::sphere::Sphere;
use models::cube::Cube;
use models::shuttlebug::Shuttlebug;
use models::model::*;
mod utils;
use utils::constants::*;
use nalgebra_glm as glm;


fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    // Create a windowed mode window and its OpenGL context
    let name = "C.A.S";
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, events) = glfw.create_window(1900, 1100, name, glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

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
    let spSimple = ShaderProgram::new(
        "assets/shaders/v_simple.glsl", 
        None,
        "assets/shaders/f_simple.glsl",
    );
    // let spColored = ShaderProgram::new(
    //     "assets/shaders/v_colored.glsl", 
    //     None,
    //     "assets/shaders/f_colored.glsl",
    // );
    // let spTextured = ShaderProgram::new(
    //     "assets/shaders/v_textured.glsl", 
    //     None,
    //     "assets/shaders/f_textured.glsl",
    // );
    // let spLambertTextured = ShaderProgram::new(
    //     "assets/shaders/v_lamberttextured.glsl", 
    //     None,
    //     "assets/shaders/f_lamberttextured.glsl",
    // );
    let r:Option<f32> = Some(1.0);
    let mainDivs:Option<f32> = Some(12.0);
    let tubeDivs:Option<f32> = Some(12.0);

    let mySphere : Sphere = Sphere::new(r, mainDivs, tubeDivs);
    let myCube : Cube = Cube::new();
    let myShuttlebug : Shuttlebug = Shuttlebug::new();


    // Shaders
    // let vertex_shader_src = CString::new(
    //     r#"#version 330 core
    //     layout(location = 0) in vec3 aPos;
    //     uniform mat4 mvp;
    //     void main() {
    //         gl_Position = mvp * vec4(aPos, 1.0);
    //     }"#,
    // )
    // .unwrap();
    //
    // let fragment_shader_src = CString::new(
    //     r#"#version 330 core
    //     out vec4 FragColor;
    //     void main() {
    //         FragColor = vec4(0.2, 0.7, 1.0, 1.0); // Constant color
    //     }"#,
    // )
    // .unwrap();
    //
    // let shader_program = unsafe {
    //     let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    //     gl::ShaderSource(vertex_shader, 1, &vertex_shader_src.as_ptr(), ptr::null());
    //     gl::CompileShader(vertex_shader);
    //
    //     let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    //     gl::ShaderSource(fragment_shader, 1, &fragment_shader_src.as_ptr(), ptr::null());
    //     gl::CompileShader(fragment_shader);
    //
    //     let program = gl::CreateProgram();
    //     gl::AttachShader(program, vertex_shader);
    //     gl::AttachShader(program, fragment_shader);
    //     gl::LinkProgram(program);
    //
    //     gl::DeleteShader(vertex_shader);
    //     gl::DeleteShader(fragment_shader);
    //     program
    // };

    let mut fov: f32 = glm::radians(&glm::vec1(100.0)).x;
    let aspect = 1900.0 / 1100.0;
    let mut P: glm::Mat4 = glm::perspective(aspect,fov,1.0,50.0);
    let mut eye = glm::vec3(0.0 ,0.0, -5.0);
    let mut center = glm::vec3(0.0, 0.0, 0.0);
    let mut up = glm::vec3(0.0, 1.0, 0.0);
    let mut V: glm::Mat4 = glm::look_at(&eye, &center, &up);
    // V = glm::rotate(&V, 0.5*PI, &glm::vec3(0.0,1.0,0.0));
    // let mut M: glm::Mat4 = glm::Mat4::from_element(1.0);

    let mut M = glm::identity();
    // let V = Matrix4::look_at_rh(&Point3::new(1.5, 1.5, 2.0), &Point3::origin(), &Vector3::y());
    // let Pp = Perspective3::new(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);
    // let P = Pp.as_matrix();
    // let mvp = proj.as_matrix() * view * model;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Viewport(0, 0, 1900, 1100);
    }


    // Loop until the user closes the window
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            // gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            spLambert.use_program();
            // spSimple.use_program();

            let axis = glm::vec3(1.0, 1.0, 0.0); // Y axis
            M = glm::rotate(&M, PI+0.1, &axis);
            gl::UniformMatrix4fv(spLambert.get_uniform_location("P"),1,gl::FALSE,P.as_ptr());
            gl::UniformMatrix4fv(spLambert.get_uniform_location("V"),1,gl::FALSE,V.as_ptr());
            gl::UniformMatrix4fv(spLambert.get_uniform_location("M"),1,gl::FALSE,M.as_ptr());
            // gl::UniformMatrix4fv(spConstant.get_uniform_location("M"),1,gl::FALSE,M.as_ptr());
            gl::Uniform4f(spLambert.get_uniform_location("color") as GLint,0.0,1.0,1.0,1.0); 


        
        }
        

        // myCube.draw_solid(true);
        mySphere.draw_solid(true);
        myShuttlebug.draw_solid(true);
        // mySphere.draw_wire(Some(true));

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
