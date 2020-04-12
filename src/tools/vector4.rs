extern crate num;

use super::derive_more::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};
use super::vector3::Vec3;

#[derive(Copy,Clone,Debug,PartialEq,Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn origin() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
    pub fn grey(xyz: f32) -> Vec4 {
        Vec4 {
            x: xyz,
            y: xyz,
            z: xyz,
            w: 1.0,
        }
    }

    pub fn from_vec3(xyz: Vec3, w: f32) -> Vec4 {
        Vec4 {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
            w: w,
        }
    }

    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Vec4 {
        Vec4 {
            x: r as f32 / 255.0,
            y: g as f32 / 255.0,
            z: b as f32 / 255.0,
            w: a as f32 / 255.0,
        }
    }
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn rgb(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
    pub fn dot(&self, v: Vec4) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn intensity(&self) -> f32 {
        self.x * 0.299 + self.y * 0.587 + self.z * 0.144
    }

    pub const WHITE: Vec4 = Vec4 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };
    pub const BLACK: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub const RED: Vec4 = Vec4 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub const GREEN: Vec4 = Vec4 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 1.0,
    };
    pub const BLUE: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 1.0,
    };
}

impl Default for Vec4 {
    fn default() -> Vec4 {
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    }
}

// impl Mul<f32> for Vec4 {
//     type Output = Vec4;

//     fn mul(self, factor: f32) -> Vec4 {
//         Vec4 {
//             x: self.x * factor,
//             y: self.y * factor,
//             z: self.z * factor,
//             w: self.w * factor,
//         }
//     }
// }

// impl Div<f32> for Vec4 {
//     type Output = Vec4;

//     fn div(self, factor: f32) -> Vec4 {
//         Vec4 {
//             x: self.x / factor,
//             y: self.y / factor,
//             z: self.z / factor,
//             w: self.w / factor,
//         }
//     }
// }

// impl Mul<i32> for Vec4 {
//     type Output = Vec4;

//     fn mul(self, factor: i32) -> Vec4 {
//         Vec4 {
//             x: self.x * factor as f32,
//             y: self.y * factor as f32,
//             z: self.z * factor as f32,
//             w: self.w * factor as f32,
//         }
//     }
// }

// impl Div<i32> for Vec4 {
//     type Output = Vec4;

//     fn div(self, factor: i32) -> Vec4 {
//         Vec4 {
//             x: self.x / factor as f32,
//             y: self.y / factor as f32,
//             z: self.z / factor as f32,
//             w: self.w / factor as f32,
//         }
//     }
// }

// // A op= a

// impl MulAssign<f32> for Vec4 {
//     fn mul_assign(&mut self, factor: f32) {
//         self.x *= factor;
//         self.y *= factor;
//         self.z *= factor;
//         self.w *= factor;
//     }
// }

// impl DivAssign<f32> for Vec4 {
//     fn div_assign(&mut self, factor: f32) {
//         self.x /= factor;
//         self.y /= factor;
//         self.z /= factor;
//         self.w /= factor;
//     }
// }

// impl MulAssign<i32> for Vec4 {
//     fn mul_assign(&mut self, factor: i32) {
//         self.x *= factor as f32;
//         self.y *= factor as f32;
//         self.z *= factor as f32;
//         self.w *= factor as f32;
//     }
// }

// impl DivAssign<i32> for Vec4 {
//     fn div_assign(&mut self, factor: i32) {
//         self.x /= factor as f32;
//         self.y /= factor as f32;
//         self.z /= factor as f32;
//         self.w /= factor as f32;
//     }
// }

// pub type Vec4f = Vec4;
