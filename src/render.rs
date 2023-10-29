use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgbauv::*;
use crate::game::*;
use minvect::*;

impl Game {

    pub fn get_geometry(&self) -> Vec<XYZRGBA> {
            // background clouds etc
            let wall_colour = vec4(0.0, 1.0, 0.3, 1.0).hsv_to_rgb();

            let player_r = PLAYER_R_BASE + (1.0 - (self.t - self.t_press)).max(0.0) * 0.01;

            let mut buf = vec![];
            put_poly(&mut buf, self.player_pos, player_r, 69, 0.0, vec4(1.0, 1.0, 0.0, 1.0), 0.0);
            for wall in self.walls.iter() {
                glow_mesh::xyzrgba::put_rect(&mut buf, *wall, wall_colour, 0.1);
            }

            buf
    }

    pub fn get_text(&self) -> Vec<XYZRGBAUV> {
            let wall_colour = vec4(0.0, 1.0, 0.3, 1.0).hsv_to_rgb();

            let mut buf = vec![];
            // etc and include text layout with clipping
            buf
    }

    pub fn get_camera(&self) -> [f32; 16] {
        // transform to player pos basically etc
        // try drawing a triangle or some shit
        let p = mat4_translation(self.player_pos.x, self.player_pos.y);
        let flipy = [1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let p = mat4_mul(&flipy, &p);
        p
        
    }
}

pub fn mat4_translation(dx: f32, dy: f32) -> [f32; 16] {
    [
        1.0, 0.0, 0.0, -dx,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}