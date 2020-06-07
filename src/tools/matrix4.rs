extern crate num;

use super::vector3::Vec3;
use super::vector4::Vec4;
use std::fmt;
use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4 {
    data: [f32; 16],
}

impl Mat4 {
    pub fn from_arr_arr(data: [[f32; 4]; 4]) -> Mat4 {
        Mat4 {
            data: [
                data[0][0], data[1][0], data[2][0], data[3][0], data[0][1], data[1][1], data[2][1],
                data[3][1], data[0][2], data[1][2], data[2][2], data[3][2], data[0][3], data[1][3],
                data[2][3], data[3][3],
            ],
        }
    }
    pub fn identity() -> Mat4 {
        Mat4::from_arr_arr([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn new() -> Mat4 {
        Mat4::identity()
    }

    pub fn zero() -> Mat4 {
        Mat4::from_arr_arr([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    pub fn from_array(arr: [f32; 16]) -> Mat4 {
        Mat4 { data: arr }.transpose()
    }

    pub fn from_base(i: Vec3, j: Vec3, k: Vec3) -> Mat4 {
        Mat4::from_arr_arr([
            [i.x, j.x, k.x, 0.0],
            [i.y, j.y, k.y, 0.0],
            [i.z, j.z, k.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale(ratio: f32) -> Mat4 {
        Mat4::from_arr_arr([
            [ratio, 0.0, 0.0, 0.0],
            [0.0, ratio, 0.0, 0.0],
            [0.0, 0.0, ratio, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn offset(p: Vec3) -> Mat4 {
        Mat4::from_arr_arr([
            [1.0, 0.0, 0.0, p.x],
            [0.0, 1.0, 0.0, p.y],
            [0.0, 0.0, 1.0, p.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn transpose(&self) -> Mat4 {
        let d = &self.data;
        Mat4 {
            data: [
                d[0],
                d[4],
                d[8],
                d[12],
                d[0 + 1],
                d[4 + 1],
                d[8 + 1],
                d[12 + 1],
                d[0 + 2],
                d[4 + 2],
                d[8 + 2],
                d[12 + 2],
                d[0 + 3],
                d[4 + 3],
                d[8 + 3],
                d[12 + 3],
            ],
        }
    }

    pub fn perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
        let zz = (zfar + znear) / (znear - zfar);
        let zw = 2.0 * znear * zfar / (znear - zfar);

        let tfov = 1.0 / (fov * 0.5).tan();

        Mat4::from_arr_arr([
            [tfov / aspect, 0.0, 0.0, 0.0],
            [0.0, tfov, 0.0, 0.0],
            [0.0, 0.0, zz, zw],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
    pub fn inv_perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
        let zz = (zfar + znear) / (znear - zfar);
        let zw = 2.0 * znear * zfar / (znear - zfar);

        let tfov = 1.0 / (fov * 0.5).tan();

        Mat4::from_arr_arr([
            [aspect / tfov, 0.0, 0.0, 0.0],
            [0.0, 1.0 / tfov, 0.0, 0.0],
            [0.0, 0.0, 0.0, -1.0],
            [0.0, 0.0, 1.0 / zw, zz / zw],
        ])
    }

    pub fn ortho(left: f32, bottom: f32, right: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4::from_arr_arr([
            [
                2.0 / (right - left),
                0.0,
                0.0,
                (right + left) / (left - right),
            ],
            [
                0.0,
                2.0 / (top - bottom),
                0.0,
                (top + bottom) / (bottom - top),
            ],
            [0.0, 0.0, 2.0 / (far - near), (far + near) / (near - far)],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate(axis: Vec3, angle: f32) -> Mat4 {
        let sin_a = (angle * 0.5).sin();
        let x = sin_a * axis.x;
        let y = sin_a * axis.y;
        let z = sin_a * axis.z;
        let w = (angle * 0.5).cos();

        Mat4::from_array([
            1.0 - 2.0 * y * y - 2.0 * z * z,
            2.0 * x * y + 2.0 * w * z,
            2.0 * x * z - 2.0 * w * y,
            0.0,
            2.0 * x * y - 2.0 * w * z,
            1.0 - 2.0 * x * x - 2.0 * z * z,
            2.0 * y * z + 2.0 * w * x,
            0.0,
            2.0 * x * z + 2.0 * w * y,
            2.0 * y * z - 2.0 * w * x,
            1.0 - 2.0 * x * x - 2.0 * y * y,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    pub fn rotate_x(angle: f32) -> Mat4 {
        let (s, c) = angle.sin_cos();

        Mat4::from_arr_arr([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, s, 0.0],
            [0.0, -s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_y(angle: f32) -> Mat4 {
        let (s, c) = angle.sin_cos();

        Mat4::from_arr_arr([
            [c, 0.0, s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_z(angle: f32) -> Mat4 {
        let (s, c) = angle.sin_cos();

        Mat4::from_arr_arr([
            [c, s, 0.0, 0.0],
            [-s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_from_to(from: Vec3, to: Vec3) -> Mat4 {
        let v = from.cross(to);
        let c = from.dot(to);

        let vx = Mat4::from_arr_arr([
            [0.0, -v.z, v.y, 0.0],
            [v.z, 0.0, -v.x, 0.0],
            [-v.y, v.x, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Mat4::identity() + vx + vx * vx / (1.0 + c)
    }

    pub fn look_at(camera_position: Vec3, target_position: Vec3, up: Vec3) -> Mat4 {
        let t: Mat4 = Mat4::offset(-camera_position);
        let f: Vec3 = (target_position - camera_position).sgn();
        let r: Vec3 = f.cross(up).sgn();
        let u: Vec3 = r.cross(f).sgn();

        Mat4::from_arr_arr([
            [r.x, r.y, r.z, 0.0],
            [u.x, u.y, u.z, 0.0],
            [-f.x, -f.y, -f.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * t
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
        &self.data[column * 4..column * 4 + 4]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, column: usize) -> &mut Self::Output {
        &mut self.data[column * 4..column * 4 + 4]
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut m = Mat4::zero();

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    m.data[x * 4 + y] += other.data[x * 4 + z] * self.data[z * 4 + y];
                }
            }
        }

        m
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Vec4 {
        Vec4::new(
            self.data[0] * v.x + self.data[4] * v.y + self.data[8] * v.z + self.data[12] * v.w,
            self.data[1] * v.x + self.data[5] * v.y + self.data[9] * v.z + self.data[13] * v.w,
            self.data[2] * v.x + self.data[6] * v.y + self.data[10] * v.z + self.data[14] * v.w,
            self.data[3] * v.x + self.data[7] * v.y + self.data[11] * v.z + self.data[15] * v.w,
        )
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
                self.data[3] + rhs.data[3],
                self.data[4] + rhs.data[4],
                self.data[5] + rhs.data[5],
                self.data[6] + rhs.data[6],
                self.data[7] + rhs.data[7],
                self.data[8] + rhs.data[8],
                self.data[9] + rhs.data[9],
                self.data[10] + rhs.data[10],
                self.data[11] + rhs.data[11],
                self.data[12] + rhs.data[12],
                self.data[13] + rhs.data[13],
                self.data[14] + rhs.data[14],
                self.data[15] + rhs.data[15],
            ],
        }
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Mat4 {
        Mat4 {
            data: [
                self.data[0] * rhs,
                self.data[1] * rhs,
                self.data[2] * rhs,
                self.data[3] * rhs,
                self.data[4] * rhs,
                self.data[5] * rhs,
                self.data[6] * rhs,
                self.data[7] * rhs,
                self.data[8] * rhs,
                self.data[9] * rhs,
                self.data[10] * rhs,
                self.data[11] * rhs,
                self.data[12] * rhs,
                self.data[13] * rhs,
                self.data[14] * rhs,
                self.data[15] * rhs,
            ],
        }
    }
}

impl Div<f32> for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: f32) -> Mat4 {
        Mat4 {
            data: [
                self.data[0] / rhs,
                self.data[1] / rhs,
                self.data[2] / rhs,
                self.data[3] / rhs,
                self.data[4] / rhs,
                self.data[5] / rhs,
                self.data[6] / rhs,
                self.data[7] / rhs,
                self.data[8] / rhs,
                self.data[9] / rhs,
                self.data[10] / rhs,
                self.data[11] / rhs,
                self.data[12] / rhs,
                self.data[13] / rhs,
                self.data[14] / rhs,
                self.data[15] / rhs,
            ],
        }
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}",
            self.data[0],
            self.data[4],
            self.data[8],
            self.data[12],
            self.data[1],
            self.data[5],
            self.data[9],
            self.data[13],
            self.data[2],
            self.data[6],
            self.data[10],
            self.data[14],
            self.data[3],
            self.data[7],
            self.data[11],
            self.data[15]
        )
    }
}
