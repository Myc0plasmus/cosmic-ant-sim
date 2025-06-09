use std::ptr;

use gl::types::*;
use nalgebra_glm::round;
use nalgebra_glm as glm;
use crate::utils::constants::*;
use crate::utils::vec_utils::*;
use super::model::*;


pub struct Sphere {
    pub model_params: ModelParams,
    internal_vertices: Vec<glm::Vec4>, 
    internal_face_normals: Vec<glm::Vec4>, 
    internal_vertex_normals: Vec<glm::Vec4>, 
    vao: GLuint,
    vbo: GLuint

}

fn intertwine_vectors(model_params: &ModelParams, vertices: & Vec<glm::Vec4>, normals: &Vec<glm::Vec4>, result: &mut Vec<f32>)  {

    for i in 0..model_params.vertex_count {
        // Intertwining vertex and normal data into the result array
        result.push(vertices[i as usize].x);       // Vertex x
        result.push(vertices[i as usize].y);       // Vertex y
        result.push(vertices[i as usize].z);       // Vertex z
        result.push(vertices[i as usize].w);       // Vertex w

        result.push(normals[i as usize].x);       // Vertex x
        result.push(normals[i as usize].y);       // Vertex y
        result.push(normals[i as usize].z);       // Vertex z
        result.push(normals[i as usize].w);       // Vertex w
    }

}

impl Model for Sphere {
    fn read_model_params(&self) -> &ModelParams {
        &self.model_params
    }

    fn get_model_params(&mut self) -> &mut ModelParams{
        &mut self.model_params
    }

    fn draw_solid(&mut self, smooth: bool) {
   
        unsafe {

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::DrawArrays(gl::TRIANGLES,0,self.model_params.vertex_count);

        }
         


    }
}

impl Sphere {
    pub fn new(
        r: Option<f32>,
        main_divs: Option<f32>,
        tube_divs: Option<f32>
    ) -> Self{
        let r = r.unwrap_or(1.0);
        let main_divs = main_divs.unwrap_or(12.0);
        let tube_divs = tube_divs.unwrap_or(12.0);
        
                // Create an empty Sphere to populate
        let mut sphere = Sphere {
            model_params: ModelParams {
                vertex_count: 0,
                vertices: std::ptr::null_mut(),
                // flat_vertices: Vec::new(),
                normals: std::ptr::null_mut(),
                vertex_normals: std::ptr::null_mut(),
                tex_coords: std::ptr::null_mut(),
                // colors: std::ptr::null_mut(),
                // vao: 0,
                // vbo_positions: 0,
                // vbo_normals: 0,
            },
            internal_vertices: Vec::new(),
            internal_face_normals: Vec::new(),
            internal_vertex_normals: Vec::new(),
            vao: 0,
            vbo: 0
        };

        sphere.build_sphere(r, main_divs, tube_divs);

        let mut result_vector: Vec<f32> = Vec::new();
        intertwine_vectors(&sphere.model_params,&sphere.internal_vertices, &sphere.internal_vertex_normals, &mut result_vector);

        unsafe{
            gl::GenVertexArrays(1, &mut sphere.vao);
            gl::GenBuffers(1, &mut sphere.vbo);

            gl::BindVertexArray(sphere.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, sphere.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                ((result_vector.len() as usize) * std::mem::size_of::<f32>()) as _,
                result_vector.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, (8 * std::mem::size_of::<f32>()) as _, ptr::null());
            gl::EnableVertexAttribArray(0);

            // Normal attribute
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, (8 * std::mem::size_of::<f32>()) as _, (4 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);
        }

        sphere

    }

    fn d2r(&self, deg: f32) -> f32 {
        PI*deg/180.0
    }

     
    fn generate_sphere_point(
        &self,
        r: f32,
        input_alpha: f32,
        input_beta: f32
    ) -> glm::Vec4 {
        let alpha = self.d2r(input_alpha);
        let beta = self.d2r(input_beta);
        glm::vec4(r*alpha.cos()*beta.cos(), r*alpha.cos()*beta.sin(), r*alpha.sin(), 1.0)
    }

    fn compute_vertex_normal(
        &self,
        input_alpha: f32,
        input_beta: f32
    ) -> glm::Vec4 {
        let alpha = self.d2r(input_alpha);
        let beta = self.d2r(input_beta);
        glm::vec4(alpha.cos()*beta.cos(), alpha.cos()*beta.sin(), alpha.sin(), 0.0)
    }

    fn compute_face_normal(&self, face: &Vec<glm::Vec4>) -> glm::Vec4 {
        let a: glm::Vec3 = vec4_to_vec3(face[1]-face[0]);
        let b: glm::Vec3 = vec4_to_vec3(face[2]-face[0]);

        let cross = vec3_to_vec4(glm::cross(&b,&a));

        glm::normalize(&cross)
    }
   
    fn generate_sphere_face(
        &self,
        vertices: &mut Vec<glm::Vec4>,
        vertex_normals: &mut Vec<glm::Vec4>,
        face_normal: &mut glm::Vec4,
        r: f32,
        alpha: f32,
        beta: f32,
        step_alpha: f32,
        step_beta: f32,
    ) {
        vertices.clear();
        vertex_normals.clear();

        vertices.push(self.generate_sphere_point(r,alpha,beta));
        vertices.push(self.generate_sphere_point(r,alpha+step_alpha,beta));
        vertices.push(self.generate_sphere_point(r,alpha+step_alpha,beta+step_beta));
        vertices.push(self.generate_sphere_point(r,alpha,beta+step_beta));

        *face_normal = self.compute_face_normal(&vertices);

        vertex_normals.push(self.compute_vertex_normal(alpha,beta));
        vertex_normals.push(self.compute_vertex_normal(alpha+step_alpha,beta));
        vertex_normals.push(self.compute_vertex_normal(alpha+step_alpha,beta+step_beta));
        vertex_normals.push(self.compute_vertex_normal(alpha,beta+step_beta));
    }

    fn build_sphere(
        &mut self,
        r: f32,
        tube_divs: f32,
        main_divs: f32
    ) {
        let mut face: Vec<glm::Vec4> = Vec::new();
        let mut face_vertex_normals: Vec<glm::Vec4> = Vec::new();
        let mut normal: glm::Vec4 = glm::vec4(0.0,0.0,0.0,0.0);

        self.internal_vertices.clear();
        self.internal_face_normals.clear();
        self.internal_vertex_normals.clear();

        let mult_alpha: f32 = 360.0/tube_divs;
        let mult_beta: f32 = 360.0/main_divs;

        for alpha_it in (0..tube_divs.round() as i32) {
            for beta_it in (0..main_divs.round() as i32) {
                let alpha = alpha_it as f32;
                let beta = beta_it as f32;
                self.generate_sphere_face(&mut face,&mut face_vertex_normals,&mut normal,r,alpha*mult_alpha-90.0,beta*mult_beta, mult_alpha,mult_beta);

                self.internal_vertices.push(face[0]);
                self.internal_vertices.push(face[1]);
                self.internal_vertices.push(face[2]);

                self.internal_vertices.push(face[0]);
                self.internal_vertices.push(face[2]);
                self.internal_vertices.push(face[3]);

                self.internal_vertex_normals.push(face_vertex_normals[0]);
                self.internal_vertex_normals.push(face_vertex_normals[1]);
                self.internal_vertex_normals.push(face_vertex_normals[2]);

                self.internal_vertex_normals.push(face_vertex_normals[0]);
                self.internal_vertex_normals.push(face_vertex_normals[2]);
                self.internal_vertex_normals.push(face_vertex_normals[3]);

                for i in (0..6) {
                    self.internal_face_normals.push(normal);
                }

            }
        }

        // self.model_params.flat_vertices = flatten_vec4(&self.internal_vertices);
        self.model_params.vertices = self.internal_vertices.as_ptr() as *const _;
        self.model_params.normals = self.internal_face_normals.as_ptr() as *const _;
        self.model_params.vertex_normals = self.internal_vertex_normals.as_ptr() as *const _;
        self.model_params.tex_coords = self.model_params.vertex_normals;
        self.model_params.vertex_count = self.internal_vertices.len() as i32;
        // println!("flatten_vertices:");
        // for (i, v) in self.model_params.flat_vertices.iter().enumerate() {
        //     println!("param: {} with value: {}",i%4, v);
        // }
        // println!("vertices:");
        // for (i, v) in self.internal_vertices.iter().enumerate() {
        //     println!("Vertex {}: ({:.3}, {:.3}, {:.3}, {:.3})", i, v.x, v.y, v.z, v.w);
        // }
        // println!("normals:");
        // for (i, n) in self.internal_vertex_normals.iter().enumerate() {
        //     println!("Normal {}: ({:.3}, {:.3}, {:.3}, {:.3})", i, n.x, n.y, n.z, n.w);
        // }
        //
        // println!("VertexCount: {}",self.model_params.vertex_count);
    }

}

fn flatten_vec4(vecs: &Vec<glm::Vec4>) -> Vec<f32> {
    vecs.iter()
        .flat_map(|v| vec![v.x, v.y, v.z, v.w])
        .collect()
}



