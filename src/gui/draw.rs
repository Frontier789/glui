use tools::*;

use graphics::{DrawResources, DrawShaderSelector, RenderCommand, RenderSequence};
use gui::Align;

#[derive(Debug)]
pub enum DrawColor {
    Array(Vec<Vec4>),
    Const(Vec4),
    Default,
}

#[derive(Debug)]
struct DrawObject {
    pts: Vec<Vec3>,
    clr: DrawColor,
    tpt: Option<Vec<Vec2>>,
    tex: Option<u32>,
    transparent: bool,
    depth: f32,
    mode: DrawMode,
}

pub struct DrawBuilder<'a> {
    objects: Vec<DrawObject>,
    pub offset: Vec3,
    draw_resources: &'a mut DrawResources,
}

fn offset(v: Vec<Vec3>, o: Vec3) -> Vec<Vec3> {
    v.iter().map(|p| *p + o).collect()
}

impl<'a> DrawBuilder<'a> {
    pub fn gui_scale(&self) -> f32 {
        self.draw_resources.window_info.gui_scale
    }
    pub fn resources(&mut self) -> &mut DrawResources {
        self.draw_resources
    }
    pub fn new(draw_resources: &mut DrawResources) -> DrawBuilder {
        DrawBuilder {
            objects: Vec::new(),
            offset: Vec3::zero(),
            draw_resources,
        }
    }
    pub fn add_line_strip(&mut self, points: Vec<Vec2px>, clr: Vec4) {
        self.objects.push(DrawObject {
            pts: offset(
                points
                    .iter()
                    .map(|p| Vec3::from_vec2(p.to_pixels(self.gui_scale()), 0.0))
                    .collect(),
                self.offset,
            ),
            clr: DrawColor::Const(clr),
            tpt: None,
            tex: None,
            transparent: false,
            depth: self.offset.z,
            mode: DrawMode::LineStrip,
        })
    }
    pub fn add_clr_convex<FP>(&mut self, pos_fun: FP, clr: Vec4, n: usize, antialias: bool)
    where
        FP: Fn(f32) -> Vec2px,
    {
        let pts: Vec<Vec2> = (0..n)
            .map(|i| pos_fun(i as f32 / n as f32).to_pixels(self.gui_scale()))
            .collect();
        let norm: Vec<Vec2> = (0..n)
            .map(|i| {
                let a = pts[(i + n - 1) % n];
                let b = pts[i];
                let c = pts[(i + 1) % n];
                ((a - b).perp() + (b - c).perp()).sgn()
            })
            .collect();

        let id_to_p = |&i| {
            Vec3::from_vec2(
                if i < n {
                    let normal: Vec2 = norm[i];

                    pts[i] - normal * (normal.unsign().minxy() * 1.1 + 0.8)
                } else {
                    pts[i - n]
                },
                0.0,
            )
        };

        let id_to_c = |&i| {
            let mut c = clr;
            if i >= n {
                let normal: Vec2 = norm[i - n];

                let x = normal.unsign().minxy();

                c.w = if x >= 0.03 {
                    1.0 - f32::powf(x - 0.03, 0.3)
                } else {
                    let x = x * 10.0;

                    1.0 - f32::powf(3.0 * x * x - 2.0 * x * x * x, 1.5)
                };
            }
            c
        };

        let ids_fill = (1..n - 1).map(|i| vec![0, i, i + 1]).flatten();
        let ids_outline = (0..n)
            .map(|i| vec![i, (i + 1) % n, (i + 1) % n + n, i, (i + 1) % n + n, i + n])
            .flatten();

        let ids = if antialias {
            ids_outline.chain(ids_fill).collect::<Vec<usize>>()
        } else {
            ids_fill.collect::<Vec<usize>>()
        };

        let ids = ids.iter();

        self.objects.push(DrawObject {
            pts: offset(ids.clone().map(id_to_p).collect(), self.offset),
            clr: DrawColor::Array(ids.map(id_to_c).collect()),
            tpt: None,
            tex: None,
            transparent: antialias,
            depth: self.offset.z,
            mode: DrawMode::Triangles,
        })
    }
    pub fn add_clr_rect(&mut self, rct: Rect, clr: Vec4) {
        if clr.w == 0.0 {
            return;
        }

        self.objects.push(DrawObject {
            pts: offset((rct * self.gui_scale()).triangulate_3d(), self.offset),
            clr: DrawColor::Const(clr),
            tpt: None,
            tex: None,
            transparent: clr.w < 1.0,
            depth: self.offset.z,
            mode: DrawMode::Triangles,
        })
    }
    pub fn add_tex(&mut self, pos_mid: Vec2px, tex_name: &str, clr: Vec4, scale: f32) {
        let s = self
            .draw_resources
            .texture_size(tex_name)
            .unwrap_or_default();

        self.add_tex_rect(
            Rect::from_pos_size(pos_mid.to_pixels(1.0) - s / 2.0 * scale, s * scale),
            Rect::unit(),
            tex_name,
            clr,
        )
    }
    pub fn add_tex_rect(&mut self, place_rct: Rect, cutout_rect: Rect, tex_name: &str, clr: Vec4) {
        if tex_name.is_empty() || clr.w == 0.0 {
            return;
        }

        // println!("Adding tex \"{}\" at {:?} with offset {:?}", tex_name, rct.pos(), self.offset);

        self.objects.push(DrawObject {
            pts: offset((place_rct * self.gui_scale()).triangulate_3d(), self.offset),
            clr: DrawColor::Const(clr),
            tpt: Some(cutout_rect.triangulate()),
            tex: self.draw_resources.texture_id(tex_name),
            transparent: true,
            depth: self.offset.z,
            mode: DrawMode::Triangles,
        })
    }
    pub fn add_text(
        &mut self,
        text: &str,
        font: &str,
        size: Vec2px,
        clr: Vec4,
        align: Align,
        font_size: f32,
    ) {
        let gui_scale = self.gui_scale();
        let font = self.draw_resources.font_family(&font).unwrap();
        let (bb_rects, uv_rects) = font.layout_paragraph(
            &text,
            f32::round(font_size),
            f32::round(font_size),
            align,
            size.to_pixels(gui_scale),
        );
        let o = self.offset;
        self.objects.push(DrawObject {
            pts: bb_rects
                .iter()
                .map(|r| offset(r.triangulate_3d(), o))
                .flatten()
                .collect(),
            clr: DrawColor::Const(clr),
            tpt: Some(uv_rects.iter().map(|r| r.triangulate()).flatten().collect()),
            tex: Some(font.tex.id()),
            transparent: true,
            depth: self.offset.z,
            mode: DrawMode::Triangles,
        })
    }

    fn append_render_seq(&self, beg: usize, end: usize, render_seq: &mut RenderSequence) {
        let pts: Vec<Vec3> = self.objects[beg..end]
            .iter()
            .map(|o| o.pts.clone())
            .flatten()
            .map(|p| p * self.draw_resources.window_info.gui_scale)
            .collect();

        let clr: Vec<Vec4> = self.objects[beg..end]
            .iter()
            .map(|o| match &o.clr {
                DrawColor::Array(v) => v.clone(),
                DrawColor::Const(c) => vec![*c; o.pts.len()],
                DrawColor::Default => vec![Vec4::WHITE; o.pts.len()],
            })
            .flatten()
            .collect();

        let pbuf = Buffer::from_vec(&pts);
        let cbuf = Buffer::from_vec(&clr);
        let mut vao = VertexArray::new();
        vao.attrib_buffer(0, &pbuf);
        vao.attrib_buffer(1, &cbuf);
        render_seq.add_buffer(pbuf.into_base_type());
        render_seq.add_buffer(cbuf.into_base_type());

        let mut uniforms = vec![];

        let shader;
        if let (Some(tex), Some(_)) = (&self.objects[beg].tex, &self.objects[beg].tpt) {
            let tpt: Vec<Vec2> = self.objects[beg..end]
                .iter()
                .map(|o| o.tpt.clone().unwrap())
                .flatten()
                .collect();
            let tbuf = Buffer::from_vec(&tpt);
            vao.attrib_buffer(2, &tbuf);
            shader = DrawShaderSelector::Textured;
            render_seq.add_buffer(tbuf.into_base_type());

            uniforms.push(Uniform::Texture2D("tex".to_owned(), *tex));
        } else {
            shader = DrawShaderSelector::Colored;
        }
        render_seq.add_command(RenderCommand {
            vao,
            mode: self.objects[beg].mode,
            shader,
            uniforms,
            transparent: self.objects[beg].transparent,
            instances: 1,
            wireframe: false,
        });
    }

    pub fn into_render_sequence(mut self) -> RenderSequence {
        let cmp_dobj = |o1: &DrawObject, o2: &DrawObject| {
            if o1.depth != o2.depth && (o1.transparent || o2.transparent) {
                o1.depth.partial_cmp(&o2.depth).unwrap()
            } else {
                o1.transparent
                    .cmp(&o2.transparent)
                    .then(o1.tex.cmp(&o2.tex))
                    .then(o1.mode.cmp(&o2.mode))
            }
        };
        self.objects.sort_by(cmp_dobj);
        let n = self.objects.len();
        let mut r = RenderSequence::new();
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && cmp_dobj(&self.objects[i], &self.objects[j]) == std::cmp::Ordering::Equal
            {
                j += 1;
            }

            self.append_render_seq(i, j, &mut r);
            i = j;
        }
        r
    }
}
