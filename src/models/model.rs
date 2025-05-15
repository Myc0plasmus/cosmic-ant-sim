use gl::types::*;

#[derive(Clone)]
pub struct ModelParams {
    pub vertex_count: i32,
    pub flat_vertices: Vec<f32>,
    pub vertices: *const gl::types::GLvoid,
    pub normals: *const gl::types::GLvoid,
    pub vertex_normals: *const gl::types::GLvoid,
    pub tex_coords: *const gl::types::GLvoid,
    pub colors: *const gl::types::GLvoid,
}

pub trait Model{
    fn read_model_params(&self) -> &ModelParams;

    fn get_model_params(&mut self) -> &mut ModelParams;

    fn draw_solid(&self, smooth: bool);

    fn draw_wire(&self, smooth: Option<bool>) {
        let smooth = smooth.unwrap_or(false);
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            self.draw_solid(smooth);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
    }

}
