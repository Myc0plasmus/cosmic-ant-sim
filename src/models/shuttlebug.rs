use std::ptr;

use gl::types::*;
use gltf::buffer::Data;
use gltf::Document;
use nalgebra_glm as glm;
use crate::shader::shaderprogram::ShaderProgram;
use crate::utils::constants::*;
use crate::utils::vec_utils::*;
use super::model::*;

use image::GenericImageView;

use std::fs::File;
use std::io::Read;

struct GpuPrimitive {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    base_texture: GLuint,
    emissive_texture: GLuint,
    normal_texture: GLuint,
    index_count: i32,
}
pub struct Shuttlebug{
    pub model_params: ModelParams,
    primitives: Vec<GpuPrimitive>,
}

impl Model for Shuttlebug {
    fn read_model_params(&self) -> &ModelParams {
        &self.model_params
    }

    fn get_model_params(&mut self) -> &mut ModelParams {
        &mut self.model_params
    }

    fn draw_solid(&mut self, _smooth: bool, shader: &ShaderProgram) {

        for primitive in &self.primitives {
            unsafe {
                gl::BindVertexArray(primitive.vao);

                gl::BindBuffer(gl::ARRAY_BUFFER, primitive.vbo);

                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, primitive.ebo);

                // gl::ActiveTexture(gl::TEXTURE0);

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, primitive.base_texture);
                gl::Uniform1i(shader.get_uniform_location("baseColorTexture"), 0);

                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, primitive.emissive_texture);
                gl::Uniform1i(shader.get_uniform_location("emissiveTexture"), 1);

                gl::ActiveTexture(gl::TEXTURE2);
                gl::BindTexture(gl::TEXTURE_2D, primitive.normal_texture);
                gl::Uniform1i( shader.get_uniform_location("normalTexture"), 2);
                gl::DrawElements(gl::TRIANGLES, primitive.index_count, gl::UNSIGNED_INT, ptr::null());
            }
            break;
        }
        



    }
}

impl Shuttlebug {
    fn load_texture(&self, image: &gltf::image::Image, buffers: &[Data]) -> GLuint {
        let image_data = match image.source() {
            gltf::image::Source::View { view, .. } => {
                let start = view.offset();
                let end = start + view.length();
                &buffers[view.buffer().index()][start..end]
            },
            gltf::image::Source::Uri { .. } => {
                panic!("Only embedded GLB textures are supported.");
            }
        };

        let decoded = image::load_from_memory(&image_data).unwrap().flipv();
        let (width, height) = decoded.dimensions();
        let rgba = decoded.to_rgba8();

        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                rgba.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        tex_id
    }

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
                let mut texture = 0;
                let reader = primitive.reader(|b| Some(&buffers[b.index()]));

                let positions: Vec<f32> = reader.read_positions().unwrap().flat_map(|p| p).collect();
                let normals: Vec<f32> = reader.read_normals().unwrap().flat_map(|p| p).collect();
                let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
                let tex_coords: Vec<[f32; 2]> = reader
                    .read_tex_coords(0)
                    .map(|tc| tc.into_f32().collect())
                    .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len() / 3]);
        

                index_count = indices.len() as i32;

                // println!("index count: {}",index_count);
                // println!("posistions: {}",positions.len());


                let mut vertex_data = Vec::new();
                for i in 0..positions.len() / 3 {
                    // println!("Vec: {} {} {}", positions[3*i], positions[3*i+1], positions[3*i+2]);
                    vertex_data.extend_from_slice(&positions[i * 3..i * 3 + 3]);
                    vertex_data.push(1.0);
                    vertex_data.extend_from_slice(&normals[i * 3..i * 3 + 3]);
                    vertex_data.push(1.0);
                    vertex_data.extend_from_slice(&tex_coords[i]);
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

                    let stride = 10 * std::mem::size_of::<f32>() as GLsizei;
                    gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride, ptr::null());
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, (4 * std::mem::size_of::<f32>()) as *const _);
                    gl::EnableVertexAttribArray(1);
                    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (8 * std::mem::size_of::<f32>()) as *const _); // tex coords
gl::EnableVertexAttribArray(2);

                    // gl::BindVertexArray(vao);
                    // println!("index count: {}", index_count);
                    // gl::DrawArrays(gl::TRIANGLES,0,(positions.len() / 3) as _);
                }

                let mut base_texture = 0;
                let mut emissive_texture = 0;
                let mut normal_texture = 0;

                let mat = primitive.material();
                let pbr = mat.pbr_metallic_roughness();

                // Base Color
                if let Some(base_tex_info) = pbr.base_color_texture() {
                    let image = &gltf.images().nth(base_tex_info.texture().source().index()).unwrap();
                    base_texture = shuttlebug.load_texture(image, &buffers);
                }

                // Emissive
                if let Some(emissive_tex_info) = mat.emissive_texture() {
                    let image = &gltf.images().nth(emissive_tex_info.texture().source().index()).unwrap();
                    emissive_texture = shuttlebug.load_texture(image, &buffers);
                }

                // Normal Map
                if let Some(normal_tex_info) = mat.normal_texture() {
                    let image = &gltf.images().nth(normal_tex_info.texture().source().index()).unwrap();
                    normal_texture = shuttlebug.load_texture(image, &buffers);
                }

                shuttlebug.primitives.push(GpuPrimitive{
                    vao,
                    vbo,
                    ebo,
                    base_texture,
                    emissive_texture,
                    normal_texture,
                    index_count
                });
            }
        }

        shuttlebug
    }
}


