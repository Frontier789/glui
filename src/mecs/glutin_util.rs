use super::message::*;
use tools::*;

pub type GlutinKey = glutin::event::VirtualKeyCode;
pub type GlutinButton = glutin::event::MouseButton;
pub type GlutinWindowEvent<'a> = glutin::event::WindowEvent<'a>;
pub type GlutinDeviceEvent = glutin::event::DeviceEvent;
pub type GlutinScrollDelta = glutin::event::MouseScrollDelta;
pub type GlutinElementState = glutin::event::ElementState;

pub enum GlutinEvent<'a> {
    WindowEvent(GlutinWindowEvent<'a>),
    DeviceEvent(GlutinDeviceEvent),
}

pub(super) type GlutinGLContext = glutin::Context<glutin::PossiblyCurrent>;
pub(super) type GlutinControlFlow = glutin::event_loop::ControlFlow;
pub(super) type GlutinGLContextNC = glutin::Context<glutin::NotCurrent>;
pub(super) type GlutinGLWindow =
    glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
pub(super) type GlutinGLWindowNC =
    glutin::ContextWrapper<glutin::NotCurrent, glutin::window::Window>;
pub(super) type GlutinEventLoop = glutin::event_loop::EventLoop<AnnotatedMessage>;
pub(super) type GlutinEventLoopProxy = glutin::event_loop::EventLoopProxy<AnnotatedMessage>;

pub(super) fn prepare_gl<F>(bgcolor: Vec3, fnloader: F)
where
    F: FnMut(&'static str) -> *const std::ffi::c_void,
{
    unsafe {
        gl::load_with(fnloader);
        gl::ClearColor(bgcolor.x, bgcolor.y, bgcolor.z, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::PRIMITIVE_RESTART_FIXED_INDEX);
        gl::Enable(gl::CULL_FACE);
        gl::DepthFunc(gl::LEQUAL);
    }
}

impl From<glutin::dpi::PhysicalSize<u32>> for Vec2 {
    fn from(s: glutin::dpi::PhysicalSize<u32>) -> Vec2 {
        Vec2::new(s.width as f32, s.height as f32)
    }
}

impl From<glutin::dpi::PhysicalPosition<f64>> for Vec2 {
    fn from(p: glutin::dpi::PhysicalPosition<f64>) -> Vec2 {
        Vec2::new(p.x as f32, p.y as f32)
    }
}

impl From<glutin::dpi::PhysicalPosition<i32>> for Vec2 {
    fn from(p: glutin::dpi::PhysicalPosition<i32>) -> Vec2 {
        Vec2::new(p.x as f32, p.y as f32)
    }
}

impl From<&glutin::dpi::PhysicalSize<u32>> for Vec2 {
    fn from(s: &glutin::dpi::PhysicalSize<u32>) -> Vec2 {
        Vec2::new(s.width as f32, s.height as f32)
    }
}

impl From<&glutin::dpi::PhysicalPosition<f64>> for Vec2 {
    fn from(p: &glutin::dpi::PhysicalPosition<f64>) -> Vec2 {
        Vec2::new(p.x as f32, p.y as f32)
    }
}

impl From<&glutin::dpi::PhysicalPosition<i32>> for Vec2 {
    fn from(p: &glutin::dpi::PhysicalPosition<i32>) -> Vec2 {
        Vec2::new(p.x as f32, p.y as f32)
    }
}
