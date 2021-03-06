#[cfg(feature = "serializable")]
extern crate serde;

#[cfg(feature = "serializable")]
use self::serde::Deserialize;
#[cfg(feature = "serializable")]
use self::serde::Serialize;

use std::f32::consts::PI;
use tools::{Mat4, Vec3, Vec4};

#[cfg_attr(feature = "serializable", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub enum Projection {
    Perspective,
    Orthogonal,
}

#[cfg_attr(feature = "serializable", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct CameraSpatialParams {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

impl Default for CameraSpatialParams {
    fn default() -> Self {
        CameraSpatialParams {
            pos: Vec3::origin(),
            target: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

impl CameraSpatialParams {
    pub fn l(&self) -> Vec3 {
        self.up.cross(self.v()).sgn()
    }
    pub fn r(&self) -> Vec3 {
        self.v().cross(self.up).sgn()
    }
    pub fn u(&self) -> Vec3 {
        self.up
    }
    pub fn v(&self) -> Vec3 {
        (self.target - self.pos).sgn()
    }
    pub fn f(&self) -> Vec3 {
        self.v().proj_to_perp(Vec3::new(0.0, 1.0, 0.0))
    }
    pub fn pos_to_target(&self) -> Vec3 {
        self.target - self.pos
    }
    pub fn target_to_pos(&self) -> Vec3 {
        self.pos - self.target
    }
    pub fn move_to(&mut self, pos: Vec3) {
        let d = pos - self.pos;
        self.pos = pos;
        self.target += d;
    }
    pub fn move_by(&mut self, offset: Vec3) {
        self.pos += offset;
        self.target += offset;
    }
    pub fn set_target(&mut self, target: Vec3) {
        let a = (self.target - self.pos).sgn();
        let b = (target - self.pos).sgn();
        let r = Mat4::rotate_from_to(a, b);

        self.target = target;
        self.up = (r * Vec4::from_vec3(self.up, 1.0)).xyz();
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        let a = (self.pos - self.target).sgn();
        let b = (pos - self.target).sgn();
        let r = Mat4::rotate_from_to(a, b);

        self.pos = pos;
        self.up = (r * Vec4::from_vec3(self.up, 1.0)).xyz();
    }
    pub fn set_up(&mut self, up: Vec3) {
        let r = Mat4::rotate_from_to(self.up, up);
        let d = (r * Vec4::from_vec3(self.target_to_pos(), 1.0)).xyz();

        self.up = up;
        self.pos = self.target + d;
    }
    pub fn pitch(&self) -> f32 {
        let v = self.v();
        v.y.asin()
    }
    pub fn add_pitch(&mut self, delta: f32) {
        let r = self.r();
        let v = self.target - self.pos;
        let u = self.u();
        let m = Mat4::rotate(r, -delta);

        let v = (m * Vec4::from_vec3(v, 0.0)).xyz();
        let u = (m * Vec4::from_vec3(u, 0.0)).xyz();
        self.pos = self.target - v;
        self.up = u;
    }
    pub fn cos_pitch(&self) -> f32 {
        let v = self.v();
        1.0 - v.y * v.y
    }
    pub fn yaw(&self) -> f32 {
        let f = self.f().sgn();
        (-f.z).acos() * f.x.signum()
    }
    pub fn roll(&self) -> f32 {
        let r = Vec3::new(0.0, 1.0, 0.0).cross(self.v()).sgn();

        let u = r.cross(-self.v()).sgn();

        u.dot(self.up).min(1.0).max(-1.0).acos() * r.dot(self.up).signum()
    }
    pub fn add_roll(&mut self, diff: f32) {
        self.up = (Mat4::rotate(self.v(), diff) * Vec4::from_vec3(self.up, 0.0)).xyz()
    }
    pub fn view_mat(&self) -> Mat4 {
        Mat4::from_base(self.r(), self.u(), -self.v()).transpose() * Mat4::offset(-self.pos)
    }
    pub fn inv_view_mat(&self) -> Mat4 {
        Mat4::offset(self.pos) * Mat4::from_base(self.r(), self.u(), -self.v())
    }
    pub fn rotate_z(&mut self, angle: f32) {
        self.set_target(self.pos + self.pos_to_target().rotate_z(angle));
    }
    pub fn rotate_x(&mut self, angle: f32) {
        self.set_target(self.pos + self.pos_to_target().rotate_x(angle));
    }
    pub fn rotate_y(&mut self, angle: f32) {
        self.set_target(self.pos + self.pos_to_target().rotate_y(angle));
    }
    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        self.set_target(self.pos + self.pos_to_target().rotate(axis, angle));
    }
    pub fn snap_view(&mut self, sensitivity: f32) {
        let d = self.target_to_pos();
        let snapped = d.snap_if_close(sensitivity, Vec3::base_directions());
        self.set_pos(self.target + snapped);

        let snapped = self.up.snap_if_close(sensitivity, Vec3::base_directions());
        self.set_up(snapped);
    }
    pub fn look_at(&mut self, pos: Vec3, target: Vec3, up: Vec3) {
        self.pos = pos;
        self.target = target;
        self.up = -up.cross(self.v()).cross(self.v()).sgn();
    }
    pub fn cancel_roll(&mut self, allow_upside_down: bool) {
        let roll = self.roll();
        let target = if allow_upside_down && roll < -PI * 0.9 {
            -PI
        } else if allow_upside_down && roll > PI * 0.9 {
            PI
        } else {
            0.0
        };

        self.add_roll(target - roll);
    }
}

#[cfg_attr(feature = "serializable", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct CameraParameters {
    pub spatial: CameraSpatialParams,
    pub fov: f32,
    pub texel_scale: f32,
    pub znear: f32,
    pub zfar: f32,
    pub projection: Projection,
}

impl CameraParameters {
    pub fn l(&self) -> Vec3 {
        self.spatial.l()
    }
    pub fn r(&self) -> Vec3 {
        self.spatial.r()
    }
    pub fn u(&self) -> Vec3 {
        self.spatial.u()
    }
    pub fn v(&self) -> Vec3 {
        self.spatial.v()
    }
    pub fn pos_to_target(&self) -> Vec3 {
        self.spatial.pos_to_target()
    }
    pub fn target_to_pos(&self) -> Vec3 {
        self.spatial.target_to_pos()
    }
    pub fn move_to(&mut self, pos: Vec3) {
        self.spatial.move_to(pos)
    }
    pub fn move_by(&mut self, offset: Vec3) {
        self.spatial.move_by(offset)
    }
    pub fn set_target(&mut self, target: Vec3) {
        self.spatial.set_target(target)
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.spatial.set_pos(pos)
    }
    pub fn set_up(&mut self, up: Vec3) {
        self.spatial.set_up(up)
    }
    pub fn view_mat(&self) -> Mat4 {
        self.spatial.view_mat()
    }
    pub fn inv_view_mat(&self) -> Mat4 {
        self.spatial.inv_view_mat()
    }
    pub fn rotate_z(&mut self, angle: f32) {
        self.spatial.rotate_z(angle)
    }
    pub fn rotate_x(&mut self, angle: f32) {
        self.spatial.rotate_x(angle)
    }
    pub fn rotate_y(&mut self, angle: f32) {
        self.spatial.rotate_y(angle)
    }
    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        self.spatial.rotate(axis, angle)
    }
    pub fn look_at(&mut self, pos: Vec3, target: Vec3, up: Vec3) {
        self.spatial.look_at(pos, target, up)
    }
    pub fn cancel_roll(&mut self, allow_upside_down: bool) {
        self.spatial.cancel_roll(allow_upside_down)
    }
}
