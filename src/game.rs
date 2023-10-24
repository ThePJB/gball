use std::f32::consts::PI;

use crate::lib::kinput::*;
use crate::lib::kmath::*;
use crate::krenderer::*;

use glutin::event::VirtualKeyCode;

// yea maybe the event system cleans up the spawning situation

// procedural clouds!! should be easy, rect for straight bottom and variably sized and offset circles
// fade and parallax

// Can't trigger more than once per frame

pub struct RngSequence {
    seed: u32,
}

impl RngSequence {
    pub fn new(seed: u32) -> RngSequence {
        RngSequence {
            seed
        }
    }
    pub fn sample(&mut self) -> u32 {
        let res = khash(self.seed);
        self.seed = khash(self.seed + 394712377);
        res
    }
    pub fn peek(&self) -> u32 {
        khash(self.seed)
    }
}

pub struct RepeatTimer {
    t: f64,
    t_next: f64,
    period: f64,
}

impl RepeatTimer {
    pub fn new(period: f64) -> RepeatTimer {
        return RepeatTimer { 
            t: 0.0, 
            t_next: period, // nb
            period: period,
        };
    }

    pub fn tick(&mut self, dt: f64) -> bool {
        self.t += dt;
        if self.t >= self.t_next {
            self.t_next += self.period;
            return true;
        }
        return false;
    }
}

pub struct Game {
    player_position: f32,
    player_velocidad: f32,
    player_current_anim_r: f32,

    grav_dir: f32,
    
    t: f64,

    score: f64,

    wall_sequence: RngSequence,
    wall_spawn_timer: RepeatTimer,

    walls: Vec<Rect>,
    pickups: Vec<Vec2>,

    clouds_far: Vec<(u32, f32)>,
    clouds_mid: Vec<(u32, f32)>,
    clouds_near: Vec<(u32, f32)>,

    cloud_spawn_timer: RepeatTimer,

    score_lerp_timer: f32,

    tutorial_phase: i32,

    pub paused: bool,
    dead: bool,
}

impl Game {
    pub fn new(seed: u32) -> Game {
        Game {
            player_position: 0.3,
            player_velocidad: 0.0,
            player_current_anim_r: 0.0,

            grav_dir: 1.0,

            t: 0.0,

            paused: false,

            score: 0.0,

            wall_sequence: RngSequence::new(seed * 34982349),
            wall_spawn_timer: RepeatTimer::new(2.0),
            walls: Vec::new(),
            pickups: Vec::new(),

            clouds_far: Vec::new(),
            clouds_mid: Vec::new(),
            clouds_near: Vec::new(),

            cloud_spawn_timer: RepeatTimer::new(1.0),

            score_lerp_timer: 0.0,

            tutorial_phase: 0,

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
        let wall_w = 0.2;

        let score_time = 1.0;

        let game_dt = if self.paused || self.dead {
            0.0
        } else {
            inputs.dt
        };

        
        if inputs.just_pressed(VirtualKeyCode::Space) || inputs.lmb == KeyStatus::JustPressed {
            // self.player_velocidad = -1.0;
            self.grav_dir *= -1.0;
            self.player_current_anim_r = 0.007;
        }
        self.player_current_anim_r = 0.0f32.max(self.player_current_anim_r - 0.05*game_dt as f32);
        // self.player_current_anim_r *= 5.0 * game_dt as f32;

        // if self.grav_dir < 0.0 {
        //     kc.flip_y_h = Some(inputs.screen_rect.h);
        // } else {
        //     kc.flip_y_h = None;
        // }
        
        self.t += game_dt;
        self.score += game_dt * 100.0;
        
        if !self.paused && !self.dead {
            self.player_velocidad += gravity * game_dt as f32 * self.grav_dir;
            self.player_position += self.player_velocidad * game_dt as f32;
            for wall in self.walls.iter_mut() {
                wall.x -= wall_speed * game_dt as f32;
            }
            for pickup in self.pickups.iter_mut() {
                pickup.x -= wall_speed * game_dt as f32;
            }

            // spawn clouds
            if self.cloud_spawn_timer.tick(game_dt) {
                if chance(inputs.seed * 1295497987, 0.1) {
                    self.clouds_near.push((inputs.seed * 982894397, inputs.screen_rect.right() + 0.2));
                }
                if chance(inputs.seed * 35873457, 0.15) {
                    self.clouds_mid.push((inputs.seed * 3842348749, inputs.screen_rect.right() + 0.2));
                }
                if chance(inputs.seed * 576345763, 0.2) {
                    self.clouds_far.push((inputs.seed * 934697577, inputs.screen_rect.right() + 0.2));
                }

            }

            // move clouds
            for i in 0..self.clouds_near.len() {
                let (seed, pos) = self.clouds_near[i];
                self.clouds_near[i] = (seed, pos - game_dt as f32 * 0.1);
            }
            for i in 0..self.clouds_mid.len() {
                let (seed, pos) = self.clouds_mid[i];
                self.clouds_mid[i] = (seed, pos - game_dt as f32 * 0.05);
            }
            for i in 0..self.clouds_far.len() {
                let (seed, pos) = self.clouds_far[i];
                self.clouds_far[i] = (seed, pos - game_dt as f32 * 0.025);
            }
        }


        if self.wall_spawn_timer.tick(game_dt) {
            // let gap_h = kuniform(self.wall_sequence.peek() * 13912417, 0.5, 0.3);
            let h = kuniform(self.wall_sequence.sample(), 0.0, inputs.screen_rect.bot() - gap_h);
            self.walls.push(Rect::new(inputs.screen_rect.right(), -10.0, wall_w, 10.0 + h));
            self.walls.push(Rect::new(inputs.screen_rect.right(), h + gap_h, wall_w, 10.4));
            
            let halfway = ((self.wall_spawn_timer.period / 2.0) * wall_speed as f64) as f32;
            if chance(self.wall_sequence.peek() * 3458793547, 0.5) {
                // place a pickup
                let h =  if chance(inputs.seed * 123891, 0.5) {inputs.screen_rect.top() + 0.2} else {inputs.screen_rect.bot() - 0.2};
                let new_pickup = Vec2::new(inputs.screen_rect.right() + pickup_radius + halfway + wall_w/2.0, h);
                self.pickups.push(new_pickup);
            } else {
                // place an intermediate wall
                if chance(self.wall_sequence.peek() * 548965757, 0.1) {
                    let next_h = kuniform(self.wall_sequence.peek(), 0.0, inputs.screen_rect.bot() - gap_h);
                    let h = (h + next_h)/2.0;
                    self.walls.push(Rect::new(inputs.screen_rect.right() + halfway, -10.0, wall_w, 10.0 + h));
                    self.walls.push(Rect::new(inputs.screen_rect.right() + halfway, h + gap_h, wall_w, 10.4));
                }
            }
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
        
        if self.player_position < inputs.screen_rect.top() - player_radius - forgive_radius || self.player_position > inputs.screen_rect.bot() + player_radius + forgive_radius {
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
        kc.set_colour(Vec4::new(0.0, 0.9, 0.9, 1.0));

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
        let r = self.player_velocidad.abs() * 0.6;
        kc.set_colour(Vec4::new(r, 0.0, 1.0 - r, 1.0));
        kc.circle(player_pos, player_radius + forgive_radius + self.player_current_anim_r);

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

pub fn r_theta_vec(r: f32, theta: f32, orig: Vec2) -> Vec2 {
    Vec2 { x: orig.x + r * theta.cos(), y: orig.y + r * theta.sin() }
}