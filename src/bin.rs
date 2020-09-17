#![allow(dead_code)]
#![allow(unused_imports)]
extern crate gl;
extern crate glui;
extern crate glui_proc;
extern crate glutin;
extern crate image;

use std::any::Any;
use std::ops::Index;
use std::os::raw::c_char;
use std::time::Instant;

use gl::types::*;

use gui::elements::*;
use gui::widget_parser::StoredCallback;
use gui::*;
use mecs::*;
use tools::*;

#[macro_use]
mod gui;
mod graphics;
mod mecs;
mod tools;

extern "C" {
    fn puts(s: *const c_char);
}

// TODOs
// use slices instead of vecs in buffer
// put vecs in one file and use macros
// add debug asserts
//
// Caching
// Scrolling
// Scroller element
// toggle button element
// Selector element
// Touch handling
// Tab and enter for active element
//

#[derive(Clone, Debug, PartialEq)]
struct Data {
    goat: i32,
    shown: String,
    show_red: bool,
    x: f32,
}

#[allow(unused_must_use)]
impl GuiBuilder for Data {
    fn build(&self) {
        -FixedPanel {
            size: GuiDimension::Units(50.0),
            ..Default::default()
        } << {
            -Padding {
                left: PaddingValue::Units(20.0),
                ..Default::default()
            } << -Text {
                text: self.shown.clone(),
                font: "arial".to_owned(),
                align: Align::from(HAlign::Left, VAlign::Center),
                color: if self.show_red { Vec4::RED } else { Vec4::BLUE },
                ..Default::default()
            };

            -GridLayout {
                col_widths: vec![
                    GuiDimension::Relative(1.0),
                    GuiDimension::Units(130.0),
                    GuiDimension::Relative(1.0),
                    GuiDimension::Units(130.0),
                    GuiDimension::Relative(1.0),
                ],
                row_heights: vec![GuiDimension::Default; 5],
                ..Default::default()
            } << {
                let text = format!("{}", self.x);
                self.mybutton(text, false);
                for i in 1..20 {
                    let text = format!("{}", i + self.goat);
                    self.mybutton(text, i == 13);
                }
                -Button {
                    text: if self.show_red {
                        "ON".to_owned()
                    } else {
                        "OFF".to_owned()
                    },
                    callback: self.make_callback1(|data| {
                        data.show_red = !data.show_red;
                    }),
                    ..Default::default()
                };
                -LinearBar {
                    value: self.x,
                    callback: self.make_callback2(|data, bar: &LinearBar| data.x = bar.value),
                    ..Default::default()
                }
            };
        };
    }
}

#[allow(unused_must_use)]
impl Data {
    // #[glui::builder(Data)]
    pub fn mybutton(&self, text: String, exitter: bool) {
        -Padding::absolute(10.0)
            << -Button {
                text: text.clone(),
                callback: self.make_callback3(move |data, _button, sender| {
                    data.shown += &text;
                    if exitter {
                        sender.send(MessageTarget::Root, message::Exit {});
                    }
                }),
                background: ButtonBckg::RoundRect(Vec4::grey(0.1), 8.0),
                ..Default::default()
            };
    }
}

//
// Model:
// Entity     - Has id, can hold components, can be queried for components
// Component  - Holds data like position/velocity/bounding info
// System     - Updates Entities, can send Messages, can receive Messages, unique
// Message    - Can be sent, can be received, user defined
// Actor      - Can send Messages, can receive Messages, not unique, can be ui aware
//

fn main() {
    let mut w: World = World::new_win(Vec2::new(640.0, 480.0), "", Vec3::grey(0.04));

    w.add_gui(Data {
        x: 0.1,
        goat: 0,
        shown: "hy ".to_owned(),
        show_red: true,
    });
    w.run();
}

// #![allow(dead_code)]

// extern crate glutin;

// mod mecs;
// mod tools;
// mod gui;
// use mecs::*;
// use tools::*;
// use gui::*;

// #[derive(Component,Debug)]
// struct Printable {
//     text: String,
// }

// #[derive(Debug)]
// struct Print {}
// impl Message for Print {}

// #[derive(Debug)]
// struct PrintTimes(u32);
// impl Message for PrintTimes {}

// struct Printer {
// }

// impl System for Printer {
//     fn receive(&mut self, msg: Box<dyn Message>, world: &mut StaticWorld) {
//         match_downcast!( msg {
//             _ : Print => {
//                 let printables = world.entities_with_component::<Printable>();
//                 println!("Print msg for {} entities", printables.len());
//                 for p in printables {
//                     world.with_component(p, |c: &mut Printable| {
//                         println!("Text: {}", c.text);
//                     });
//                 }
//             },
//             msg : PrintTimes => {
//                 if msg.0 > 0 {
//                     world.send(SystemId::of::<Printer>(), Print{});
//                     world.send(SystemId::of::<Printer>(), PrintTimes(msg.0 - 1));
//                 }
//             },
//             _ => panic!(format!("Printer received unknown msg: {:?}",msg)),
//         });
//     }
// }

// use std::thread;
// use std::time;

// fn main() {
//     let mut w: World = World::new_win(Vec2::new(640.0,480.0), "", Vec3::grey(0.1));
//     w.add_system(Printer{});
//     let sw = w.as_static_mut();
//     let e = sw.entity();
//     sw.add_component(e, Printable{text: "hy".to_owned()});
//     sw.with_component(e,|c: &mut Printable| { c.text += "_2"; });
//     let channel = w.channel();
//     let t = {
//         let channel = channel.clone();
//         thread::spawn(move ||{
//         for _ in 0..5 {
//             thread::sleep(time::Duration::from_millis(1000));
//             channel.send(SystemId::of::<Printer>(), Print{}).unwrap();
//         }
//         channel.send(MessageTarget::None, message::Exit{}).unwrap();
//     })};
//     channel.send(SystemId::of::<Printer>(), PrintTimes(4)).unwrap();
//     w.run();
//     t.join().unwrap();
// }
