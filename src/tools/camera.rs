use graphics::{DrawResources, RenderSequence};
use mecs::{GlutinDeviceEvent, GlutinWindowEvent};
use std::time::Duration;
use tools::camera_parameters::Projection;
use tools::{
    CameraController, CameraParameters, FrameBufferAttachment, Framebuffer, Mat4, Texture,
};

#[derive(Debug)]
pub struct Camera {
    fbo: Option<Framebuffer>,
    pub params: CameraParameters,
    controller: Box<dyn CameraController>,
}

fn aspect(size: (usize, usize)) -> f32 {
    size.0 as f32 / size.1 as f32
}

// TODO: handle orthogonal projection
impl Camera {
    pub fn set_controller<T>(&mut self, controller: T)
    where
        T: CameraController,
    {
        self.controller = Box::new(controller);
        self.controller.init(&mut self.params);
    }
    pub fn controller<T>(&self) -> Option<&T>
    where
        T: CameraController,
    {
        self.controller.downcast_ref()
    }
    pub fn controller_mut<T>(&mut self) -> Option<&mut T>
    where
        T: CameraController,
    {
        self.controller.downcast_mut()
    }
    pub fn update(&mut self, delta: Duration) {
        self.controller.update(&mut self.params, delta);
    }
    pub fn on_window_event(&mut self, event: &GlutinWindowEvent) -> bool {
        self.controller.on_window_event(&mut self.params, event)
    }
    pub fn on_device_event(&mut self, event: &GlutinDeviceEvent) -> bool {
        self.controller.on_device_event(&mut self.params, event)
    }
    pub fn new<C>(controller: C) -> Camera
    where
        C: CameraController,
    {
        Camera {
            fbo: None,
            params: CameraParameters {
                spatial: Default::default(),
                projection: Projection::Perspective,
                fov: std::f32::consts::PI * 90.0 / 180.0,
                texel_scale: 1.0,
                znear: 0.1,
                zfar: 1000.0,
            },
            controller: Box::new(controller),
        }
    }

    pub fn proj(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective(
            self.params.fov,
            aspect_ratio,
            self.params.znear,
            self.params.zfar,
        )
    }
    pub fn inv_proj(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::inv_perspective(
            self.params.fov,
            aspect_ratio,
            self.params.znear,
            self.params.zfar,
        )
    }

    pub fn view(&self) -> Mat4 {
        self.params.view_mat()
    }
    pub fn inv_view(&self) -> Mat4 {
        self.params.inv_view_mat()
    }

    fn ensure_fbo(&mut self) -> &mut Framebuffer {
        if None == self.fbo {
            self.fbo = Some(Framebuffer::new())
        }

        match self.fbo {
            Some(ref mut fbo) => fbo,
            None => panic!("Should not happen"),
        }
    }

    pub fn render_to_texture<T>(
        &mut self,
        tex: &mut T,
        seq: &mut RenderSequence,
        res: &mut DrawResources,
    ) where
        T: Texture,
    {
        let fbo = self.ensure_fbo();

        fbo.attach_texture(FrameBufferAttachment::Color(0), tex);
        fbo.set_draw_targets(vec![FrameBufferAttachment::Color(0)]);
        fbo.bind();
        unsafe {
            gl::Disable(gl::DEPTH_TEST); // TODO: add depth support
        }
        res.projection_matrix = self.proj(aspect(tex.size_2d()));
        res.inv_projection_matrix = self.inv_proj(aspect(tex.size_2d()));
        res.view_matrix = self.view();
        res.inv_view_matrix = self.inv_view();
        seq.execute(res);
    }

    pub fn render_to_screen(&self, seq: &RenderSequence, res: &mut DrawResources) {
        Framebuffer::bind_def_framebuffer(Some(res.window_info.size));
        res.projection_matrix = self.proj(res.window_info.size.aspect());
        res.inv_projection_matrix = self.inv_proj(res.window_info.size.aspect());
        res.view_matrix = self.view();
        res.inv_view_matrix = self.inv_view();
        seq.execute(res);
    }
}
