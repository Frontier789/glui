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

#[glui::builder]
fn experimental(parser: &mut WidgetTreeToList, _data: ()) {
    FixedPanel {
        size: 50.0,
        children: {
            Padding {
                left: 20.0,
                children:{Text {text: "hello".to_owned(), font: "arial".to_owned(), align: font::align(HAlign::Left, VAlign::Center), color: Vec4::RED};}
            };
            GridLayout {
                col_widths: vec![1.0; 5],
                row_heights: vec![1.0; 5],
                children: {
                    for i in 0..25 {
                        Button {
                            text: format!("{}", i),
                        };
                    }
                },
            };
        },
    };
}

fn main() {
    let mut win = GlutinWindow::new(
        Vec2::new(640.0, 480.0),
        "Hello gui".to_owned(),
        Vec3::grey(0.0),
    );
    
    // tools::gltraits::check_glerr_debug().unwrap();
    // let mut font_loader = FontLoader::new();
    
    // let font = font_loader.font_family("sans-serif").unwrap();
    // let dpi_factor = 1.0;
    // let text = "Lorem ipsum dolor sit amet, \nconsectetur adipiscing elit, \nsed do eiusmod tempor incididunt ut \nlabore et dolore magna aliqua.";
    // let width = 640;
    // font.layout_paragraph(24.0 * dpi_factor, 24.0 * dpi_factor, width, &text);
    // tools::gltraits::check_glerr_debug().unwrap();

    // font.tex.into_image().save("cache.png").unwrap();
    
    let render_target = win.render_target();
    let mut cont = GuiContext::new(render_target);
    cont.init_gl_res();
    cont.build_gui(experimental, ());
    win.world.add_entity(Box::new(cont));
    
    win.run(GuiWinProps::quick_tester());
}
