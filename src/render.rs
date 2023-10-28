use glow_mesh::xyzrgba::*;
use glow_mesh::xyzrgbauv::*;
use crate::game::*;
use minvect::*;

impl Game {

    pub fn get_geometry(&mut self) -> Vec<XYZRGBA> {
            // background clouds etc
            let wall_colour = vec4(0.0, 1.0, 0.3, 1.0).hsv_to_rgb();

            // calculate player vis r

            let mut buf = vec![];
            put_poly(&mut buf, self.player_pos, self.player_r_visual, 69, 0.0, vec4(1.0, 1.0, 0.0, 1.0), 0.0);
            for wall in self.walls {
                glow_mesh::xyzrgba::put_rect(&mut buf, wall, wall_colour, 0.1);
            }

            buf
    }

    pub fn get_text(&mut self) -> Vec<XYZRGBAUV> {
            let wall_colour = vec4(0.0, 1.0, 0.3, 1.0).hsv_to_rgb();

            let mut buf = vec![];
            // etc and include text layout with clipping
            buf
    }
}