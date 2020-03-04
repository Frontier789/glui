extern crate num;

use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign,Neg};
use super::vector2::Vec2;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3{x:0.0,y:0.0,z:0.0}
    }
    
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{x:x,y:y,z:z}
    }
    
    pub fn grey(xyz: f32) -> Vec3 {
        Vec3{x:xyz,y:xyz,z:xyz}
    }
    
    pub fn from_vec2(xy: Vec2, z: f32) -> Vec3 {
        Vec3{x:xy.x,y:xy.y,z:z}
    }
    
    pub fn pol(len: f32, phi: f32, theta: f32) -> Vec3 {
        let cp = num::Float::cos(phi);
        
        Vec3 {
            x: len * cp * num::Float::cos(theta),
            y: len * cp * num::Float::sin(theta),
            z: len * num::Float::sin(phi)
        }
    }
    
    pub fn length(&self) -> f32 {
        num::Float::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
    
    pub fn sgn(&self) -> Vec3 {
        let l = self.length();
        Vec3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }
    
    pub fn xzy(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }
    
    pub fn dot(&self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y*v.z - self.z*v.y,
            y: self.z*v.x - self.x*v.z,
            z: self.x*v.y - self.y*v.x,
        }
    }
}

// A op B

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// A op= B

impl AddAssign for Vec3 {

    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {

    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign for Vec3 {

    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign for Vec3 {

    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

// A op a

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f32) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, factor: f32) -> Vec3 {
        Vec3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: i32) -> Vec3 {
        Vec3 {
            x: self.x * factor as f32,
            y: self.y * factor as f32,
            z: self.z * factor as f32,
        }
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, factor: i32) -> Vec3 {
        Vec3 {
            x: self.x / factor as f32,
            y: self.y / factor as f32,
            z: self.z / factor as f32,
        }
    }
}

// A op= a

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, factor: f32) {
        self.x /= factor;
        self.y /= factor;
        self.z /= factor;
    }
}

impl MulAssign<i32> for Vec3 {
    fn mul_assign(&mut self, factor: i32) {
        self.x *= factor as f32;
        self.y *= factor as f32;
        self.z *= factor as f32;
    }
}

impl DivAssign<i32> for Vec3 {
    fn div_assign(&mut self, factor: i32) {
        self.x /= factor as f32;
        self.y /= factor as f32;
        self.z /= factor as f32;
    }
}

// -A

impl Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        Vec3::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}



// pub type Vec3f = Vec3;