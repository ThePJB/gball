use minirng::hash::next_f32;
use minvect::*;
use crate::game::*;
use std::time::Instant;

fn sd_box(p: Vec2, w: f32, h: f32) -> f32 {
    let d = vec2(p.x.abs(), p.y.abs()) - vec2(w, h);
    let a = vec2(d.x.max(0.0), d.y.max(0.0));
    let b = d.x.max(d.y);
    a.dot(a).sqrt() + b.min(0.0)
}

fn sd_rect(p: Vec2, r: Rect) -> f32 {
    let p = p - r.xy - r.wh / 2.0;
    sd_box(p, r.wh.x / 2.0, r.wh.y / 2.0)
}

impl Game {
    pub fn reset(&mut self) {
        self.t_last_frame = Instant::now();
        self.player_pos = vec2(0.0, -0.9);
        self.player_vel = vec2(PLAYER_SPEED, 0.0);
        self.t_press = 0.0;
        self.grav_dir = 1.0;
        self.x_last_wall = 0.0;
        self.t = 0.0;
        self.score = 0.0;
        self.wall_seed = minirng::hash::random_seed();
        self.t_last_wall = 0.0;
        self.walls = vec![];
        self.pickups = vec![];
        self.paused = false;
        self.dead = false;
        self.press = false;
    }

    pub fn frame(&mut self) {
        let tnow = Instant::now();
        let dt = tnow.duration_since(self.t_last_frame).as_secs_f32();
        self.t_last_frame = tnow;
        self.t += dt;
        if self.press {
            self.press = false;
            self.t_press = self.t;
            self.grav_dir *= -1.0;
        }
        if !self.paused && !self.dead {
            self.update(dt);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.score += 100.0 * dt;
        self.player_vel.y += dt * GRAVITY * self.grav_dir;
        self.player_pos += self.player_vel * dt;
        
        // wall spawning
        if self.player_pos.x - self.x_last_wall > WALL_SEPARATION {
            self.x_last_wall = self.player_pos.x;

            let level_period = 6.0;
            let initial_phase = -2.0;
            let x = self.player_pos.x + level_period + initial_phase;

            if next_f32(&mut self.wall_seed) < PICKUP_CHANCE  {
                let pickup_x = x + level_period * 0.5;
                let pickup_y = next_f32(&mut self.wall_seed)*2.0 - 1.0;
                let pickup_y = pickup_y * 0.8;
                self.pickups.push(vec2(pickup_x, pickup_y));
            } 

            push_wall_rects(&mut self.walls, &mut self.wall_seed, x);

            // i think walls is just rects
        }

        // cloud spawning and moving

        // dying of offscreen
        if self.player_pos.y < -1.0 || self.player_pos.y > 1.0 {
            self.dead = true;
        }

        // die to walls
        for wall in self.walls.iter() {
            let d = sd_rect(self.player_pos, *wall);
            if d < (PLAYER_R_BASE - PLAYER_R_SYMPATHY) {
                // player dies
                self.dead = true;
            }
        }

        // cull offscreen walls
        let mut i = self.walls.len();
        loop {
            if i == 0 { break; }
            i -= 1;
            
            if self.walls[i].tl().x < self.player_pos.x - 2.0 {
                self.walls.swap_remove(i);
            }
        }

        // collect pickups
        let mut i = self.pickups.len();
        loop {
            if i == 0 { break; }
            i -= 1;

            let v = self.player_pos - self.pickups[i];
            let d = v.dot(v).sqrt();
            if d < (PLAYER_R_BASE + PICKUP_RADIUS) {
                self.pickups.swap_remove(i);
                self.score += PICKUP_VALUE;
            } else if self.pickups[i].x < self.player_pos.x - 2.0 {
                // cull offscreen pickups
                self.pickups.swap_remove(i);
            }
        }
    }
}

pub fn push_wall_rects(buf: &mut Vec<Rect>, rng: &mut u32, x: f32) {
    let h = next_f32(rng);
    let h = h * 1.5;
    let wall_x = x;
    let r1 = rect(wall_x, -1.0 - 10.0, WALL_W, h + 10.0);
    let r2 = rect(wall_x, -1.0 + GAP_H + h, WALL_W, 10.0);
    buf.push(r1);
    buf.push(r2);
}