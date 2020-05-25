use super::glutin_util::*;
use super::render_target::*;
use std::time::Duration;
use tools::*;

pub struct GlutinContextData {
    pub event_loop: GlutinEventLoop,
    pub gl_context: GlutinGLContext,
    pub bgcolor: Vec3,
    pub update_interval: Duration,
}

impl GlutinContextData {
    fn create_context(
        event_loop: &GlutinEventLoop,
        bgcolor: Vec3,
    ) -> Result<GlutinGLContext, glutin::CreationError> {
        let cb = glutin::ContextBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl(glutin::GlRequest::Latest);

        let gl_context = build_context(cb, event_loop)?;
        let gl_context = unsafe { gl_context.make_current().unwrap() };

        prepare_gl(bgcolor, |symbol| {
            gl_context.get_proc_address(symbol) as *const _
        });

        Ok(gl_context)
    }

    pub fn new(bgcolor: Vec3, update_interval: Duration) -> Result<Self, glutin::CreationError> {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let gl_context = Self::create_context(&event_loop, bgcolor)?;

        Ok(GlutinContextData {
            event_loop,
            gl_context,
            bgcolor,
            update_interval,
        })
    }
    pub fn render_target(&self) -> WindowInfo {
        WindowInfo {
            size: Vec2::new(1.0, 1.0),
            gui_scale: 1.0,
            ..Default::default()
        }
        .fill_from_context()
    }
    pub fn event_loop_proxy(&self) -> GlutinEventLoopProxy {
        self.event_loop.create_proxy()
    }
}

#[cfg(target_os = "linux")]
fn build_context_surfaceless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    cb.build_surfaceless(&el)
}

fn build_context_headless<T1: glutin::ContextCurrentState>(
    cb: glutin::ContextBuilder<T1>,
    el: &GlutinEventLoop,
) -> Result<GlutinGLContextNC, glutin::CreationError> {
    let size_one = glutin::dpi::PhysicalSize::new(1, 1);
    cb.build_headless(&el, size_one)
}

#[cfg(target_os = "linux")]
fn build_context_osmesa<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    let size_one = PhysicalSize::new(1, 1);
    cb.build_osmesa(size_one)
}

#[cfg(target_os = "linux")]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &GlutinEventLoop,
) -> Result<Context<NotCurrent>, glutin::CreationError> {
    let el = EventLoop::new();

    match build_context_surfaceless(cb.clone(), &el) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => {
            println!("Context creation using surfaceless failed: {}", err);
        }
    };

    match build_context_headless(cb.clone(), el) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => {
            println!("Context creation using headless failed: {}", err);
        }
    };

    build_context_osmesa(cb)?
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: glutin::ContextCurrentState>(
    cb: glutin::ContextBuilder<T1>,
    el: &GlutinEventLoop,
) -> Result<GlutinGLContextNC, glutin::CreationError> {
    build_context_headless(cb, el)
}
