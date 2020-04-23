use super::*;
use tools::*;

pub type GlutinKey = glutin::event::VirtualKeyCode;
pub type GlutinButton = glutin::event::MouseButton;
pub type GlutinEvent<'a> = glutin::event::WindowEvent<'a>;

type GlutinGLWindow = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
type GlutinGLWindowNC = glutin::ContextWrapper<glutin::NotCurrent, glutin::window::Window>;
type GlutinEventLoop = glutin::event_loop::EventLoop<AnnotatedMessage>;
type GlutinEventLoopProxy = glutin::event_loop::EventLoopProxy<AnnotatedMessage>;

pub struct GlutinWindowData {
    event_loop: GlutinEventLoop,
    gl_window: GlutinGLWindow,
    bgcolor: Vec3,
}

impl GlutinWindowData {
    fn prepare_gl(gl_window: GlutinGLWindowNC, bgcolor: Vec3) -> GlutinGLWindow {
        gl_window
            .window()
            .set_cursor_icon(glutin::window::CursorIcon::Default);
        let gl_window = unsafe { gl_window.make_current().unwrap() };
        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::ClearColor(bgcolor.x, bgcolor.y, bgcolor.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
        gl_window.swap_buffers().unwrap();
        gl_window
    }

    fn create_window(size: Vec2, title: &str, event_loop: &GlutinEventLoop) -> GlutinGLWindowNC {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(size.x, size.y))
            .with_visible(false);
        glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
    }
    pub fn new(size: Vec2, title: &str, bgcolor: Vec3) -> Self {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let gl_window = Self::prepare_gl(Self::create_window(size, title, &event_loop), bgcolor);
        gl_window.window().set_visible(true);
        GlutinWindowData {
            event_loop,
            gl_window,
            bgcolor,
        }
    }
    pub fn render_target(&self) -> RenderTarget {
        let win = self.gl_window.window();
        RenderTarget {
            size: win.inner_size().into(),
            gui_scale: win.scale_factor() as f32,
            ..Default::default()
        }
        .fill_from_context()
    }
    pub fn event_loop_proxy(&self) -> GlutinEventLoopProxy {
        self.event_loop.create_proxy()
    }
    pub fn unpack(self) -> (GlutinEventLoop,GlutinGLWindow,Vec3) {
        (self.event_loop, self.gl_window, self.bgcolor)
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
