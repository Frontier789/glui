extern crate num;

use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign,Neg};

#[derive(Copy,Clone,Debug)]
pub struct Vec2px {
    pub x: f32,
    pub y: f32
}

impl Vec2px {
    pub fn origin() -> Self {
        Self{x:0.0,y:0.0}
    }
    
    pub fn zero() -> Self {
        Self{x:0.0,y:0.0}
    }
    
    pub fn inf() -> Self {
        Self{x:std::f32::INFINITY, y:std::f32::INFINITY}
    }
    
    pub fn new<T: Into<f32>>(x: T, y: T) -> Self {
        Self{x:x.into(),y:y.into()}
    }
    
    pub fn pol(len: f32, angle: f32) -> Self {
        Self {
            x: len * num::Float::cos(angle),
            y: len * num::Float::sin(angle)
        }
    }
    
    pub fn length(&self) -> f32 {
        num::Float::sqrt(self.x * self.x + self.y * self.y)
    }
    
    pub fn sgn(&self) -> Self {
        let l = self.length();
        Self {
            x: self.x/l,
            y: self.y/l
        }
    }
    
    pub fn dot(&self, v: Self) -> f32 {
        self.x * v.x + self.y * v.y
    }
    
    pub fn perp(&self) -> Self {
        Self {
            x: self.y*(-1f32),
            y: self.x
        }
    }
    
    pub fn aspect(&self) -> f32 {
        self.x / self.y
    }
}

// A op B

impl Add for Vec2px {
    type Output = Vec2px;

    fn add(self, other: Vec2px) -> Vec2px {
        Vec2px {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2px {
    type Output = Vec2px;

    fn sub(self, other: Vec2px) -> Vec2px {
        Vec2px {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul for Vec2px {
    type Output = Vec2px;

    fn mul(self, other: Vec2px) -> Vec2px {
        Vec2px {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl Div for Vec2px {
    type Output = Vec2px;

    fn div(self, other: Vec2px) -> Vec2px {
        Vec2px {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

// A op= B

impl AddAssign for Vec2px {

    fn add_assign(&mut self, other: Vec2px) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2px {

    fn sub_assign(&mut self, other: Vec2px) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign for Vec2px {

    fn mul_assign(&mut self, other: Vec2px) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl DivAssign for Vec2px {

    fn div_assign(&mut self, other: Vec2px) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

// A op a

impl Mul<f32> for Vec2px {
    type Output = Vec2px;

    fn mul(self, factor: f32) -> Vec2px {
        Vec2px {
            x: self.x * factor,
            y: self.y * factor
        }
    }
}

impl Div<f32> for Vec2px {
    type Output = Vec2px;

    fn div(self, factor: f32) -> Vec2px {
        Vec2px {
            x: self.x / factor,
            y: self.y / factor
        }
    }
}

impl Mul<i32> for Vec2px {
    type Output = Vec2px;

    fn mul(self, factor: i32) -> Vec2px {
        Vec2px {
            x: self.x * factor as f32,
            y: self.y * factor as f32
        }
    }
}

impl Div<i32> for Vec2px {
    type Output = Vec2px;

    fn div(self, factor: i32) -> Vec2px {
        Vec2px {
            x: self.x / factor as f32,
            y: self.y / factor as f32
        }
    }
}

// A op= a

impl MulAssign<f32> for Vec2px {
    fn mul_assign(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl DivAssign<f32> for Vec2px {
    fn div_assign(&mut self, factor: f32) {
        self.x /= factor;
        self.y /= factor;
    }
}

impl MulAssign<i32> for Vec2px {
    fn mul_assign(&mut self, factor: i32) {
        self.x *= factor as f32;
        self.y *= factor as f32;
    }
}

impl DivAssign<i32> for Vec2px {
    fn div_assign(&mut self, factor: i32) {
        self.x /= factor as f32;
        self.y /= factor as f32;
    }
}

// -A

impl Neg for Vec2px {
    type Output = Vec2px;
    
    fn neg(self) -> Vec2px {
        Vec2px::new(
            -self.x,
            -self.y,
        )
    }
}

use super::vector2::Vec2;
impl Vec2px {
    pub fn to_pixels(&self, dpi: f32) -> Vec2 {
        Vec2 {
            x: self.x * dpi,
            y: self.y * dpi,
        }
    }
    
    pub fn from_pixels(pix: Vec2, dpi: f32) -> Vec2px {
        Vec2px {
            x: pix.x / dpi,
            y: pix.y / dpi,
        }
    }
}