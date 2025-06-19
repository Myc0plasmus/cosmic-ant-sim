use glutin::prelude::GlDisplay;
use rand::{rng, thread_rng, Rng};
use gl::types::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::time::Instant;

use image::GenericImageView;
use nalgebra_glm as glm;
use crate::shader::shaderprogram::ShaderProgram;
use crate::models::{cube::Cube, model::*, shuttlebug::Shuttlebug, sphere::Sphere};

pub struct Renderer {
    M: glm::Mat4,
    V: glm::Mat4,
    P: glm::Mat4,
    shader: ShaderProgram,
    lambert: ShaderProgram,
    models: HashMap<String, Box<dyn Model>>,
    pub zoom: f32,
    dirtTexture: GLuint,
    random_pos_vector: Vec<glm::Vec3>, 
    pub speed: f32

}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

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

        let mut models = HashMap::new();
        
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
        let spLambertTextured = ShaderProgram::new(
            "assets/shaders/v_lamberttextured.glsl", 
            None,
            "assets/shaders/f_lamberttextured.glsl",
        );
        let r:Option<f32> = Some(0.3);
        let mainDivs:Option<f32> = Some(36.0);
        let tubeDivs:Option<f32> = Some(36.0);

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

        // M = glm::scale(&M, &glm::vec3(5.0,5.0,5.0));

        let mut mySphere = Box::new(Sphere::new(r, mainDivs, tubeDivs));
        let mut myCube = Box::new(Cube::new());
        let mut myShuttlebug  = Box::new(Shuttlebug::new());
        let mut renderer = Renderer {M,V,P,shader: spSimple, lambert: spLambertTextured, models, zoom: 5.0, dirtTexture: 0, speed: 0.0, random_pos_vector: Vec::new()};
        renderer.generateRandomPos();
        renderer.dirtTexture = renderer.load_texture("assets/textures/dirtTexture.png");
        
           
        renderer.addModel("cube", myCube);
        renderer.addModel("sphere",mySphere);
        renderer.addModel("ant",myShuttlebug);
        renderer
    }

    pub fn changeCameraZoom(&mut self) {
        let mut eye = glm::vec3(0.0 ,0.0, -self.zoom);
        let mut center = glm::vec3(0.0, 0.0, 0.0);
        let mut up = glm::vec3(0.0, 1.0, 0.0);
        self.V = glm::look_at(&eye, &center, &up);
        
    }

    
    fn load_texture<P: AsRef<Path>>(&self, path: P) -> GLuint {
        // Load the image using the image crate
        let img = image::open(path).expect("Failed to load texture");
        let img = img.flipv(); // Flip vertically to match OpenGL convention
        let (width, height) = img.dimensions();
        let data = img.to_rgba8();

        // Generate and bind a texture
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Upload image data to GPU
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLint,
                height as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

            // Unbind and return
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        texture_id
    }

        

    pub fn addModel(&mut self, name: impl Into<String>, mut model: Box<dyn Model>) {
        self.models.insert(name.into(), model);
    }

    pub fn generateRandomPos(&mut self) {
        self.random_pos_vector.clear();
        for i in 0..10 {
            self.random_pos_vector.push(self.spherical_rand(3.0));
        }
    }

    fn spherical_rand(&self,radius: f32) -> glm::Vec3 {
        let mut rng = rng();

        let u: f32 = rng.random_range(0.0..1.0);
        let v: f32 = rng.random_range(0.0..1.0);

        let theta = 2.0 * std::f32::consts::PI * u;
        let phi = (1.0 - 2.0 * v).acos();

        let x = radius * phi.sin() * theta.cos();
        let y = radius * phi.sin() * theta.sin();
        let z = radius * phi.cos();

        glm::vec3(x, y, z)
    }

    pub fn draw(&mut self) {

        // let V = Matrix4::look_at_rh(&Point3::new(1.5, 1.5, 2.0), &Point3::origin(), &Vector3::y());
        // let Pp = Perspective3::new(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);
        // let P = Pp.as_matrix();
        // let mvp = proj.as_matrix() * view * model;



        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Viewport(0, 0, 1900, 1100);
            gl::Disable(gl::CULL_FACE);
            gl::FrontFace(gl::CW);
        }

        let mut angle = 0.0;
        // Loop until the user closes the window
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            // gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.shader.use_program();
            // spSimple.use_program();

            let axis = glm::vec3(1.0, 1.0, 0.0); // Y axis

            self.M = glm::rotate(&self.M, 0.01, &axis);
            angle+=self.speed ;
            self.V = glm::rotate(&self.V, angle, &glm::vec3(0.0,1.0,0.0));
            // self.V = glm::rotate(&self.V, (PI)+0.01, &axis);
            gl::UniformMatrix4fv(self.shader.get_uniform_location("P"),1,gl::FALSE,self.P.as_ptr());
            gl::UniformMatrix4fv(self.shader.get_uniform_location("V"),1,gl::FALSE,self.V.as_ptr());
            gl::UniformMatrix4fv(self.shader.get_uniform_location("M"),1,gl::FALSE,self.M.as_ptr());
            // gl::UniformMatrix4fv(spConstant.get_uniform_location("M"),1,gl::FALSE,M.as_ptr());
            gl::Uniform4f(self.shader.get_uniform_location("color") as GLint,1.0,1.0,1.0,1.0); 

            // gl::Uniform1i(self.shader.get_uniform_location("baseColorTexture"), 0);


        
        }
        

        // myCube.draw_solid(true);
        // self.models["ant"].draw_wire(Some(true));
        // self.models.get_mut("sphere").unwrap().draw_wire(Some(true));
        

        self.models.get_mut("ant").unwrap().draw_solid(false,&self.shader);
        for pos in &self.random_pos_vector {
            let mut randM: glm::Mat4 = glm::identity(); 
            randM = glm::translate(&randM, &pos);

            self.lambert.use_program();
            unsafe {
                gl::UniformMatrix4fv(self.lambert.get_uniform_location("P"),1,gl::FALSE,self.P.as_ptr());
                gl::UniformMatrix4fv(self.lambert.get_uniform_location("V"),1,gl::FALSE,self.V.as_ptr());
                gl::UniformMatrix4fv(self.lambert.get_uniform_location("M"),1,gl::FALSE,randM.as_ptr());
                // gl::UniformMatrix4fv(spConstant.get_uniform_location("M"),1,gl::FALSE,M.as_ptr());
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.dirtTexture);
                gl::Uniform1i(self.lambert.get_uniform_location("tex"),0);

                gl::Uniform4f(self.lambert.get_uniform_location("color") as GLint,1.0,1.0,1.0,1.0); 
            }
            self.models.get_mut("sphere").unwrap().draw_solid(false,&self.shader);

        }
        // unsafe {
        //     let mut ms = glm::identity();
        //     ms = glm::translate(&ms, &glm::vec3(3.0,0.0,0.0));
        //     
        //
        //     gl::Uniform4f(self.shader.get_uniform_location("color") as GLint,1.0,1.0,1.0,1.0); 
        //     gl::UniformMatrix4fv(self.shader.get_uniform_location("M"),1,gl::FALSE,ms.as_ptr());
        //
        // }
        //
        // self.models.get_mut("cube").unwrap().draw_solid(true);

        // myShuttlebug.draw_solid(true);
        // mySphere.draw_wire(Some(true));

        // Swap front and back buffers
        // window.swap_buffers();

        // Poll for and process events

    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}




pub fn get_gl_string(variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl::GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}
