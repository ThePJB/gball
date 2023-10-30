use minvect::*;
use std::collections::HashMap;
use glow_mesh::xyzrgbauv::*;

#[derive(Default)]
pub struct GlyphClips {
    map: HashMap<char, Rect>
}

impl GlyphClips {
    pub fn define_string_rect(&mut self, s: &str, r: Rect) {
        let n = s.len() as f32;
        let cw = r.wh.x / n;
        let mut cr = rect(r.xy.x, r.xy.y, cw, r.wh.y);
        for c in s.chars() {
            self.map.insert(c, cr);
            cr.xy.x += cw
        }
    }

    pub fn push_geometry_for_string(&self, buf: &mut Vec<XYZRGBAUV>, s: &str, r: Rect, col: Vec4, depth: f32) {
        let n = s.len();
        for (i, c) in s.chars().enumerate() {
            let r = r.uniform_grid(i, 0, n, 1);
            put_rect(buf, r, *self.map.get(&c).expect("tried to write unloaded character"), col, depth);
        }
    }
}
