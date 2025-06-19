// extern crate gl;
extern crate nalgebra_glm;



use app::app_window::{window_attributes, App};

mod shader;
mod models;
mod utils;
mod app;


use utils::constants::*;


use winit::event_loop::{ControlFlow, EventLoop};
use glutin::config::{Config, ConfigTemplateBuilder, GetGlConfig};
use winit::window::{Window, WindowAttributes, WindowId};
use glutin_winit::{DisplayBuilder, GlWindow};


fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);
    //
    // let mut app = App::new(template, display_builder);;
    // event_loop.run_app(&mut app);
    let template =
    ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let mut app = App::new(template, display_builder);
    event_loop.run_app(&mut app);

    // app.exit_state
}
