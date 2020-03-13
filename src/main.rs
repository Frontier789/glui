#![allow(dead_code)]
#![allow(unused_imports)]
extern crate gl;
extern crate glui;
extern crate glutin;
extern crate image;
extern crate rusttype;
extern crate unicode_normalization;
use rusttype::gpu_cache;
use unicode_normalization::UnicodeNormalization;

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
            Button {};
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

struct Font<'a> {
    rt_font: rusttype::Font<'a>,
    cache: gpu_cache::Cache<'a>,
    tex: RgbaTexture,
}

enum FontError {
    RusttypeError(rusttype::Error),
    IoError(std::io::Error),
}

impl From<std::io::Error> for FontError {
    fn from(e: std::io::Error) -> FontError {
        FontError::IoError(e)
    }
}

impl From<rusttype::Error> for FontError {
    fn from(e: rusttype::Error) -> FontError {
        FontError::RusttypeError(e)
    }
}

impl<'a> Font<'a> {
    fn from_file(file: &str) -> Result<Font<'a>, FontError> {
        let str_data = std::fs::read_to_string(file)?;
        Ok(Font::from_bytes(str_data.as_bytes().into())?)
    }
    fn from_bytes(bytes: std::sync::Arc<[u8]>) -> Result<Font<'a>, rusttype::Error> {
        let font = rusttype::Font::from_bytes(bytes)?;

        Ok(Font {
            rt_font: font,
            cache: gpu_cache::Cache::builder().dimensions(1024, 1024).build(),
            tex: RgbaTexture::new(1024, 1024),
        })
    }

    fn layout_paragraph(&mut self, scale_x: f32, scale_y: f32, width: u32, text: &str) {
        let scale = rusttype::Scale { x: scale_x, y: scale_y };
        let mut glyphs = Vec::new();
        let v_metrics = self.rt_font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let mut caret = rusttype::point(0.0, v_metrics.ascent);
        let mut last_glyph_id = None;
        
        for c in text.nfc() {
            if c.is_control() {
                match c {
                    '\n' => {
                        caret = rusttype::point(0.0, caret.y + advance_height);
                    }
                    _ => {}
                }
                continue;
            }
            let base_glyph = self.rt_font.glyph(c);
            if let Some(id) = last_glyph_id.take() {
                caret.x += self.rt_font.pair_kerning(scale, id, base_glyph.id());
            }
            last_glyph_id = Some(base_glyph.id());
            let mut glyph = base_glyph.scaled(scale).positioned(caret);
            if let Some(bb) = glyph.pixel_bounding_box() {
                if bb.max.x > width as i32 {
                    caret = rusttype::point(0.0, caret.y + advance_height);
                    glyph.set_position(caret);
                    last_glyph_id = None;
                }
            }
            caret.x += glyph.unpositioned().h_metrics().advance_width;
            self.cache.queue_glyph(0, glyph.clone());
            glyphs.push(glyph);
        }
        
        let cache = &mut self.cache;
        let tex = &mut self.tex;
        
        cache
            .cache_queued(|rect, data| {
                tex.update_u8(
                    rect.min.x as usize,
                    rect.min.y as usize,
                    (rect.max.x - rect.min.x) as usize,
                    (rect.max.y - rect.min.y) as usize,
                    gl::RED,
                    data.as_ptr(),
                );
            })
            .unwrap();
    }
}

fn main() {
    let mut win = GlutinWindow::new(
        Vec2::new(640.0, 480.0),
        "Hello gui".to_owned(),
        Vec3::grey(0.2),
    );
    // tools::gltraits::check_glerr_debug().unwrap();
    // let mut f = Font::from_bytes(include_bytes!("arial.ttf").to_vec().into()).unwrap();
    // let dpi_factor = 1.0;
    // let text = "Lorem ipsum dolor sit amet, \nconsectetur adipiscing elit, \nsed do eiusmod tempor incididunt ut \nlabore et dolore magna aliqua.";
    // let width = 640;
    // f.layout_paragraph(24.0 * dpi_factor, 24.0 * dpi_factor, width, &text);
    // tools::gltraits::check_glerr_debug().unwrap();

    // f.tex.into_image().save("cache.png").unwrap();
    
    let render_target = win.render_target();
    let mut cont = GuiContext::new(render_target);
    cont.init_gl_res();
    cont.build_gui(experimental, () );
    win.world.add_entity(Box::new(cont));
    
    win.run(GuiWinProps::tester());
}
