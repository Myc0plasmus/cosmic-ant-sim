#[derive(Clone)]
pub struct ModelParams {
    pub vertex_count: i32,
    pub vertices: *const f32,
    pub normals: *const f32,
    pub vertex_normals: *const f32,
    pub tex_coords: *const f32,
    pub colors: *const f32
}

pub trait Model{
    fn read_model_params(&self) -> &ModelParams;

    fn get_model_params(&self) -> &mut ModelParams;

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
