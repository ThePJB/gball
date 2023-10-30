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
            for p in self.pickups.iter() {
                let pickup_colour = vec4(1.0, 0.0, 0.0, 1.0);
                glow_mesh::xyzrgba::put_poly(&mut buf, *p, PICKUP_RADIUS, 16, 0.0, pickup_colour, 0.1);
            }
            


            buf
    }

    pub fn get_text(&self) -> Vec<XYZRGBAUV> {
            let wall_colour = vec4(0.0, 1.0, 0.3, 1.0).hsv_to_rgb();

            let mut buf = vec![];
            // etc and include text layout with clipping
            let score_str = format!("{:.0}", self.score);
            let len = score_str.len() as f32;
            let cw = score_str.len() as f32 * 0.01;
            let ch = cw * 8./7.;
            let score_r = rect(-cw*len/2.0, -1.0 + ch, cw*len, ch);
            self.glyph_clips.push_geometry_for_string(&mut buf, &score_str, score_r, vec4(1.0, 1.0, 1.0, 1.0), -0.9);


            buf
    }

    pub fn get_camera(&self) -> [f32; 16] {
        // transform to player pos basically etc
        // try drawing a triangle or some shit
        let p = mat4_translation(self.player_pos.x + CAM_X_OFFSET, self.player_pos.y);
        let flipy = [1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let p = mat4_mul(&flipy, &p);
        let undistort = [
            self.yres / self.xres, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ];
        let p = mat4_mul(&undistort, &p);
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