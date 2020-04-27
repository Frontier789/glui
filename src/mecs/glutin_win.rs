use tools::*;
use super::glutin_util::*;
use super::render_target::*;

pub struct GlutinWindowData {
    event_loop: GlutinEventLoop,
    gl_window: GlutinGLWindow,
    bgcolor: Vec3,
}

impl GlutinWindowData {
    fn initialize(gl_window: GlutinGLWindowNC, bgcolor: Vec3) -> GlutinGLWindow {
        gl_window
            .window()
            .set_cursor_icon(glutin::window::CursorIcon::Default);
        let gl_window = unsafe { gl_window.make_current().unwrap() };
        
        prepare_gl(bgcolor, |symbol| gl_window.get_proc_address(symbol) as *const _);
        
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
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl(glutin::GlRequest::Latest)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
    }
    pub fn new(size: Vec2, title: &str, bgcolor: Vec3) -> Self {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let gl_window = Self::initialize(Self::create_window(size, title, &event_loop), bgcolor);
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
