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
}

impl Model for Sphere {
    fn read_model_params(&self) -> &ModelParams {
        &self.model_params
    }

    fn get_model_params(&self) -> &mut ModelParams{
        &mut self.model_params
    }

    fn draw_solid(&self, smooth: bool) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribPointer(0,4,GLfloat,false,0,&self.model_params.vertices);
            if(smooth) {
                gl::VertexAttribPointer(1,4,GLfloat,false,0,&self.model_params.normals);
            } else {
                gl::VertexAttribPointer(1,4,GLfloat,false,0,&self.model_params.vertex_normals);
            }
            gl::VertexAttribPointer(2,4,GLfloat,false,0,&self.model_params.tex_coords);

            gl::DrawArrays(gl::TRIANGLES,0,self.model_params.vertex_count);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);


        }
    }
}

impl Sphere {
    pub fn new(
        r: Option<f32>,
        main_divs: Option<f32>,
        tube_divs: Option<f32>
    ){
        let r = r.unwrap_or(1.0);
        let main_divs = main_divs.unwrap_or(12.0);
        let tube_divs = tube_divs.unwrap_or(12.0);
        self.build_sphere(r,main_divs,tube_divs);
    }

    fn d2r(&self, deg: f32) -> f32 {
        PI*deg/180.0
    }

     
    fn generate_sphere_point(
        &self,
        r: f32,
        mut alpha: f32,
        mut beta: f32
    ) -> glm::Vec4 {
        alpha = self.d2r(alpha);
        beta = self.d2r(beta);
        glm::vec4(r*alpha.cos()*beta.cos(), r*alpha.cos()*beta.sin(), r*alpha.sin()*beta.sin(), 1.0)
    }

    fn compute_vertex_normal(
        &self,
        mut alpha: f32,
        mut beta: f32
    ) -> glm::Vec4 {
        alpha = self.d2r(alpha);
        beta = self.d2r(beta);
        glm::vec4(alpha.cos()*beta.cos(), alpha.cos()*beta.sin(), beta.sin(), 1.0)
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

        vertex_normals.push(self.generate_sphere_point(r,alpha,beta));
        vertex_normals.push(self.generate_sphere_point(r,alpha+step_alpha,beta));
        vertex_normals.push(self.generate_sphere_point(r,alpha+step_alpha,beta+step_beta));
        vertex_normals.push(self.generate_sphere_point(r,alpha,beta+step_beta));
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

        let mult_alpha: f32 = 180.0/tube_divs;
        let mult_beta: f32 = 180.0/main_divs;

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

                self.internal_vertex_normals.push(face[0]);
                self.internal_vertex_normals.push(face[1]);
                self.internal_vertex_normals.push(face[2]);

                self.internal_vertex_normals.push(face[0]);
                self.internal_vertex_normals.push(face[2]);
                self.internal_vertex_normals.push(face[3]);

                for i in (0..6) {
                    self.internal_face_normals.push(normal);
                }

            }
        }
        self.model_params.vertices = self.internal_vertices.as_ptr() as *mut f32;
        self.model_params.normals = self.internal_face_normals.as_ptr() as *mut f32;
        self.model_params.vertex_normals = self.internal_vertex_normals.as_ptr() as *mut f32;
        self.model_params.tex_coords = self.model_params.vertex_normals;
        self.model_params.vertex_count = self.internal_vertices.len() as i32;
        
    }

}



