use itertools::Itertools;

/***************************************************
 * Easing
 ***************************************************/
pub fn lerp(x1: f32, x2: f32, t: f32) -> f32 {
    x1 * (1.0 - t) + x2 * t
}

pub fn unlerp(x: f32, t1: f32, t2: f32) -> f32 {
    (x - t1) / (t2 - t1)
}

pub fn remap(x: f32, from_low: f32, from_high: f32, to_low: f32, to_high: f32) -> f32 {
    lerp(to_low, to_high, unlerp(x, from_low, from_high))
}

pub fn gradient(t: f32, colours: Vec<(Vec3, f32)>) -> Vec3 {
    // find nearest 2 neighbours in colours vec and interp between them
    for ((c1, t1), (c2, t2)) in colours.iter().tuple_windows() {
        if t >= *t1 && t <= *t2 {
            return c1.lerp(*c2, unlerp(t, *t1, *t2));
        }
    }

    Vec3::new(1.0, 1.0, 1.0)
}

pub fn cubic_bezier(start: Vec2, c1: Vec2, c2: Vec2, end: Vec2, t: f32) -> Vec2 {
    start.lerp(c1.lerp(c2.lerp(end, t), t), t)
}

/***************************************************
 * RNG
 ***************************************************/

pub fn khash(mut state: u32) -> u32 {
    state = (state ^ 2747636419) * 2654435769;
    state = (state ^ (state >> 16)) * 2654435769;
    state = (state ^ (state >> 16)) * 2654435769;
    state
}

pub fn krand(seed: u32) -> f32 {
    khash(seed) as f32 / 4294967295.0
}

pub fn kuniform(seed: u32, min: f32, max: f32) -> f32 {
    min + (khash(seed) as f32 / 4294967295.0) * (max - min)
}

pub fn chance(seed: u32, percent: f32) -> bool {
    krand(seed) < percent
}

/***************************************************
 * Vec
 ***************************************************/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x * scalar, self.y * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x / scalar, self.y / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
    pub fn dist(&self, other: Vec2) -> f32 { (*self - other).magnitude() }
    pub fn normalize(&self) -> Vec2 { let m = self.magnitude(); if m == 0.0 { *self } else { self.div_scalar(self.magnitude()) }}
    pub fn lerp(&self, other: Vec2, t: f32) -> Vec2 { Vec2::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t)) }
    pub fn rotate(&self, radians: f32) -> Vec2 { 
        Vec2::new(
            self.x * radians.cos() - self.y * radians.sin(), 
            self.x * radians.sin() + self.y * radians.cos()
        ) 
    }
    pub fn offset_r_theta(&self, r: f32, theta: f32) -> Vec2 {
        *self + Vec2::new(r, 0.0).rotate(theta)
    }
    pub fn promote(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }
    // pub fn transform(&self, from: Rect, to: Rect) -> Vec2 {
    //     // maintains its relative position
    //     Vec2::new(
    //         ((self.x - from.x) / from.w) * to.w + to.x,
    //         ((self.y - from.y) / from.h) * to.h + to.y,
    //     )
    // }
    
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: f32) -> Vec2 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        self.mul_scalar(-1.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{x, y, z} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z).sqrt() }
    pub fn square_distance(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z }
    pub fn normalize(&self) -> Vec3 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec3, t: f32) -> Vec3 { Vec3::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t), self.z*(1.0-t) + other.z*(t)) }
    pub fn dist(&self, other: Vec3) -> f32 {(*self - other).magnitude().sqrt()}
    pub fn dot(&self, other: Vec3) -> f32 {self.x*other.x + self.y*other.y + self.z*other.z} // is squ dist lol
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }
    pub fn rotate_about_vec3(&self, axis: Vec3, theta: f32) -> Vec3 {
        *self*theta.cos() + (axis.cross(*self)*theta.sin()) + axis * (axis.dot(*self)*(1.0 - theta.cos()))
    }
    pub fn promote(&self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self.mul_scalar(-1.0)
    }
}

impl std::ops::AddAssign for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!("[{:.*}, {:.*}, {:.*}]", decimals, self.x, decimals, self.y, decimals, self.z);
        f.pad_integral(true, "", &string)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4{x, y, z, w} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec4 { Vec4::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec4 { Vec4::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt() }
    pub fn square_distance(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w }
    pub fn normalize(&self) -> Vec4 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec4, t: f32) -> Vec4 { Vec4::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t), self.z*(1.0-t) + other.z*(t), self.w*(1.0-t) + other.w*t) }
    pub fn dist(&self, other: Vec4) -> f32 {(*self - other).magnitude().sqrt()}
    pub fn dot(&self, other: Vec4) -> f32 {self.x*other.x + self.y*other.y + self.z*other.z} // is squ dist lol
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, _rhs: Vec4) -> Vec4 {
        Vec4 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z, w: self.w - _rhs.w}
    }
}

impl std::ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, _rhs: Vec4) -> Vec4 {
        Vec4 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z, w: self.w + _rhs.w}
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, _rhs: f32) -> Vec4 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, _rhs: Vec4) -> Vec4 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, _rhs: f32) -> Vec4 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        self.mul_scalar(-1.0)
    }
}

impl std::ops::AddAssign for Vec4 {

    fn add_assign(&mut self, rhs: Vec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::fmt::Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!("[{:.*}, {:.*}, {:.*}]", decimals, self.x, decimals, self.y, decimals, self.z);
        f.pad_integral(true, "", &string)
    }
}