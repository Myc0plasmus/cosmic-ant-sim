use std::ptr;

use gl::types::*;
use gltf::buffer::Data;
use gltf::Document;
use nalgebra_glm as glm;
use crate::utils::constants::*;
use crate::utils::vec_utils::*;
use super::model::*;

use std::fs::File;
use std::io::Read;

struct GpuPrimitive {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    index_count: i32,
}
pub struct Shuttlebug {
    pub model_params: ModelParams,
    primitives: Vec<GpuPrimitive>
}

impl Model for Shuttlebug {
    fn read_model_params(&self) -> &ModelParams {
        &self.model_params
    }

    fn get_model_params(&mut self) -> &mut ModelParams {
        &mut self.model_params
    }

    fn draw_solid(&mut self, _smooth: bool) {

        for primitive in &self.primitives {
            unsafe {
                gl::BindVertexArray(primitive.vao);

                gl::BindBuffer(gl::ARRAY_BUFFER, primitive.vbo);

                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, primitive.ebo);

                gl::DrawElements(gl::TRIANGLES, primitive.index_count, gl::UNSIGNED_INT, ptr::null());
            }
            break;
        }
        



    }
}

impl Shuttlebug {
    pub fn new() -> Self {


        let mut shuttlebug = Shuttlebug {
            model_params: ModelParams {
                vertex_count: 0,
                vertices: std::ptr::null_mut(),
                normals: std::ptr::null_mut(),
                vertex_normals: std::ptr::null_mut(),
                tex_coords: std::ptr::null_mut(),
            },
            primitives: Vec::new(),
        };

        let mut file = File::open("assets/models/shuttlebug2.glb").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let (gltf, buffers, _) = gltf::import_slice(&buffer).unwrap();


        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let mut vao = 0;
                let mut vbo = 0;
                let mut ebo = 0;
                let mut index_count = 0;
                let reader = primitive.reader(|b| Some(&buffers[b.index()]));

                let positions: Vec<f32> = reader.read_positions().unwrap().flat_map(|p| p).collect();
                let normals: Vec<f32> = reader.read_normals().unwrap().flat_map(|p| p).collect();
                let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
        

                index_count = indices.len() as i32;

                // println!("index count: {}",index_count);
                // println!("posistions: {}",positions.len());


                let mut vertex_data = Vec::new();
                for i in 0..positions.len() / 3 {
                    println!("Vec: {} {} {}", positions[3*i], positions[3*i+1], positions[3*i+2]);
                    vertex_data.extend_from_slice(&positions[i * 3..i * 3 + 3]);
                    vertex_data.push(1.0);
                    vertex_data.extend_from_slice(&normals[i * 3..i * 3 + 3]);
                    vertex_data.push(1.0);
                }

                unsafe {
                    gl::GenVertexArrays(1, &mut vao);
                    gl::GenBuffers(1, &mut vbo);
                    gl::GenBuffers(1, &mut ebo);

                    gl::BindVertexArray(vao);

                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (vertex_data.len() * std::mem::size_of::<f32>()) as isize,
                        vertex_data.as_ptr() as *const _,
                        gl::STATIC_DRAW,
                    );

                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                    gl::BufferData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        (indices.len() * std::mem::size_of::<u32>()) as isize,
                        indices.as_ptr() as *const _,
                        gl::STATIC_DRAW,
                    );

                    let stride = 8 * std::mem::size_of::<f32>() as GLsizei;
                    gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride, ptr::null());
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, (4 * std::mem::size_of::<f32>()) as *const _);
                    gl::EnableVertexAttribArray(1);

                    // gl::BindVertexArray(vao);
                    // println!("index count: {}", index_count);
                    // gl::DrawArrays(gl::TRIANGLES,0,(positions.len() / 3) as _);
                }

                shuttlebug.primitives.push(GpuPrimitive{
                    vao,
                    vbo,
                    ebo,
                    index_count
                });
            }
        }

        shuttlebug
    }
}

fn load_glb(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Current dir: {:?}", std::env::current_dir()?);
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (gltf, buffers, _images) = gltf::import_slice(&buffer)?;

    println!("Loaded {} meshes", gltf.meshes().count());
    for mesh in gltf.meshes() {
        println!("Mesh: {}", mesh.name().unwrap_or("Unnamed"));
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            if let Some(positions) = reader.read_positions() {
                for position in positions {
                    println!("Vertex position: {:?}", position);
                }
            }

            if let Some(tex_coords) = reader.read_tex_coords(0) {
                for tex in tex_coords.into_f32() {
                    println!("Texcoord: {:?}", tex);
                }
            }

            if let Some(indices) = reader.read_indices() {
                for index in indices.into_u32() {
                    println!("Index: {:?}", index);
                }
            }
        }
    }

    Ok(())
}
