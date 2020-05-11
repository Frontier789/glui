extern crate font_kit;
extern crate rusttype;
extern crate unicode_normalization;

use self::rusttype::gpu_cache;
use self::unicode_normalization::UnicodeNormalization;

use gui::{Align, HAlign, VAlign};
use tools::RgbaTexture;
use tools::Vec2;
use tools::{FontError, Rect};

pub struct Font {
    rt_font: rusttype::Font<'static>,
    cache: gpu_cache::Cache<'static>,
    pub tex: RgbaTexture,
}

impl Font {
    pub(super) fn from_collection(
        collection: &rusttype::FontCollection<'static>,
        index: usize,
    ) -> Result<Font, FontError> {
        Ok(Font {
            rt_font: collection.font_at(index)?,
            cache: gpu_cache::Cache::builder().dimensions(1024, 1024).build(),
            tex: RgbaTexture::new(1024, 1024),
        })
    }

    pub fn layout_paragraph(
        &mut self,
        text: &str,
        scale_x: f32,
        scale_y: f32,
        align: Align,
        size: Vec2,
    ) -> (Vec<Rect>, Vec<Rect>) {
        let scale = rusttype::Scale {
            x: scale_x,
            y: scale_y,
        };
        let mut glyphs = Vec::new();
        let v_metrics = self.rt_font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let mut caret = rusttype::point(0.0, v_metrics.ascent);
        let mut last_glyph_id = None;
        let mut row_widths: Vec<f32> = vec![0.0];

        for c in text.nfc() {
            if c.is_control() {
                match c {
                    '\n' => {
                        caret = rusttype::point(0.0, caret.y + advance_height);
                        row_widths.push(0.0);
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
                if bb.max.x > size.x as i32 {
                    caret = rusttype::point(0.0, caret.y + advance_height);
                    row_widths.push(0.0);
                    glyph.set_position(caret);
                    last_glyph_id = None;
                }
            }
            caret.x += glyph.unpositioned().h_metrics().advance_width;
            let w = row_widths.last_mut().unwrap();
            *w = caret.x;
            glyphs.push(glyph);
        }

        let all_height = caret.y;

        let mut cur_y = v_metrics.ascent;
        let mut row_i = 0;

        for g in &mut glyphs {
            if g.position().y != cur_y {
                row_i += 1;
                cur_y = g.position().y;
            }

            let w = row_widths[row_i];
            let p_old = g.position();
            let mut p = g.position();

            match align.horizontal {
                HAlign::Left => {}
                HAlign::Center => p.x += (size.x - w) / 2.0,
                HAlign::Right => p.x += size.x - w,
            };

            match align.vertical {
                VAlign::Top => {}
                VAlign::Center => p.y += (size.y - all_height) / 2.0,
                VAlign::Bottom => p.y += size.y - all_height,
            };

            if p_old != p {
                g.set_position(p);
            }
        }

        let cache = &mut self.cache;
        let tex = &mut self.tex;

        for g in &glyphs {
            cache.queue_glyph(0, g.clone());
        }

        cache
            .cache_queued(|rect, data| {
                let mut data_a: Vec<u8> = vec![255u8; data.len() * 4];
                for i in 0..data.len() {
                    data_a[i * 4 + 3] = data[i];
                }

                tex.update_u8(
                    rect.min.x as usize,
                    rect.min.y as usize,
                    (rect.max.x - rect.min.x) as usize,
                    (rect.max.y - rect.min.y) as usize,
                    gl::RGBA,
                    data_a.as_ptr(),
                );
            })
            .unwrap();

        let mut uv_rects = Vec::<Rect>::with_capacity(glyphs.len());
        let mut bb_rects = Vec::<Rect>::with_capacity(glyphs.len());

        for g in glyphs {
            let opt_rect = cache.rect_for(0, &g).unwrap();

            if let Some((uv, bb)) = opt_rect {
                uv_rects.push(Rect {
                    left: uv.min.x,
                    right: uv.max.x,
                    top: uv.min.y,
                    bottom: uv.max.y,
                });
                bb_rects.push(Rect {
                    left: bb.min.x as f32,
                    right: bb.max.x as f32,
                    top: bb.min.y as f32,
                    bottom: bb.max.y as f32,
                });
            }
        }

        (bb_rects, uv_rects)
    }
}
