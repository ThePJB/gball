use std::f32::consts::PI;
use std::time::Instant;

use crate::lib::kinput::*;

use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgbauv::*;
use minvect::*;
use minirng::hash::random_seed;
use glutin::event::{Event, WindowEvent};


use glutin::event::VirtualKeyCode;
use winit::event::ElementState;

pub const PLAYER_R_BASE: f32 = 0.02;
pub const PLAYER_R_SYMPATHY: f32 = 0.01;
pub const GRAVITY: f32 = 1.8;
pub const CAM_X_OFFSET: f32 = 0.5;
pub const PICKUP_VALUE: f32 = 1000.0;
pub const PICKUP_RADIUS: f32 = 0.015;
pub const GAP_H: f32 = 0.4;
pub const WALL_W: f32 = 0.2;

pub struct Game {
    pub gl: glow::Context,
    pub window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub prog_shape: ProgramXYZRGBA,
    pub prog_text: ProgramXYZRGBAUV,
    pub xres: i32,
    pub yres: i32,

    pub t_last_frame: Instant,
    pub player_pos: Vec2,
    pub player_vel: Vec2,
    pub t_press: Instant,

    pub grav_dir: f32,
    
    pub t: f32,

    pub score: f64,

    pub wall_seed: u32,
    pub t_last_wall: f32,

    pub walls: Vec<Rect>,
    pub pickups: Vec<Vec2>,

    pub clouds_far: Vec<(u32, f32)>,
    pub clouds_mid: Vec<(u32, f32)>,
    pub clouds_near: Vec<(u32, f32)>,

    pub t_last_cloud: Instant,

    pub paused: bool,
    pub dead: bool,
    pub press: bool,
}

impl Game {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Self {
        let xres = 900;
        let yres = 900;
    
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
                t_press: now,
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
                t_last_cloud: now,
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
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
            let buf = self.get_geometry();
            let h = upload_xyzrgba_mesh(&buf, &self.gl);
            self.prog_shape.bind(&self.gl);
            h.render(&self.gl);
            let buf = self.get_text();
            let h = upload_xyzrgbauv_mesh(&buf, &self.gl);
            self.prog_text.bind(&self.gl);
            h.render(&self.gl);
            self.window.swap_buffers().unwrap();
        }
    }
    
    pub fn frame(&mut self, inputs: &FrameInputState, kc: &mut KRCanvas) {

        
        if inputs.just_pressed(VirtualKeyCode::Space) || inputs.lmb == KeyStatus::JustPressed {
            // self.player_v = -1.0;
            self.player_r = 0.007;
        }
        self.player_r = 0.0f32.max(self.player_r - 0.05*game_dt as f32);
        // self.player_r *= 5.0 * game_dt as f32;

        // if self.grav_dir < 0.0 {
        //     kc.flip_y_h = Some(inputs.screen_rect.h);
        // } else {
        //     kc.flip_y_h = None;
        // }
        


        // player collides with walls
        let player_pos = Vec2::new(player_x, self.player_h);
        for wall in self.walls.iter() {
            let closest_point = wall.snap(player_pos);
            let penetration = player_radius - (closest_point - player_pos).magnitude();
            if penetration > 0.0 {
                self.dead = true;
            }
        }
        
        if self.player_h < inputs.screen_rect.top() - player_radius - forgive_radius || self.player_h > inputs.screen_rect.bot() + player_radius + forgive_radius {
            self.dead = true;
        }

        let mut i = self.pickups.len();
        while i > 0 {
            i = i - 1;
            if self.pickups[i].dist(player_pos) < player_radius + pickup_radius + forgive_radius {
                self.score += pickup_score;
                self.pickups.swap_remove(i);
            } else {
                if self.pickups[i].x - pickup_radius < 0.0 {
                    self.pickups.swap_remove(i);
                }
            }
        }
        
        self.walls.retain(|w| w.right() > 0.0);

        // bg
        let (sky, ocean) = inputs.screen_rect.split_ud(0.7);
        kc.set_camera(inputs.screen_rect);
        kc.set_depth(1.0);
        let col_top = Vec4::new(0.2, 0.2, 0.8, 1.0);
        // let col_bot = Vec4::new(0.3, 0.3, 0.7, 1.0);
        let col_bot = Vec4::new(0.3, 0.3, 1.0, 1.0);

        kc.grad_rect_ud(sky, col_top, col_bot);
        
        kc.set_depth(1.05);
        let col_far = Vec4::new(0.2, 0.2, 0.55, 1.0);
        let col_near = Vec4::new(0.2, 0.2, 0.65, 1.0);
        kc.grad_rect_ud(ocean, col_far, col_near);

        // clouds
        kc.set_depth(1.1);
        kc.set_colour(Vec4::new(0.6, 0.6, 0.7, 1.0));
        for (seed, xpos) in &self.clouds_far {
            kc.cloud(Rect::new(*xpos, 0.6, 0.1, 0.05), *seed)            
        }
        kc.set_depth(1.2);
        kc.set_colour(Vec4::new(0.65, 0.65, 0.75, 1.0));
        for (seed, xpos) in &self.clouds_mid {
            kc.cloud(Rect::new(*xpos, 0.533, 0.15, 0.07), *seed)            
        }
        kc.set_depth(1.3);
        kc.set_colour(Vec4::new(0.7, 0.7, 0.8, 1.0));
        for (seed, xpos) in &self.clouds_near {
            kc.cloud(Rect::new(*xpos, 0.467, 0.2, 0.09), *seed)            
        }
        
        // player
        kc.set_depth(1.5);
        let r = self.player_v.abs() * 0.6;
        kc.set_colour(Vec4::new(r, 0.9, 0.9 - r, 1.0));

        let r = (player_radius + forgive_radius) * 0.9;
        if self.grav_dir > 0.0 {
            kc.triangle(
                r_theta_vec(r, PI/2.0, player_pos),
                r_theta_vec(r, PI/2.0 + 2.0*PI/3.0, player_pos),
                r_theta_vec(r, PI/2.0 + 4.0*PI/3.0, player_pos),
            );
        } else {
            kc.triangle(
                r_theta_vec(r, PI + PI/2.0, player_pos),
                r_theta_vec(r, PI + PI/2.0 + 2.0*PI/3.0, player_pos),
                r_theta_vec(r, PI + PI/2.0 + 4.0*PI/3.0, player_pos),
            );
        }
        kc.set_colour(Vec4::new(1.0, 0.0, 1.0, 1.0));
        kc.circle(player_pos, player_radius + forgive_radius + self.player_r);

        // walls
        kc.set_colour(Vec4::new(0.4, 0.0, 0.0, 1.0));
        for wall in self.walls.iter() {
            kc.rect(*wall);
        }
        
        //     let (l, r) = wall.split_lr(0.5);

            
        //     let col_l = Vec4::new(0.7, 0.45, 0.2, 1.0);
        //     let col_r = Vec4::new(0.6, 0.45, 0.2, 1.0);
        //     kc.grad_rect_lr(l, col_l, col_c);
        //     kc.grad_rect_lr(r, col_c, col_r);

        // }
        // pickups
        kc.set_colour(Vec4::new(0.8, 0.0, 0.0, 1.0));
        for pickup in self.pickups.iter() {
            kc.circle(*pickup, 0.02);
        }

        // paused overlay
        if self.paused {
            kc.set_colour(Vec4::new(1.0, 1.0, 1.0, 0.5));
            kc.set_depth(10.0);
            kc.rect(inputs.screen_rect);
        }

        // text + control flow

        kc.set_depth(2.0);
        kc.set_colour(Vec4::new(1.0, 1.0, 1.0, 1.0));

        let alive_score_rect = inputs.screen_rect.child(0.0, 0.0, 1.0, 0.05);
        let dead_score_rect = inputs.screen_rect.child(0.0, 0.4, 1.0, 0.2);
        if !self.dead {
            let sr = inputs.screen_rect.child(0.0, 0.0, 1.0, 0.05);
            kc.text_center(format!("{:.0}", self.score).as_bytes(), sr);
        } else {
            self.score_lerp_timer += inputs.dt as f32;
            let mut text_rect = inputs.screen_rect.dilate_pc(-0.2);
            text_rect.y += 0.2;
            
            if self.score_lerp_timer/score_time > 1.0 {
                self.score_lerp_timer = 1.0*score_time;
                kc.text_center("You died, press space to reset".as_bytes(), text_rect); // bug ???
                if inputs.just_pressed(VirtualKeyCode::Space) {
                    *self = Game::new(inputs.seed);
                }
            }
            let sr = alive_score_rect.lerp(dead_score_rect, self.score_lerp_timer/score_time);
            kc.text_center(format!("{:.0}", self.score).as_bytes(), sr);
            
        }

        if self.dead && self.tutorial_phase < 2 {
            *self = Game::new(inputs.seed);
        }
    }
}

// fade score in death screen
