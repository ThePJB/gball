use std::time::Instant;

use glow::HasContext;
use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgbauv::*;
use minvect::*;
use minirng::hash::random_seed;
use glutin::event::{Event, WindowEvent};

use glutin::event::VirtualKeyCode;
use winit::event::ElementState;

pub const PLAYER_R_BASE: f32 = 0.03;
pub const PLAYER_R_SYMPATHY: f32 = 0.01;
pub const GRAVITY: f32 = 1.8;
pub const CAM_X_OFFSET: f32 = 0.5;
pub const PICKUP_VALUE: f32 = 1000.0;
pub const PICKUP_RADIUS: f32 = 0.015;
pub const GAP_H: f32 = 0.4;
pub const WALL_W: f32 = 0.2;
pub const WALL_SEPARATION: f32 = 2.0;

pub struct Game {
    pub gl: glow::Context,
    pub window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub prog_shape: ProgramXYZRGBA,
    pub prog_text: ProgramXYZRGBAUV,
    pub xres: f32,
    pub yres: f32,

    pub t_last_frame: Instant,
    pub player_pos: Vec2,
    pub player_vel: Vec2,
    pub t_press: f32,
    
    pub x_last_wall: f32,

    pub grav_dir: f32,
    
    pub t: f32,

    pub score: f32,

    pub wall_seed: u32,
    pub t_last_wall: f32,

    pub walls: Vec<Rect>,
    pub pickups: Vec<Vec2>,

    pub clouds_far: Vec<(u32, f32)>,
    pub clouds_mid: Vec<(u32, f32)>,
    pub clouds_near: Vec<(u32, f32)>,

    pub t_last_cloud: f32,

    pub paused: bool,
    pub dead: bool,
    pub press: bool,
}

impl Game {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Self {
        let xres = 900.0f32;
        let yres = 900.0f32;
    
        unsafe {
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("gball")
                .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
    
            let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            gl.enable(glow::DEPTH_TEST);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.depth_func(glow::LEQUAL);
            gl.enable(glow::BLEND);
    
            let prog_shape = ProgramXYZRGBA::default(&gl);
            let img = minimg::ImageBuffer::from_bytes(include_bytes!("../atlas.png"));
            let prog_text = ProgramXYZRGBAUV::default(&gl, &img);

            let now = Instant::now();
            Game {
                gl,
                window,
                xres,
                yres,
                prog_shape,
                prog_text,
                t_last_frame: now,
                player_pos: vec2(0.0, -0.9),
                player_vel: vec2(0.45, 0.0),
                t_press: 0.0,
                x_last_wall: 0.0,
                grav_dir: 1.0,
                t: 0.0,
                score: 0.0,
                wall_seed: random_seed(),
                t_last_wall: 0.0,
                walls: vec![],
                pickups: vec![],
                clouds_far: vec![],
                clouds_mid: vec![],
                clouds_near: vec![],
                t_last_cloud: 0.0,
                paused: false,
                dead: false,
                press: false,
            }
        }
    }
    
    pub fn handle_event(&mut self, event: &glutin::event::Event<()>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    self.window.resize(*physical_size);
                    self.xres = physical_size.width as f32;
                    self.yres = physical_size.height as f32;
                    unsafe {self.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32)};
                },
                WindowEvent::Focused(false) => {
                    self.paused = true;
                },
                WindowEvent::Focused(true) => {
                    self.paused = false;
                },
                WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::Space {
                            if input.state == ElementState::Pressed {
                                if self.press {
                                    self.press = false;
                                } else {
                                    self.press = true;
                                }
                            }
                        } else if keycode == VirtualKeyCode::R {
                            self.reset();
                        }
                    }
                },
                _ => {},
            }
            Event::MainEventsCleared => {
                self.frame();
                self.render();
            },
            _ => {},
        }
    }

    pub fn render(&self) {
        unsafe {
            self.gl.clear_color(0.65, 0.65, 1.0, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
            let buf = self.get_geometry();
            let h = upload_xyzrgba_mesh(&buf, &self.gl);
            self.prog_shape.bind(&self.gl);
            self.prog_shape.set_proj(&self.get_camera(), &self.gl);
            h.render(&self.gl);
            // let buf = self.get_text();
            // let h = upload_xyzrgbauv_mesh(&buf, &self.gl);
            // self.prog_text.bind(&self.gl);
            // h.render(&self.gl);
            self.window.swap_buffers().unwrap();
        }
    }

    pub fn reset(&mut self) {
        self.t_last_frame = Instant::now();
        self.player_pos = vec2(0.0, -0.9);
        self.player_vel = vec2(0.45, 0.0);
        self.t_press = 0.0;
        self.grav_dir = 1.0;
        self.x_last_wall = 0.0;
        self.t = 0.0;
        self.score = 0.0;
        self.wall_seed = random_seed();
        self.t_last_wall = 0.0;
        self.walls = vec![];
        self.pickups = vec![];
        self.paused = false;
        self.dead = false;
        self.press = false;
    }
}

// fade score in death screen
