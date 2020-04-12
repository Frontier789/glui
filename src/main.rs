// #![allow(dead_code)]
// #![allow(unused_imports)]
// extern crate gl;
// extern crate glui;
// extern crate glutin;
// extern crate image;

// #[macro_use]
// mod gui;
// mod mecs;
// mod tools;

// use gl::types::*;
// use std::any::Any;
// use std::ops::Index;
// use std::time::Instant;
// use tools::camera::Camera;
// use tools::*;

// use mecs::*;
// use gui::*;

// use std::os::raw::c_char;
// extern "C" {
//     fn puts(s: *const c_char);
// }

// // TODOs
// // use slices instead of vecs in buffer
// // put vecs in one file and use macros
// // add debug asserts
// //
// // Caching
// // Scrolling
// // Scroller element
// // toggle button element
// // Selector element
// // Touch handling
// // Tab and enter for active element
// // 

// #[derive(Clone,Debug,PartialEq)]
// struct Data {
//     goat: i32,
//     shown: String,
// }

// #[glui::builder(Data)]
// fn experimental(param: Data) {
//     FixedPanel {
//         size: 50.0,
//         children: {
//             Padding {
//                 left: 20.0,
//                 children: {
//                     Text {
//                         text: param.shown.clone(),
//                         font: "arial".to_owned(),
//                         align: font::align(HAlign::Left, VAlign::Center),
//                         color: Vec4::RED,
//                     };
//                 },
//             };
//             GridLayout {
//                 col_widths: vec![1.0; 5],
//                 row_heights: vec![1.0; 5],
//                 children: {
//                     for i in 0..25 {
//                         let text = format!("{}", i + param.goat);
//                         mybutton(text);
//                     }
//                 },
//             };
//         },
//     };
// }

// #[glui::builder(Data)]
// pub fn mybutton(text: String) {
//     Padding {
//         left: 10.0,
//         right: 10.0,
//         top: 10.0,
//         bottom: 10.0,
//         children: {
//             Button {
//                 text: text.clone(),
//                 callback: |data| {data.shown = data.shown.clone() + " " + &text;},
//                 background: ButtonBckg::RoundRect(Vec4::grey(0.1), 6.0),
//             };
//         },
//     };
// }

// // 
// // Model:
// // Entity     - Has id, can hold components, can be queried for components
// // Component  - Holds data like position/velocity/bounding info
// // System     - Updates Entities, can send Messages, can receive Messages, unique
// // Message    - Can be sent, can be received, user defined
// // Actor      - Can send Messages, can receive Messages, unique
// // Singleton?
// // 

// fn main() {
//     let mut win = GlutinWindow::new(Vec2::new(640.0, 480.0), "Hello gui", Vec3::grey(0.0));
//     let render_target = win.render_target();
//     let mut cont = GuiContext::new(render_target, true, experimental, Data{goat: 0, shown: "".to_owned()});
//     cont.init_gl_res();
//     cont.rebuild_gui();
//     win.world.add_entity(Box::new(cont));
//     win.run(GuiWinProps::quick_tester());
// }
#![allow(dead_code)]

extern crate glutin;

mod mecs;
mod tools;
use mecs::*;

#[derive(Component,Debug)]
struct Printable {
    text: String,
}

#[derive(Debug)]
struct Print {}
impl Message for Print {}

#[derive(Debug)]
struct PrintTimes(u32);
impl Message for PrintTimes {}

struct Printer {
    
}

impl System for Printer {
    fn receive(&mut self, msg: Box<dyn Message>, world: &mut StaticWorld) {
        match_downcast!( msg {
            _ : Print => {
                let printables = world.entities_with_component::<Printable>();
                println!("Print msg for {} entities", printables.len());
                
                for p in printables {
                    world.with_component(p, |c: &mut Printable| {
                        println!("Text: {}", c.text);
                    });
                }
            },
            PrintTimes(n) if n > 0 => {
                world.send(SystemId::of::<Printer>(), Print{});
                world.send(SystemId::of::<Printer>(), PrintTimes(n-1));
            },
            PrintTimes(0) => {},
            _ => panic!(format!("Printer received unknown msg: {:?}",msg)),
        });
        
    }
}

use std::thread;
use std::time;

fn main() {
    let mut w: World = World::new();
    w.add_system(Printer{});
    
    let sw = w.as_static_mut();
    
    let e = sw.entity();
    sw.add_component(e, Printable{text: "hy".to_owned()});
    sw.with_component(e,|c: &mut Printable| { c.text += "_2"; });
    
    
    let channel = w.channel();
    let t = {
        let channel = channel.clone();
        thread::spawn(move ||{
        
        for _ in 0..5 {
            thread::sleep(time::Duration::from_millis(1000));
            
            channel.send(SystemId::of::<Printer>(), Print{}).unwrap();
        }
        channel.send(MessageTarget::None, message::Exit{}).unwrap();
    })};
        
    channel.send(SystemId::of::<Printer>(), PrintTimes(4)).unwrap();
    
    w.run();
    
    t.join().unwrap();
}
