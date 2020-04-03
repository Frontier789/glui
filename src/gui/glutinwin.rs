use super::*;
use crate::ecs::*;

pub type GlutinKey = glutin::event::VirtualKeyCode;
pub type GlutinButton = glutin::event::MouseButton;
pub type GlutinEvent<'a> = glutin::event::WindowEvent<'a>;

type GlutinGLWindow = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
type GlutinGLWindowNC = glutin::ContextWrapper<glutin::NotCurrent, glutin::window::Window>;
type GlutinEventLoop = glutin::event_loop::EventLoop<()>;

pub struct GuiWinProps {
    pub quit_on_esc: bool,
    pub quit_on_focus_lost: bool,
}

impl GuiWinProps {
    pub fn quick_tester() -> GuiWinProps {
        GuiWinProps {
            quit_on_esc: true,
            quit_on_focus_lost: true,
        }
    }
    
    pub fn tester() -> GuiWinProps {
        GuiWinProps {
            quit_on_esc: true,
            quit_on_focus_lost: false,
        }
    }
}

pub struct GlutinWindow {
    event_loop: glutin::event_loop::EventLoop<()>,
    gl_window: GlutinGLWindow,
    bgcolor: Vec3,
    pub world: World,
}

impl GlutinWindow {
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
    
    pub fn create_window(size: Vec2, title: &str, event_loop: &GlutinEventLoop) -> GlutinGLWindowNC {
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
        let event_loop = glutin::event_loop::EventLoop::new();
        
        let gl_window = Self::prepare_gl(Self::create_window(size, title, &event_loop), bgcolor);
        
        gl_window.window().set_visible(true);
        
        GlutinWindow {
            event_loop,
            gl_window,
            bgcolor,
            world: World {
                entities: vec![]
            },
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

    pub fn run(self, props: GuiWinProps)
    {
        let event_loop = self.event_loop;
        let gl_window = self.gl_window;
        let bgcolor = self.bgcolor;
        let mut world = self.world;
        
        event_loop.run(move |event, _, control_flow| {
            *control_flow = glutin::event_loop::ControlFlow::Wait;
    
            match event {
                glutin::event::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                        }
                        glutin::event::WindowEvent::Focused(false) => {
                            if props.quit_on_focus_lost {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                            }
                        },
                        glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                            match input.virtual_keycode {
                                None => (),
                                Some(glutin::event::VirtualKeyCode::Escape) => if props.quit_on_esc {
                                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                                },
                                _ => {},
                            }
                        },
                        glutin::event::WindowEvent::Resized(size) => {
                            unsafe {
                                gl::Viewport(0, 0, size.width as i32, size.height as i32);
                            }
                        },
                        _ => (),
                    }
                    
                    world.handle_event(event);
                },
                glutin::event::Event::RedrawRequested(..) => {
                    unsafe {
                        gl::ClearColor(bgcolor.x, bgcolor.y, bgcolor.z, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }
                    
                    world.render();
                    
                    gl_window.swap_buffers().unwrap();
                }
                glutin::event::Event::RedrawEventsCleared => {
                    gl_window.window().request_redraw();
                }
                _ => (),
            }
        });
    }
}