// extern crate gl;
extern crate nalgebra_glm;

use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::time::Instant;

use gl::types::*;

mod shader;
mod models;
mod utils;

use nalgebra_glm as glm;
use shader::shaderprogram::ShaderProgram;
use models::{cube::Cube, model::*, shuttlebug::Shuttlebug, sphere::Sphere};
use utils::constants::*;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes, WindowId};

use glutin::config::{Config, ConfigTemplateBuilder, GetGlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

use glutin_winit::{DisplayBuilder, GlWindow};

struct AppState {
    gl_surface: Surface<WindowSurface>,
    // NOTE: Window should be dropped after all resources created using its
    // raw-window-handle.
    window: Window,
}

struct App {
    template: ConfigTemplateBuilder,
    renderer: Option<Renderer>,
    // NOTE: `AppState` carries the `Window`, thus it should be dropped after everything else.
    state: Option<AppState>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_display: GlDisplayCreationState,
    exit_state: Result<(), Box<dyn Error>>,
}

impl App {
    fn new(template: ConfigTemplateBuilder, display_builder: DisplayBuilder) -> Self {
        Self {
            template,
            gl_display: GlDisplayCreationState::Builder(display_builder),
            exit_state: Ok(()),
            gl_context: None,
            state: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, gl_config) = match &self.gl_display {
            // We just created the event loop, so initialize the display, pick the config, and
            // create the context.
            GlDisplayCreationState::Builder(display_builder) => {
                let (window, gl_config) = match display_builder.clone().build(
                    event_loop,
                    self.template.clone(),
                    gl_config_picker,
                ) {
                    Ok((window, gl_config)) => (window.unwrap(), gl_config),
                    Err(err) => {
                        self.exit_state = Err(err);
                        event_loop.exit();
                        return;
                    },
                };

                println!("Picked a config with {} samples", gl_config.num_samples());

                // Mark the display as initialized to not recreate it on resume, since the
                // display is valid until we explicitly destroy it.
                self.gl_display = GlDisplayCreationState::Init;

                // Create gl context.
                self.gl_context =
                    Some(create_gl_context(&window, &gl_config).treat_as_possibly_current());

                (window, gl_config)
            },
            GlDisplayCreationState::Init => {
                println!("Recreating window in `resumed`");
                // Pick the config which we already use for the context.
                let gl_config = self.gl_context.as_ref().unwrap().config();
                match glutin_winit::finalize_window(event_loop, window_attributes(), &gl_config) {
                    Ok(window) => (window, gl_config),
                    Err(err) => {
                        self.exit_state = Err(err.into());
                        event_loop.exit();
                        return;
                    },
                }
            },
        };

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");
        let gl_surface =
            unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

        // The context needs to be current for the Renderer to set up shaders and
        // buffers. It also performs function loading, which needs a current context on
        // WGL.
        let gl_context = self.gl_context.as_ref().unwrap();
        gl_context.make_current(&gl_surface).unwrap();

        self.renderer.get_or_insert_with(|| Renderer::new(&gl_config.display()));

        // Try setting vsync.
        if let Err(res) = gl_surface
            .set_swap_interval(gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("Error setting vsync: {res:?}");
        }

        assert!(self.state.replace(AppState { gl_surface, window }).is_none());
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // This event is only raised on Android, where the backing NativeWindow for a GL
        // Surface can appear and disappear at any moment.
        println!("Android window removed");

        // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
        // the window back to the system.
        self.state = None;

        // Make context not current.
        self.gl_context = Some(
            self.gl_context.take().unwrap().make_not_current().unwrap().treat_as_possibly_current(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
                // Some platforms like EGL require resizing GL surface to update the size
                // Notable platforms here are Wayland and macOS, other don't require it
                // and the function is no-op, but it's wise to resize it for portability
                // reasons.
                if let Some(AppState { gl_surface, window: _ }) = self.state.as_ref() {
                    let gl_context = self.gl_context.as_ref().unwrap();
                    gl_surface.resize(
                        gl_context,
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );

                    let renderer = self.renderer.as_ref().unwrap();
                    renderer.resize(size.width as i32, size.height as i32);
                }
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: Key::Named(NamedKey::ArrowUp), state: ElementState::Pressed, ..},
                ..
            } => {

                let mut renderer = self.renderer.as_mut().unwrap();
                renderer.zoom += 0.1;
                renderer.changeCameraZoom();

            },
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: Key::Named(NamedKey::ArrowDown), state: ElementState::Pressed, ..},
                ..
            } => {

                let mut renderer = self.renderer.as_mut().unwrap();
                renderer.zoom -= 0.1;
                renderer.changeCameraZoom();

            },
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
                ..
            } => event_loop.exit(),
            _ => (),

        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // NOTE: The handling below is only needed due to nvidia on Wayland to not crash
        // on exit due to nvidia driver touching the Wayland display from on
        // `exit` hook.
        let _gl_display = self.gl_context.take().unwrap().display();

        // Clear the window.
        self.state = None;
        #[cfg(egl_backend)]
        #[allow(irrefutable_let_patterns)]
        if let glutin::display::Display::Egl(display) = _gl_display {
            unsafe {
                display.terminate();
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(AppState { gl_surface, window }) = self.state.as_ref() {
            let gl_context = self.gl_context.as_ref().unwrap();
            let renderer = self.renderer.as_mut().unwrap();
            renderer.draw();
            window.request_redraw();

            gl_surface.swap_buffers(gl_context).unwrap();
        }
    }
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display.create_context(gl_config, &context_attributes).unwrap_or_else(|_| {
            gl_display.create_context(gl_config, &fallback_context_attributes).unwrap_or_else(
                |_| {
                    gl_display
                        .create_context(gl_config, &legacy_context_attributes)
                        .expect("failed to create context")
                },
            )
        })
    }
}

fn window_attributes() -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title("Cosmic Ant Simulator (press Escape to exit)")
}

enum GlDisplayCreationState {
    /// The display was not build yet.
    Builder(DisplayBuilder),
    /// The display was already created for the application.
    Init,
}

// Find the config with the maximum number of samples, so our triangle will be
// smooth.
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

pub struct Renderer {
    M: glm::Mat4,
    V: glm::Mat4,
    P: glm::Mat4,
    shader: ShaderProgram,
    models: HashMap<String, Box<dyn Model>>,
    zoom: f32

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
        // let spLambertTextured = ShaderProgram::new(
        //     "assets/shaders/v_lamberttextured.glsl", 
        //     None,
        //     "assets/shaders/f_lamberttextured.glsl",
        // );
        let r:Option<f32> = Some(1.0);
        let mainDivs:Option<f32> = Some(36.0);
        let tubeDivs:Option<f32> = Some(36.0);

        let mySphere = Box::new(Sphere::new(r, mainDivs, tubeDivs));
        let myCube = Box::new(Cube::new());
        let myShuttlebug  = Box::new(Shuttlebug::new());




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

        // M = glm::scale(&M, &glm::vec3(10.0,10.0,10.0));

           
        let mut renderer = Renderer {M,V,P,shader: spLambert, models, zoom: 5.0};
        renderer.addModel("cube", myCube);
        renderer.addModel("sphere",mySphere);
        renderer.addModel("ant",myShuttlebug);
        renderer
    }

    fn changeCameraZoom(&mut self) {
        let mut eye = glm::vec3(0.0 ,0.0, -self.zoom);
        let mut center = glm::vec3(0.0, 0.0, 0.0);
        let mut up = glm::vec3(0.0, 1.0, 0.0);
        self.V = glm::look_at(&eye, &center, &up);
        
    }

    fn addModel(&mut self, name: impl Into<String>, model: Box<dyn Model>) {
        self.models.insert(name.into(), model);
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


        // Loop until the user closes the window
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            // gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.shader.use_program();
            // spSimple.use_program();

            let axis = glm::vec3(1.0, 1.0, 0.0); // Y axis
            self.M = glm::rotate(&self.M, (PI)+0.01, &axis);
            // self.V = glm::rotate(&self.V, (PI)+0.01, &axis);
            gl::UniformMatrix4fv(self.shader.get_uniform_location("P"),1,gl::FALSE,self.P.as_ptr());
            gl::UniformMatrix4fv(self.shader.get_uniform_location("V"),1,gl::FALSE,self.V.as_ptr());
            gl::UniformMatrix4fv(self.shader.get_uniform_location("M"),1,gl::FALSE,self.M.as_ptr());
            // gl::UniformMatrix4fv(spConstant.get_uniform_location("M"),1,gl::FALSE,M.as_ptr());
            gl::Uniform4f(self.shader.get_uniform_location("color") as GLint,0.0,1.0,1.0,1.0); 


        
        }
        

        // myCube.draw_solid(true);
        // self.models["ant"].draw_wire(Some(true));
        self.models["sphere"].draw_wire(Some(true));

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




fn get_gl_string(variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl::GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}



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
