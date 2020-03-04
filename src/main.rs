#![allow(dead_code)]
#![allow(unused_imports)]
extern crate gl;
extern crate glui;
extern crate glutin;
extern crate image;

#[macro_use]
extern crate downcast_rs;

#[macro_use]
mod gui;
mod tools;

use gl::types::*;
use std::any::Any;
use std::ops::Index;
use std::time::Instant;
use tools::camera::Camera;
use tools::*;

use gui::*;

use std::os::raw::c_char;
extern "C" {
    fn puts(s: *const c_char);
}

// TODOs
// use slices instead of vecs in buffer
// put vecs in one file and use macros
// add debug asserts
//
// change f32 to T: Into<f32> or smth
//
// gui
// font-render
// gui layout
// renderer
// gui renderer
// render target
//
// gui hierarchy - good
// owned geometry and 1 by 1 rendering: flexible and easy to implement might be tedious?
// shared renderer(s): difficult to generalize, ownership?, can optimize MUCH
//
//

////////////////////////////// Vertical layout //////////////////////////////
struct VertLayout {
    size: Vec2px,
    padding: Vec2px,
}

impl Widget for VertLayout {
    fn start_layout(&mut self) {
        self.size = Vec2px::zero();
    }
    
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.size.x = self_constraint.max_size.x;
    }
    
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: Vec2px::new(self.size.x, std::f32::INFINITY),
        })
    }

    fn place_child(&mut self, child_size: Vec2px) -> Vec2px {
        let y = self.size.y;
        self.size.y += child_size.y + self.padding.y;
        Vec2px::new(0.0, y)
    }

    fn size(&self) -> Vec2px {
        self.size - Vec2px::new(0.0, self.padding.y)
    }
}

////////////////////////////// Button //////////////////////////////
struct Button {
    text: String,
}

impl Widget for Button {
    fn size(&self) -> Vec2px {
        Vec2px::new(100.0, 100.0)
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        builder.add_rectangle(
            Vec2::zero(),
            self.size().to_pixels(1.0),
            Vec3::new(0.3, 0.0, 0.0),
        );
    }
}

#[glui::builder]
fn tagged_function(parser: &mut WidgetTreeToList, data: i32)
{
    VertLayout {
        padding: Vec2px::zero(),
        size: Vec2px::zero(),
        children: {
            VertLayout {
                padding: Vec2px::new(0.0, 2.0),
                size: Vec2px::zero(),
                children: {
                    for i in 1..6 {
                        Button {
                            text: format!("{}", i),
                        };
                    }
                    VertLayout {
                        padding: Vec2px::new(0.0, 2.0),
                        size: Vec2px::zero(),
                        children: {
                            Button {
                                text: format!("{}", 42),
                            };
                        },
                    };
                },
            };
            VertLayout {
                padding: Vec2px::new(0.0, 2.0),
                size: Vec2px::zero(),
                children: {
                    Button {
                        text: format!("A_{}", data),
                    };
                    Button {
                        text: format!("B_{}", data),
                    };
                },
            };
        },
    };
}

impl From<glutin::dpi::PhysicalSize<u32>> for Vec2 {
    fn from(s: glutin::dpi::PhysicalSize<u32>) -> Vec2 {
        Vec2::new(s.width as f32, s.height as f32)
    }
}

type GlutinGLWindow = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
type GlutinGLWindowNC = glutin::ContextWrapper<glutin::NotCurrent, glutin::window::Window>;
type GlutinEventLoop = glutin::event_loop::EventLoop<()>;

struct GuiWindow {
    event_loop: glutin::event_loop::EventLoop<()>,
    gl_window: GlutinGLWindow,
    bgcolor: Vec3,
}

impl GuiWindow {
    fn prepare_gl(gl_window: GlutinGLWindowNC, bgcolor: Vec3) -> GlutinGLWindow {
        gl_window
            .window()
            .set_cursor_icon(glutin::window::CursorIcon::Default);
        
        let gl_window = unsafe { gl_window.make_current().unwrap() };
        
        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            
            gl::ClearColor(bgcolor.x, bgcolor.y, bgcolor.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        gl_window
    }
    
    fn create_window(size: Vec2, title: String, event_loop: &GlutinEventLoop) -> GlutinGLWindowNC {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(size.x, size.y))
            .with_visible(false);
        glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
    }
    
    pub fn new(size: Vec2, title: String, bgcolor: Vec3) -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();
        
        let gl_window = Self::prepare_gl(Self::create_window(size, title, &event_loop), bgcolor);
        
        gl_window.window().set_visible(true);
        
        GuiWindow {
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

    pub fn run<F>(self, mut draw_handler: F)
    where
        F: 'static + FnMut(),
    {
        let event_loop = self.event_loop;
        let gl_window = self.gl_window;
        let bgcolor = self.bgcolor;
        
        event_loop.run(move |event, _, control_flow| {
            *control_flow = glutin::event_loop::ControlFlow::Wait;
    
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                    glutin::event::WindowEvent::Focused(false) => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                    _ => (),
                },
                glutin::event::Event::RedrawRequested(..) => {
                    unsafe {
                        gl::ClearColor(bgcolor.x, bgcolor.y, bgcolor.z, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }
                    
                    draw_handler();
                    
                    gl_window.swap_buffers().unwrap();
                }
                glutin::event::Event::MainEventsCleared => {
                    gl_window.window().request_redraw();
                }
                _ => (),
            }
        });
    }
}

struct GuiContext {
    render_target: RenderTarget,
    
    render_seq: Option<RenderSequence>,
}

impl GuiContext {
    pub fn new(target: RenderTarget) -> GuiContext {
        GuiContext {
            render_target: target,
            render_seq: None
        }
    }
    
    pub fn build_gui<F,D>(&mut self, parse_fun: F, parse_data: D)
        where F: Fn(&mut WidgetTreeToList, D)
    {
        let mut parser = WidgetTreeToList::new();
        parse_fun(&mut parser, parse_data);

        let mut layout_builder =
            WidgetLayoutBuilder::new(parser.widgets, parser.postorder, parser.child_count);
        
        layout_builder.build(self.render_target.logical_size());
        layout_builder.make_pos_abs(0, Vec2px::origin());
        
        let mut drawer = WidgetDrawBuilder::new();
        
        drawer.build(&layout_builder.widgets, &layout_builder.positions);
        
        self.render_seq = Some(drawer.builder.to_render_sequence(&self.render_target));
    }
    
    pub fn on_render(&self) {
        self.render_seq.as_ref().unwrap().execute();
    }
}

fn main() {
    let win = GuiWindow::new(Vec2::new(1024.0, 768.0), "Hello gui".to_owned(), Vec3::grey(0.2));
    
    let render_target = win.render_target();
    
    println!("rt = {:#?}", render_target);
    
    let mut cont = GuiContext::new(render_target);
    
    cont.build_gui(tagged_function, 5);
    
    
    win.run(move || {
        cont.on_render();
    });
}

// fn main() {
//     let pi = std::f32::consts::PI;
//     let event_loop = glutin::event_loop::EventLoop::new();
//     let window_builder = glutin::window::WindowBuilder::new()
//         .with_title("Hello world!")
//         .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
//     let gl_window = glutin::ContextBuilder::new()
//         .with_vsync(true)
//         .build_windowed(window_builder, &event_loop)
//         .unwrap();

//     gl_window
//         .window()
//         .set_cursor_icon(glutin::window::CursorIcon::Default);

//     let gl_window = unsafe { gl_window.make_current().unwrap() };
//     unsafe {
//         gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
//         let s = gl::GetString(gl::VERSION);
//         puts(s as *const i8);
//         gl::Enable(gl::DEPTH_TEST);
//         gl::DepthFunc(gl::LEQUAL);
//     }
//     let n = 2;
//     let m = 2;

//     let pts = parsurf(
//         |x, y| Vec3::new((x - 0.5) * 2.0, (y - 0.5) * 2.0, 1.0),
//         n,
//         m,
//     );
//     let nrm = parsurf(|_, _| Vec3::new(0.0, 0.0, 1.0), n, m);
//     let tng = parsurf(|_, _| Vec3::new(1.0, 0.0, 0.0), n, m);
//     let tpt = parsurf(|x, y| Vec2::new(x/2.0, y/2.0), n, m);
//     let pbuf = Buffer::from_vec(pts);
//     let tbuf = Buffer::from_vec(tpt);
//     let nbuf = Buffer::from_vec(nrm);
//     let gbuf = Buffer::from_vec(tng);

//     let mut s = DrawShader::compile(
//         "
// #version 420 core

// layout(location = 0) in vec3 position;
// layout(location = 2) in vec2 tpt;

// uniform mat4 model;
// uniform mat4 proj;

// out vec2 texp;

// void main()
// {
//     gl_Position = proj * model * vec4(position, 1);
//     texp = tpt;
// }",
//         "
// #version 420 core

// uniform sampler2D ambientOcclusion;
// uniform sampler2D basecolor;
// uniform sampler2D height;
// uniform sampler2D metallic;
// uniform sampler2D normal;
// uniform sampler2D roughness;

// in vec2 texp;
// out vec4 color;

// void main()
// {
//     color = texture(basecolor, texp) * texture(ambientOcclusion, texp).x;
// }",
//     )
//     .unwrap();

//     let mut vao = VertexArray::new();
//     vao.attrib_buffer(0, &pbuf);
//     vao.attrib_buffer(1, &tbuf);
//     vao.attrib_buffer(2, &nbuf);
//     vao.attrib_buffer(3, &gbuf);

//     let mut model_mat = Mat4::new();
//     let mut rot_mat = Mat4::new();
//     let tex_ambient_occlusion = Texture::from_file("metal_plate/ambientOcclusion.jpg").unwrap();
//     let tex_basecolor = Texture::from_file("metal_plate/basecolor.jpg").unwrap();
//     let tex_height = Texture::from_file("metal_plate/height.png").unwrap();
//     let tex_metallic = Texture::from_file("metal_plate/metallic.jpg").unwrap();
//     let tex_normal = Texture::from_file("metal_plate/normal.jpg").unwrap();
//     let tex_roughness = Texture::from_file("metal_plate/roughness.jpg").unwrap();
//     let mut last_update   = Instant::now();
//     let     shader_clock  = Instant::now();
//     let mut update_shader = true;
//     let mut lpressed      = false;
//     let mut shift_pressed = false;
//     let mut finger_id: u64 = 0;
//     let mut touched = false;
//     let mut last_finger = Vec2::origin();
//     let mut cam = Camera3D::with_params(Vec3::origin(), Vec3::new(0.0,0.0,-1.0), Vec2::new(1024.0,768.0), 0.1, 100.0, pi / 2.0);
//     cam.set_pos(Vec3::new(5.0,14.0,10.0));
//     cam.set_target(Vec3::origin());
//     // println!("r={:?}, u={:?}, v={:?}", cam.r(), cam.u(), cam.v());
//     // return ();
//     println!("view = {}", cam.view());
//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = glutin::event_loop::ControlFlow::Poll;

//         match event {
//             glutin::event::Event::WindowEvent { event, .. } => match event {
//                 glutin::event::WindowEvent::CloseRequested => {
//                     *control_flow = glutin::event_loop::ControlFlow::Exit;
//                 }
//                 glutin::event::WindowEvent::Focused(true) => {
//                     update_shader = true;
//                 }
//                 glutin::event::WindowEvent::KeyboardInput { input, .. } => {
//                     match input.virtual_keycode {
//                         None => (),
//                         Some(key) => {
//                             match key {
//                                 glutin::event::VirtualKeyCode::Escape => {
//                                     *control_flow = glutin::event_loop::ControlFlow::Exit;
//                                 },
//                                 glutin::event::VirtualKeyCode::LShift => {
//                                     shift_pressed = input.state == glutin::event::ElementState::Pressed;
//                                 },
//                                 _ => ()
//                             }
//                         }
//                     }
//                 }
//                 glutin::event::WindowEvent::Touch(t) => {
//                     match t.phase {
//                         glutin::event::TouchPhase::Started => {
//                             if !touched {
//                                 finger_id = t.id;
//                                 touched = true;
//                                 last_finger = Vec2::new(t.location.x as f32,t.location.y as f32);
//                             }
//                         }
//                         glutin::event::TouchPhase::Moved => {
//                             if t.id == finger_id {
//                                 let fin = Vec2::new(t.location.x as f32,t.location.y as f32);
//                                 let delta = fin - last_finger;
//                                 if !shift_pressed {
//                                     rot_mat = Mat4::rotate(Vec3::new(0.0, -1.0, 0.0), delta.x as f32 / 150.0)
//                                         * Mat4::rotate(Vec3::new(-1.0, 0.0, 0.0), delta.y as f32 / 150.0)
//                                         * rot_mat;
//                                 } else {
//                                     rot_mat = Mat4::rotate_z(delta.x as f32 / 150.0) * rot_mat;
//                                 }
//                                 last_finger = fin;
//                             }
//                         }
//                         glutin::event::TouchPhase::Ended => {
//                             if t.id == finger_id {
//                                 touched = false;
//                             }
//                         }
//                         _ => ()
//                     }
//                 }
//                 glutin::event::WindowEvent::MouseInput { button, state, .. } => {
//                     if button == glutin::event::MouseButton::Left {
//                         lpressed = state == glutin::event::ElementState::Pressed;
//                     }
//                 }
//                 glutin::event::WindowEvent::MouseWheel { delta, .. } => match delta {
//                     glutin::event::MouseScrollDelta::LineDelta(_x, y) => {
//                         model_mat = model_mat * Mat4::scale(1.1f32.powf(y));
//                     }
//                     _ => (),
//                 },
//                 _ => (),
//             },
//             glutin::event::Event::DeviceEvent { event, .. } => match event {
//                 glutin::event::DeviceEvent::MouseMotion { delta } => {
//                     if lpressed {
//                         if !shift_pressed {
//                             rot_mat = Mat4::rotate(-cam.u(), delta.0 as f32 / 150.0)
//                                 * Mat4::rotate(cam.l(), delta.1 as f32 / 150.0)
//                                 * rot_mat;
//                         } else {
//                             rot_mat = Mat4::rotate_z(delta.0 as f32 / 150.0) * rot_mat;
//                         }
//                     }
//                 }
//                 _ => (),
//             },
//             glutin::event::Event::RedrawRequested(..) => {
//                 unsafe {
//                     let dt = last_update.elapsed();
//                     last_update += dt;
//                     let secs = shader_clock.elapsed().as_millis() as f32 / 1000.0;
//                     // let mut p = Vec3::pol(13.0, 1.0, secs / 2.0).xzy();
//                     // p.y = num::Float::sin(secs)*5.0;
//                     // cam.set_pos(p);

//                     gl::ClearColor(56.0 / 255.0, 56.0 / 255.0, 56.0 / 255.0, 1.0);
//                     gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
//                     s.prepare_draw();
//                     s.set_uniform("seconds", secs);
//                     s.set_uniform("view", cam.view());
//                     s.set_uniform("cam_dir", cam.v());
//                     for x in 0..20 {
//                     for y in 0..30 {
//                     for z in 0..40 {
//                         if x+y==0 || y+z==0 || x+z==0 || num::abs((x*x+y*y+z*z) as f32 -(num::Float::sin(secs/5.0)*30.0).powi(2)) < num::Float::sin(secs/5.0)*30.0 {
//                             for m in [
//                                 Mat4::identity(),
//                                 Mat4::rotate_x(pi / 2.0),
//                                 Mat4::rotate_x(pi),
//                                 Mat4::rotate_x(-pi / 2.0),
//                                 Mat4::rotate_y(pi / 2.0),
//                                 Mat4::rotate_y(-pi / 2.0),
//                             ]
//                             .iter()
//                             {
//                                 s.set_uniform("model", model_mat * rot_mat * Mat4::offset(Vec3::new((x*2) as f32,(y*2) as f32,(z*2) as f32)) * *m);
//                                 gl::DrawArrays(gl::TRIANGLES, 0, pbuf.len() as i32);
//                             }
//                         }
//                     }
//                     }
//                     }
//                 }
//                 gl_window.swap_buffers().unwrap();
//             }
//             glutin::event::Event::MainEventsCleared => {
//                 gl_window.window().request_redraw();
//             }
//             _ => (),
//         }
//         if update_shader {
//             match std::fs::read_to_string("shaders/vert.glsl") {
//                 Ok(vert_source) => match std::fs::read_to_string("shaders/frag.glsl") {
//                     Ok(frag_source) => match DrawShader::compile(&vert_source, &frag_source) {
//                         Ok(sh) => {
//                             s = sh;
//                             s.set_uniform("proj", cam.projection());
//                             s.set_uniform("ambientOcclusion", &tex_ambient_occlusion);
//                             s.set_uniform("basecolor", &tex_basecolor);
//                             s.set_uniform("height", &tex_height);
//                             s.set_uniform("metallic", &tex_metallic);
//                             s.set_uniform("normal", &tex_normal);
//                             s.set_uniform("roughness", &tex_roughness);
//                             println!("Compilation seccessful");
//                         }
//                         Err(e) => {
//                             println!("Compilation failed, err: {}", e.details());
//                         }
//                     },
//                     Err(_) => {
//                         println!("File shaders/frag.glsl not found");
//                     }
//                 },
//                 Err(_) => {
//                     println!("File shaders/vert.glsl not found");
//                 }
//             }
//             update_shader = false;
//         }
//     });
// }
