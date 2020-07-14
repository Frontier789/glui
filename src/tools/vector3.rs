extern crate num;

use self::num::Float;
use super::vector2::Vec2;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use tools::{Mat4, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn grey(xyz: f32) -> Vec3 {
        Vec3 {
            x: xyz,
            y: xyz,
            z: xyz,
        }
    }

    pub fn from_vec2(xy: Vec2, z: f32) -> Vec3 {
        Vec3 {
            x: xy.x,
            y: xy.y,
            z,
        }
    }

    pub fn pol(len: f32, phi: f32, theta: f32) -> Vec3 {
        let cp = Float::cos(phi);

        Vec3 {
            x: len * cp * Float::cos(theta),
            y: len * cp * Float::sin(theta),
            z: len * Float::sin(phi),
        }
    }

    pub fn proj_to_perp(self, n: Vec3) -> Vec3 {
        self - n * self.dot(n)
    }

    pub fn proj_to(self, v: Vec3) -> Vec3 {
        v * self.dot(v)
    }

    pub fn length(self) -> f32 {
        Float::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self, p: f32) -> f32 {
        if p.is_infinite() {
            self.unsign().max()
        } else {
            (self.x.powf(p) + self.y.powf(p) + self.z.powf(p)).powf(1.0 / p)
        }
    }

    pub fn snap_if_close(self, cos_of_angle: f32, directions: Vec<Vec3>) -> Vec3 {
        let len = self.length();

        let mut dir = self / len;
        for principal_dir in directions {
            if principal_dir.dot(dir) > cos_of_angle {
                dir = principal_dir;
            }
        }

        dir * len
    }

    pub fn base_directions() -> Vec<Vec3> {
        vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ]
    }

    pub fn sgn(self) -> Vec3 {
        let l = self.length();
        Vec3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn xzy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }
    pub fn unsign(self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.z.abs(),
            z: self.y.abs(),
        }
    }

    pub fn max(self) -> f32 {
        self.x.max(self.y.max(self.z))
    }

    pub fn dot(self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn intensity(self) -> f32 {
        self.x * 0.299 + self.y * 0.587 + self.z * 0.144
    }

    pub fn rotate_z(self, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();

        Vec3::new(self.x * c + self.y * s, -self.x * s + self.y * c, self.z)
    }
    pub fn rotate_x(self, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();

        Vec3::new(self.x, self.y * c + self.z * s, -self.y * s + self.z * c)
    }
    pub fn rotate_y(self, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();

        Vec3::new(self.x * c + self.z * s, self.y, -self.x * s + self.z * c)
    }
    pub fn rotate(self, axis: Vec3, angle: f32) -> Vec3 {
        (Mat4::rotate(axis, angle) * Vec4::from_vec3(self, 1.0)).xyz()
    }
    pub fn xz(self) -> Vec2 {
        Vec2::new(self.x, self.z)
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

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

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
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// pub type Vec3f = Vec3;
