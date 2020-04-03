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
mod ecs;
mod tools;

use gl::types::*;
use std::any::Any;
use std::ops::Index;
use std::time::Instant;
use tools::camera::Camera;
use tools::*;

use ecs::*;
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

#[derive(Clone,Debug,PartialEq)]
struct Data {
    goat: i32,
    shown: String,
}

#[glui::builder(Data)]
fn experimental(param: Data) {
    FixedPanel {
        size: 50.0,
        children: {
            Padding {
                left: 20.0,
                children: {
                    Text {
                        text: param.shown.clone(),
                        font: "arial".to_owned(),
                        align: font::align(HAlign::Left, VAlign::Center),
                        color: Vec4::RED,
                    };
                },
            };
            GridLayout {
                col_widths: vec![1.0; 5],
                row_heights: vec![1.0; 5],
                children: {
                    for i in 0..25 {
                        let text = format!("{}", i + param.goat);
                        mybutton(text);
                    }
                },
            };
        },
    };
}

#[glui::builder(Data)]
pub fn mybutton(text: String) {
    Padding {
        left: 10.0,
        right: 10.0,
        top: 10.0,
        bottom: 10.0,
        children: {
            Button {
                text: text.clone(),
                callback: move |data: &mut Data| {data.shown = data.shown.clone() + " " + &text;},
                background: ButtonBckg::RoundRect(Vec4::grey(0.1), 6.0),
            };
        },
    };
}

fn main() {
    let mut win = GlutinWindow::new(Vec2::new(640.0, 480.0), "Hello gui", Vec3::grey(0.0));
    let render_target = win.render_target();
    let mut cont = GuiContext::new(render_target, true, experimental, Data{goat: 0, shown: "".to_owned()});
    cont.init_gl_res();
    cont.rebuild_gui();
    win.world.add_entity(Box::new(cont));
    win.run(GuiWinProps::quick_tester());
}
