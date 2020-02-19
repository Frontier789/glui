extern crate num;

use std::ops::*;
use std::fmt;
use super::vector3::Vec3;
use super::vector4::Vec4;

#[derive(Copy,Clone,Debug)]
pub struct Mat4 {
    data: [f32; 16]
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        Mat4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
    
    pub fn new() -> Mat4 {
        Mat4::identity()
    }
    
    pub fn zero() -> Mat4 {
        Mat4 {
            data: [
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0
            ]
        }
    }
    
    pub fn from_array(arr: [f32; 16]) -> Mat4 {
        Mat4 {
            data: arr
        }.transpose()
    }
    
    pub fn from_base(i: Vec3, j: Vec3, k: Vec3) -> Mat4 {
        Mat4 {
            data: [
                i.x, i.y, i.z, 0.0,
                j.x, j.y, j.z, 0.0,
                k.x, k.y, k.z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
    
    pub fn scale(ratio: f32) -> Mat4 {
        Mat4 {
            data: [
                ratio, 0.0,   0.0,   0.0,
                0.0,   ratio, 0.0,   0.0,
                0.0,   0.0,   ratio, 0.0,
                0.0,   0.0,   0.0,   1.0
            ]
        }
    }
    
    pub fn offset(p: Vec3) -> Mat4 {
        Mat4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                p.x, p.y, p.z, 1.0
            ]
        }
    }
    
    pub fn transpose(&self) -> Mat4 {
        let mut m: Mat4 = *self;
        
        for x in 1..4 {
            for y in 0..x {
                m.data[x*4+y] = self.data[y*4+x];
                m.data[y*4+x] = self.data[x*4+y];
            }
        }
        
        m
    }
    
    pub fn perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
        
        let zz = (zfar+znear)/(znear-zfar);
        let zw = 2.0*znear*zfar/(znear-zfar);
        
        let tfov = 1.0 / num::Float::tan(fov / 2.0);
        
        Mat4 {
            data: [
                tfov / aspect, 0.0,  0.0, 0.0,
                0.0,           tfov, 0.0, 0.0,
                0.0,           0.0,  zz, -1.0,
                0.0,           0.0,  zw,  0.0
            ]
        }
    }
    
    pub fn ortho(left: f32, bottom: f32, right: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4 {
            data: [
                2.0/(right-left),             0.0,           0.0,                            0.0,
                             0.0,2.0/(top-bottom),           0.0,                            0.0,
                             0.0,             0.0,2.0/(far-near),                            0.0,
                (right+left)/(left-right), (top+bottom)/(bottom-top), (far+near)/(near-far), 1.0,
            ]
        }
    }
    
    pub fn rotate(axis: Vec3, angle: f32) -> Mat4 {
        let sin_a = num::Float::sin(angle*0.5);
		let x = sin_a*axis.x;
		let y = sin_a*axis.y;
		let z = sin_a*axis.z;
        let w = num::Float::cos(angle*0.5);
        
        Mat4::from_array([
            1.0-2.0*y*y-2.0*z*z,   2.0*x*y+2.0*w*z,   2.0*x*z-2.0*w*y, 0.0,
            2.0*x*y-2.0*w*z, 1.0-2.0*x*x-2.0*z*z,   2.0*y*z+2.0*w*x, 0.0,
            2.0*x*z+2.0*w*y,   2.0*y*z-2.0*w*x, 1.0-2.0*x*x-2.0*y*y, 0.0,
                    0.0,             0.0,             0.0, 1.0
        ])
    }
    
    pub fn rotate_x(angle: f32) -> Mat4 {
        let c = num::Float::cos(angle);
        let s = num::Float::sin(angle);
        
        Mat4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0,   c,   s, 0.0,
                0.0,  -s,   c, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
    
    pub fn rotate_y(angle: f32) -> Mat4 {
        let c = num::Float::cos(angle);
        let s = num::Float::sin(angle);
        
        Mat4 {
            data: [
                  c, 0.0,   s, 0.0,
                0.0, 1.0, 0.0, 0.0,
                 -s, 0.0,   c, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
    
    pub fn rotate_z(angle: f32) -> Mat4 {
        let c = num::Float::cos(angle);
        let s = num::Float::sin(angle);
        
        Mat4 {
            data: [
                  c,   s, 0.0, 0.0,
                 -s,   c, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
    
    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }
    
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr()
    }
}

impl Index<usize> for Mat4 {
    type Output = [f32];

    fn index(&self, column: usize) -> &Self::Output {
        &self.data[column*4..column*4+4]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, column: usize) -> &mut Self::Output {
        &mut self.data[column*4..column*4+4]
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    
    fn mul(self, other: Mat4) -> Mat4 {
        let mut m = Mat4::zero();
        
        for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                m.data[x*4+y] += self.data[z*4+y] * other.data[x*4+z];
            }
        }}
        
        m
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    
    fn mul(self, v: Vec4) -> Vec4 {
        Vec4::new(
            self.data[0] * v.x + self.data[4] * v.y + self.data[8]  * v.z + self.data[12] * v.w,
            self.data[1] * v.x + self.data[5] * v.y + self.data[9]  * v.z + self.data[13] * v.w,
            self.data[2] * v.x + self.data[6] * v.y + self.data[10] * v.z + self.data[14] * v.w,
            self.data[3] * v.x + self.data[7] * v.y + self.data[11] * v.z + self.data[15] * v.w,
        )
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}",
               self.data[0],self.data[4],self.data[8] ,self.data[12],
               self.data[1],self.data[5],self.data[9] ,self.data[13],
               self.data[2],self.data[6],self.data[10],self.data[14],
               self.data[3],self.data[7],self.data[11],self.data[15])
    }
}