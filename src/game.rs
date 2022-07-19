use crate::lib::kinput::*;
use crate::lib::kmath::*;
use crate::krenderer::*;

use glutin::event::VirtualKeyCode;

// yea maybe the event system cleans up the spawning situation

pub struct Game {
    player_position: f32,
    player_velocidad: f32,

    grav_dir: f32,
    
    t: f64,
    t_next_spawn: f64,
    t_next_pickup: Option<f64>,

    score: f64,

    walls: Vec<Rect>,
    pickups: Vec<Vec2>,

    pub paused: bool,
    dead: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_position: 0.3,
            player_velocidad: 0.0,

            grav_dir: 1.0,

            t: 0.0,
            t_next_spawn: 0.0,
            t_next_pickup: None,

            paused: false,

            score: 0.0,

            walls: Vec::new(),
            pickups: Vec::new(),
            dead: false,
        }
    }
    
    pub fn frame(&mut self, inputs: &FrameInputState, kc: &mut KRCanvas) {
        let gravity = 1.8;
        let player_x = 0.5;
        let player_radius = 0.02;
        let forgive_radius = 0.01;
        let pickup_radius = 0.02;
        let pickup_score = 1000.0;

        let wall_speed = 0.45;
        let gap_h = 0.4;
        let gap_t = 2.0;

        
        if inputs.just_pressed(VirtualKeyCode::Space) {
            // self.player_velocidad = -1.0;
            self.grav_dir *= -1.0;
        }
        
        
        if !self.paused && !self.dead {
            self.t += inputs.dt;
            self.score += inputs.dt * 100.0;
            self.player_velocidad += gravity * inputs.dt as f32 * self.grav_dir;
            self.player_position += self.player_velocidad * inputs.dt as f32;
            for wall in self.walls.iter_mut() {
                wall.x -= wall_speed * inputs.dt as f32;
            }
            for pickup in self.pickups.iter_mut() {
                pickup.x -= wall_speed * inputs.dt as f32;
            }
        }
        
        if self.t >= self.t_next_spawn {
            self.t_next_spawn += gap_t;
            let h = kuniform(inputs.seed, 0.0, inputs.screen_rect.bot() - gap_h);
            self.walls.push(Rect::new(inputs.screen_rect.right(), -10.0, 0.2, 10.0 + h));
            self.walls.push(Rect::new(inputs.screen_rect.right(), h + gap_h, 0.2, 10.4));
            
            if chance(inputs.seed * 213235121, 0.5) {
                self.t_next_pickup = Some(self.t + gap_t / 2.0);
            } else {
                self.t_next_pickup = None;
            }
        }

        if let Some(t_next_pickup) = self.t_next_pickup {
            if self.t >= t_next_pickup {
                let new_pickup = Vec2::new(inputs.screen_rect.right() + pickup_radius, if chance(inputs.seed * 123891, 0.5) {inputs.screen_rect.top() + 0.2} else {inputs.screen_rect.bot() - 0.2});
                self.pickups.push(new_pickup);
                self.t_next_pickup = None;
            }
        }

        if inputs.just_pressed(VirtualKeyCode::R) {
            *self = Game::new();
            return;
        }

        // player collides with walls
        let player_pos = Vec2::new(player_x, self.player_position);
        for wall in self.walls.iter() {
            let closest_point = wall.snap(player_pos);
            let penetration = player_radius - (closest_point - player_pos).magnitude();
            if penetration > 0.0 {
                self.dead = true;
            }
        }
        
        if self.player_position < inputs.screen_rect.top() || self.player_position > inputs.screen_rect.bot() {
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
        kc.set_camera(inputs.screen_rect);
        kc.set_depth(1.0);
        kc.set_colour(Vec4::new(0.3, 0.3, 0.7, 1.0));
        kc.rect(inputs.screen_rect);

        // player
        kc.set_depth(1.5);
        kc.set_colour(Vec4::new(1.0, 1.0, 0.0, 1.0));
        kc.circle(player_pos, player_radius + forgive_radius);

        // walls
        kc.set_colour(Vec4::new(0.0, 0.7, 0.0, 1.0));
        for wall in self.walls.iter() {
            kc.rect(*wall);
        }
        // pickups
        kc.set_colour(Vec4::new(0.8, 0.0, 0.0, 1.0));
        for pickup in self.pickups.iter() {
            kc.circle(*pickup, 0.02);
        }

        kc.set_depth(2.0);
        kc.set_colour(Vec4::new(1.0, 1.0, 1.0, 1.0));
        if self.dead {
            let sr = inputs.screen_rect.dilate_pc(-0.6);
            kc.text_center(format!("{:.0}", self.score).as_bytes(), sr);
            kc.text_center("you died, press R to reset".as_bytes(), sr.translate(Vec2::new(0.0, sr.h))); // bug ???
        } else {
            kc.text_left(format!("{:.0}", self.score).as_bytes(), inputs.screen_rect.child(0.0, 0.0, 1.0, 0.05));
        }
        
        if self.paused {
            kc.set_colour(Vec4::new(1.0, 1.0, 1.0, 0.5));
            kc.set_depth(10.0);
            kc.rect(inputs.screen_rect);
        }

    }
}