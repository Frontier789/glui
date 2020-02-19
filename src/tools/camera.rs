use super::Vec2;
use super::Vec3;
use super::Mat4;

pub trait Camera {
    fn view(&self) -> Mat4;
    fn projection(&self) -> Mat4;
    fn set_canvas_size(&mut self, size: Vec2);
}

pub struct Camera3D {
    pos: Vec3,
    target: Vec3,
    canvas_size: Vec2,
    znear: f32,
    zfar: f32,
    fov: f32,
}

impl Camera3D {
    pub fn with_params(
        pos: Vec3,
        target: Vec3,
        canvas_size: Vec2,
        znear: f32,
        zfar: f32,
        fov: f32,
    ) -> Camera3D {
        Camera3D {
            pos: pos,
            target: target,
            canvas_size: canvas_size,
            znear: znear,
            zfar: zfar,
            fov: fov,
        }
    }

    pub fn l(&self) -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0).cross(self.v()).sgn()
    }
    pub fn u(&self) -> Vec3 {
        self.v().cross(self.l()).sgn()
    }
    pub fn v(&self) -> Vec3 {
        (self.target - self.pos).sgn()
    }
    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }
}

impl Camera for Camera3D {
    fn view(&self) -> Mat4 {
        Mat4::from_base(-self.l(), self.u(), -self.v()).transpose() * Mat4::offset(-self.pos())
    }
    fn projection(&self) -> Mat4 {
        Mat4::perspective(self.fov, self.canvas_size.aspect(), self.znear, self.zfar)
    }
    fn set_canvas_size(&mut self, size: Vec2) {
        self.canvas_size = size;
    }
}