#![allow(dead_code)]
#![allow(unused_imports)]
extern crate gl;
extern crate glui;
extern crate glutin;
extern crate image;

#[macro_use]
mod gui;
mod tools;

use std::time::Instant;
use gl::types::*;
use tools::*;
use tools::camera::Camera;

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

struct DrawObject {
    pts: Vec<Vec2>,
    clr: Vec3,
}

struct DrawBuilder {
    objects: Vec<DrawObject>,
}

impl DrawBuilder {
    pub fn new() -> DrawBuilder {
        DrawBuilder {
            objects: Vec::new(),
        }
    }
    
    pub fn add_rectangle(&mut self, left: f32, up: f32, right: f32, down: f32, clr: Vec3) {
        self.objects.push(DrawObject {
            pts: vec![
                Vec2::new(left, up),
                Vec2::new(right, up),
                Vec2::new(right, down),
                
                Vec2::new(left, up),
                Vec2::new(right, down),
                Vec2::new(left, down),
            ],
            clr,
        })
    }
    
    pub fn to_render_sequence(self) -> RenderSequence {
        let pbuf = Buffer::from_vec(
            self.objects
                .iter()
                .map(|o| o.pts.clone())
                .flatten()
                .collect(),
        );
        
        let cbuf = Buffer::from_vec(
            self.objects
                .iter()
                .map(|o| vec![o.clr; 6])
                .flatten()
                .collect(),
        );
        
        let n = pbuf.len();
        
        let mut vao = VertexArray::new();
        vao.attrib_buffer(0, &pbuf);
        vao.attrib_buffer(1, &cbuf);
        
        let cmd = RenderCommand {
            vao,
            mode: DrawMode::Triangles,
            first: 0,
            count: n
        };
        
        let mut shader = DrawShader::compile(
            "#version 420 core
            
            layout(location = 0) in vec2 pos;
            layout(location = 1) in vec3 clr;
            
            uniform mat4 proj;
            
            out vec3 va_clr;
            
            void main()
            {
                gl_Position = proj * vec4(pos, 0, 1);
                
                va_clr = clr;
            }",
            "#version 420 core
            
            in vec3 va_clr;
            
            out vec4 color;
            
            void main()
            {
                color = vec4(va_clr, 1);
            }").unwrap();
            
            shader.set_uniform("proj", Mat4::ortho(0.0,768.0,1024.0,0.0,1.0,-1.0));
        
        RenderSequence {
            buffers: vec![pbuf.as_base_type(), cbuf.as_base_type()],
            commands: vec![cmd],
            shader
        }
    }
}

struct RenderSequence {
    pub buffers: Vec<Buffer<f32>>,
    pub commands: Vec<RenderCommand>,
    pub shader: DrawShader,
}

impl RenderSequence {
    pub fn execute(&self) {
        for cmd in &self.commands {
            cmd.execute(&self.shader);
        }
    }
}

struct RenderCommand {
    pub vao: VertexArray,
    pub mode: DrawMode,
    pub first: usize,
    pub count: usize,
}

impl RenderCommand {
    fn execute(&self, shader: &DrawShader) {
        self.vao.bind();
        shader.prepare_draw();
        unsafe {
            gl::DrawArrays(
                self.mode.into(),
                self.first as GLint,
                self.count as GLsizei
            );
        }
    }
}

trait Widget {
    fn on_draw_build(&self, _h:f32, _builder: &mut DrawBuilder) {}
    
    fn size(&self) -> Vec2;
}

struct GuiContext {
    draw_builder: DrawBuilder,
    hstack: Vec<f32>
}

impl GuiContext {
    pub fn new() -> GuiContext {
        GuiContext {
            draw_builder: DrawBuilder::new(),
            hstack: vec![0.0]
        }
    }
    
    fn parse_start<T>(&mut self, w: &T) -> bool
    where
        T: Widget,
    {
        let h = *self.hstack.last().unwrap();
        self.hstack.push(h);
        
        w.on_draw_build(h, &mut self.draw_builder);
        true
    }
    fn parse_end<T>(&mut self, w: &T)
    where
        T: Widget,
    {
        self.hstack.pop();
        
        let h = self.hstack.pop().unwrap() + w.size().y + 10.0;
        
        self.hstack.push(h);
    }
}

struct Layout {
    height: usize,
}

impl Widget for Layout {
    fn on_draw_build(&self, h:f32, builder: &mut DrawBuilder) {
        builder.add_rectangle(0.0, h, 100.0, h + self.height as f32, Vec3::new(0.1,0.1,0.1));
    }
    
    fn size(&self) -> Vec2 {
        Vec2::new(0.0, self.height as f32)
    }
}

struct Button {
    text: String,
}

impl Widget for Button {
    fn on_draw_build(&self, h:f32, builder: &mut DrawBuilder) {
        println!("A button with text \"{}\" is created at height {}", self.text, h);
        
        builder.add_rectangle(0.0, h, 50.0, h + 50.0, Vec3::new(0.8,0.1,0.1));
    }
    
    fn size(&self) -> Vec2 {
        Vec2::new(50.0, 50.0)
    }
}

#[glui::builder]
fn tagged_function(context: &mut GuiContext, data: i32) {
    Layout {
        height: 400,
        children: {
            for i in 1..6 {
                Button {
                    text: format!("{}", i),
                };
            }
        },
    };
    Layout {
        height: 400,
        children: {
            Button {
                text: format!("A_{}", data),
            };
            Button {
                text: format!("B_{}", data),
            };
        },
    };
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let gl_window = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    gl_window
        .window()
        .set_cursor_icon(glutin::window::CursorIcon::Default);
        
        
    let gl_window = unsafe { gl_window.make_current().unwrap() };
    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        let s = gl::GetString(gl::VERSION);
        puts(s as *const i8);
        // gl::Enable(gl::DEPTH_TEST);
        // gl::DepthFunc(gl::LEQUAL);
    }
    
    let mut cont = GuiContext::new();
    
    tagged_function(&mut cont, 42);
    
    let seq = cont.draw_builder.to_render_sequence();
    
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
                    gl::ClearColor(0.5,0.5,0.5, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                seq.execute();
                gl_window.swap_buffers().unwrap();
            }
            glutin::event::Event::MainEventsCleared => {
                gl_window.window().request_redraw();
            }
            _ => (),
        }
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
