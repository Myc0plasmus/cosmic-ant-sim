use crate::shader::shaderprogram::ShaderProgram;

use super::model::*;
use gl::types::GLuint;
use nalgebra_glm as glm;
use std::ptr;

pub const CUBE_VERTEX_COUNT: usize = 36;

pub const CUBE_VERTICES: [f32; CUBE_VERTEX_COUNT * 4] = [
				1.0,-1.0,-1.0,1.0,
				-1.0, 1.0,-1.0,1.0,
				-1.0,-1.0,-1.0,1.0,

				1.0,-1.0,-1.0,1.0,
				1.0, 1.0,-1.0,1.0,
				-1.0, 1.0,-1.0,1.0,


				-1.0,-1.0, 1.0,1.0,
				1.0, 1.0, 1.0,1.0,
				1.0,-1.0, 1.0,1.0,

				-1.0,-1.0, 1.0,1.0,
				-1.0, 1.0, 1.0,1.0,
				1.0, 1.0, 1.0,1.0,

				1.0,-1.0, 1.0,1.0,
				1.0, 1.0,-1.0,1.0,
				1.0,-1.0,-1.0,1.0,

				1.0,-1.0, 1.0,1.0,
				1.0, 1.0, 1.0,1.0,
				1.0, 1.0,-1.0,1.0,

				-1.0,-1.0,-1.0,1.0,
				-1.0, 1.0, 1.0,1.0,
				-1.0,-1.0, 1.0,1.0,

				-1.0,-1.0,-1.0,1.0,
				-1.0, 1.0,-1.0,1.0,
				-1.0, 1.0, 1.0,1.0,

				-1.0,-1.0,-1.0,1.0,
				1.0,-1.0, 1.0,1.0,
				1.0,-1.0,-1.0,1.0,

				-1.0,-1.0,-1.0,1.0,
				-1.0,-1.0, 1.0,1.0,
				1.0,-1.0, 1.0,1.0,

				-1.0, 1.0, 1.0,1.0,
				1.0, 1.0,-1.0,1.0,
				1.0, 1.0, 1.0,1.0,

				-1.0, 1.0, 1.0,1.0,
				-1.0, 1.0,-1.0,1.0,
				1.0, 1.0,-1.0,1.0,
];

pub const CUBE_COLORS: [f32; CUBE_VERTEX_COUNT * 4] = [
				1.0,0.0,0.0,1.0,
				1.0,0.0,0.0,1.0,
				1.0,0.0,0.0,1.0,

				1.0,0.0,0.0,1.0,
				1.0,0.0,0.0,1.0,
				1.0,0.0,0.0,1.0,

				0.0,1.0,0.0,1.0,
				0.0,1.0,0.0,1.0,
				0.0,1.0,0.0,1.0,

				0.0,1.0,0.0,1.0,
				0.0,1.0,0.0,1.0,
				0.0,1.0,0.0,1.0,

				0.0,0.0,1.0,1.0,
				0.0,0.0,1.0,1.0,
				0.0,0.0,1.0,1.0,

				0.0,0.0,1.0,1.0,
				0.0,0.0,1.0,1.0,
				0.0,0.0,1.0,1.0,

				1.0,1.0,0.0,1.0,
				1.0,1.0,0.0,1.0,
				1.0,1.0,0.0,1.0,

				1.0,1.0,0.0,1.0,
				1.0,1.0,0.0,1.0,
				1.0,1.0,0.0,1.0,

				0.0,1.0,1.0,1.0,
				0.0,1.0,1.0,1.0,
				0.0,1.0,1.0,1.0,

				0.0,1.0,1.0,1.0,
				0.0,1.0,1.0,1.0,
				0.0,1.0,1.0,1.0,

				1.0,1.0,1.0,1.0,
				1.0,1.0,1.0,1.0,
				1.0,1.0,1.0,1.0,

				1.0,1.0,1.0,1.0,
				1.0,1.0,1.0,1.0,
				1.0,1.0,1.0,1.0,
];

pub const CUBE_NORMALS: [f32; CUBE_VERTEX_COUNT * 4] = [
 				0.0, 0.0,-1.0,0.0,
				0.0, 0.0,-1.0,0.0,
				0.0, 0.0,-1.0,0.0,

				0.0, 0.0,-1.0,0.0,
				0.0, 0.0,-1.0,0.0,
				0.0, 0.0,-1.0,0.0,

				0.0, 0.0, 1.0,0.0,
				0.0, 0.0, 1.0,0.0,
				0.0, 0.0, 1.0,0.0,

				0.0, 0.0, 1.0,0.0,
				0.0, 0.0, 1.0,0.0,
				0.0, 0.0, 1.0,0.0,

				1.0, 0.0, 0.0,0.0,
				1.0, 0.0, 0.0,0.0,
				1.0, 0.0, 0.0,0.0,

				1.0, 0.0, 0.0,0.0,
				1.0, 0.0, 0.0,0.0,
				1.0, 0.0, 0.0,0.0,

				-1.0, 0.0, 0.0,0.0,
				-1.0, 0.0, 0.0,0.0,
				-1.0, 0.0, 0.0,0.0,

				-1.0, 0.0, 0.0,0.0,
				-1.0, 0.0, 0.0,0.0,
				-1.0, 0.0, 0.0,0.0,

				0.0,-1.0, 0.0,0.0,
				0.0,-1.0, 0.0,0.0,
				0.0,-1.0, 0.0,0.0,

				0.0,-1.0, 0.0,0.0,
				0.0,-1.0, 0.0,0.0,
				0.0,-1.0, 0.0,0.0,

				0.0, 1.0, 0.0,0.0,
				0.0, 1.0, 0.0,0.0,
				0.0, 1.0, 0.0,0.0,

				0.0, 1.0, 0.0,0.0,
				0.0, 1.0, 0.0,0.0,
				0.0, 1.0, 0.0,0.0,
];

pub const CUBE_VERTEX_NORMALS: [f32; CUBE_VERTEX_COUNT * 4] = [
    // same layout as vertices
                1.0,-1.0,-1.0,0.0,
				-1.0, 1.0,-1.0,0.0,
				-1.0,-1.0,-1.0,0.0,

				1.0,-1.0,-1.0,0.0,
				1.0, 1.0,-1.0,0.0,
				-1.0, 1.0,-1.0,0.0,


				-1.0,-1.0, 1.0,0.0,
				1.0, 1.0, 1.0,0.0,
				1.0,-1.0, 1.0,0.0,

				-1.0,-1.0, 1.0,0.0,
				-1.0, 1.0, 1.0,0.0,
				1.0, 1.0, 1.0,0.0,

				1.0,-1.0, 1.0,0.0,
				1.0, 1.0,-1.0,0.0,
				1.0,-1.0,-1.0,0.0,

				1.0,-1.0, 1.0,0.0,
				1.0, 1.0, 1.0,0.0,
				1.0, 1.0,-1.0,0.0,

				-1.0,-1.0,-1.0,0.0,
				-1.0, 1.0, 1.0,0.0,
				-1.0,-1.0, 1.0,0.0,

				-1.0,-1.0,-1.0,0.0,
				-1.0, 1.0,-1.0,0.0,
				-1.0, 1.0, 1.0,0.0,

				-1.0,-1.0,-1.0,0.0,
				1.0,-1.0, 1.0,0.0,
				1.0,-1.0,-1.0,0.0,

				-1.0,-1.0,-1.0,0.0,
				-1.0,-1.0, 1.0,0.0,
				1.0,-1.0, 1.0,0.0,

				-1.0, 1.0, 1.0,0.0,
				1.0, 1.0,-1.0,0.0,
				1.0, 1.0, 1.0,0.0,

				-1.0, 1.0, 1.0,0.0,
				-1.0, 1.0,-1.0,0.0,
				1.0, 1.0,-1.0,0.0,


];

pub const CUBE_TEX_COORDS: [f32; CUBE_VERTEX_COUNT*2 ] = [
				1.0,1.0, 0.0,0.0, 0.0,1.0,
				1.0,1.0, 1.0,0.0, 0.0,0.0,

				1.0,1.0, 0.0,0.0, 0.0,1.0,
				1.0,1.0, 1.0,0.0, 0.0,0.0,

				1.0,1.0, 0.0,0.0, 0.0,1.0,
				1.0,1.0, 1.0,0.0, 0.0,0.0,

				1.0,1.0, 0.0,0.0, 0.0,1.0,
				1.0,1.0, 1.0,0.0, 0.0,0.0,

				1.0,1.0, 0.0,0.0, 0.0,1.0,
				1.0,1.0, 1.0,0.0, 0.0,0.0,

				1.0,1.0, 0.0,0.0, 0.0,1.0,
				1.0,1.0, 1.0,0.0, 0.0,0.0,
];

// pub const CUBE_OTHER_VERTICES: [f32; 108] =  [
//         // positions
//         -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,
//          0.5,  0.5, -0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5, // back face
//         -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5,
//          0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5, // front face
//         -0.5,  0.5,  0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5,
//         -0.5, -0.5, -0.5, -0.5, -0.5,  0.5, -0.5,  0.5,  0.5, // left face
//          0.5,  0.5,  0.5,  0.5,  0.5, -0.5,  0.5, -0.5, -0.5,
//          0.5, -0.5, -0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5, // right face
//         -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5, -0.5,  0.5,
//          0.5, -0.5,  0.5, -0.5, -0.5,  0.5, -0.5, -0.5, -0.5, // bottom face
//         -0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,
//          0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5, -0.5  // top face
//     ];

pub struct Cube {
    pub model_params: ModelParams,
    vao: GLuint,
    vbo: GLuint


}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 4],
    normal: [f32; 4],
}

/// Function to intertwine the vertex and normal data into a flat array.
fn intertwine_arrays(vertices: &[f32], normals: &[f32]) -> [f32; CUBE_VERTEX_COUNT * 8] {
    let mut result = [0.0; CUBE_VERTEX_COUNT * 8];

    for i in 0..CUBE_VERTEX_COUNT {
        // Intertwining vertex and normal data into the result array
        result[i * 8] = vertices[i * 4];       // Vertex x
        result[i * 8 + 1] = vertices[i * 4 + 1]; // Vertex y
        result[i * 8 + 2] = vertices[i * 4 + 2]; // Vertex z
        result[i * 8 + 3] = vertices[i * 4 + 3]; // Vertex w

        result[i * 8 + 4] = normals[i * 4];       // Normal nx
        result[i * 8 + 5] = normals[i * 4 + 1];   // Normal ny
        result[i * 8 + 6] = normals[i * 4 + 2];   // Normal nz
        result[i * 8 + 7] = normals[i * 4 + 3];   // Normal nw
    }

    result
}

impl Model for Cube {
    fn read_model_params(&self) -> &ModelParams {
        &self.model_params
    }

    fn get_model_params(&mut self) -> &mut ModelParams{
        &mut self.model_params
    }

    fn draw_solid(&mut self, smooth: bool, shader: &ShaderProgram) {
        unsafe {


            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::DrawArrays(gl::TRIANGLES,0,self.model_params.vertex_count);

        }
    }
}

impl Cube {
   pub fn new() -> Self {
       let mut cube = Cube {
           model_params: ModelParams {
                vertex_count: CUBE_VERTEX_COUNT as i32,
                vertices: CUBE_VERTICES.as_ptr() as *const _,
                normals: CUBE_NORMALS.as_ptr() as *const _,
                vertex_normals: CUBE_VERTEX_NORMALS.as_ptr() as *const _,
                tex_coords: CUBE_TEX_COORDS.as_ptr() as *const _,
                // colors: CUBE_COLORS.as_ptr() as *const _,
                // vao: 0,
                // vbo_positions: 0,
                // vbo_normals: 0,
            },
            vao: 0,
            vbo: 0
       };
       unsafe {
            gl::GenVertexArrays(1, &mut cube.vao);
            gl::GenBuffers(1, &mut cube.vbo);

            gl::BindVertexArray(cube.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, cube.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (CUBE_VERTICES.len() * 2 * std::mem::size_of::<f32>()) as _,
                intertwine_arrays(&CUBE_VERTICES, &CUBE_VERTEX_NORMALS).as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, (8 * std::mem::size_of::<f32>()) as _, ptr::null());
            gl::EnableVertexAttribArray(0);

            // Normal attribute
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, (8 * std::mem::size_of::<f32>()) as _, (4 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);
       }
       cube
   }
}
