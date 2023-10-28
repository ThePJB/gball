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


        // cloud spawning and moving

        // dying of offscreen

        // die to walls
        for wall in self.walls {
            let d = sd_rect(wall, self.player_pos);
            if d < (PLAYER_R_BASE - PLAYER_R_SYMPATHY) {
                // player dies
                self.dead = true;
            }
        }

        // cull offscreen walls
        let mut i = self.pickups.len();
        loop {
            i -= 1;
            
            if self.walls[i].tl().x < self.player_pos.x - 2.0 {
                self.walls.swap_remove(i)
            }

            if i == 0 { break; }
        }

        // collect pickups
        let mut i = self.pickups.len();
        loop {
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
            
            if i == 0 { break; }
        }
    }
}