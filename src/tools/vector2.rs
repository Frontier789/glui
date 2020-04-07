extern crate num;

use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign,Neg};

#[derive(Copy,Clone,Debug,Default,PartialEq,Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign,Neg)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn origin() -> Self {
        Self{x:0.0, y:0.0}
    }
    
    pub fn zero() -> Self {
        Self{x:0.0, y:0.0}
    }
    
    pub fn inf() -> Self {
        Self{x:std::f32::INFINITY, y:std::f32::INFINITY}
    }
    
    pub fn new(x: f32, y: f32) -> Self {
        Self{x:x, y:y}
    }
    
    pub fn new_xy(xy: f32) -> Self {
        Self{x:xy, y:xy}
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
    
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    pub fn sgn(&self) -> Self {
        let l = self.length();
        Self {
            x: self.x/l,
            y: self.y/l
        }
    }
    
    pub fn minxy(&self) -> f32 {
        if self.x < self.y {
            self.x
        } else {
            self.y
        }
    }
    
    pub fn unsign(&self) -> Self {
        Vec2::new(
            f32::abs(self.x),
            f32::abs(self.y),
        )
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

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x / factor,
            y: self.y / factor
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, factor: i32) -> Vec2 {
        Vec2 {
            x: self.x * factor as f32,
            y: self.y * factor as f32
        }
    }
}

impl Div<i32> for Vec2 {
    type Output = Vec2;

    fn div(self, factor: i32) -> Vec2 {
        Vec2 {
            x: self.x / factor as f32,
            y: self.y / factor as f32
        }
    }
}

// A op= a

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, factor: f32) {
        self.x /= factor;
        self.y /= factor;
    }
}

impl MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, factor: i32) {
        self.x *= factor as f32;
        self.y *= factor as f32;
    }
}

impl DivAssign<i32> for Vec2 {
    fn div_assign(&mut self, factor: i32) {
        self.x /= factor as f32;
        self.y /= factor as f32;
    }
}

// pub type Vec2f = Vec2;