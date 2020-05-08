use super::font::*;
use super::gl::types::*;
use mecs::RenderTarget;
use std::collections::HashMap;
use tools::*;

use gui::Align;

pub struct DrawResources {
    shaders: HashMap<String, DrawShader>,
    textures: HashMap<String, RgbaTexture>,
    fonts: FontLoader,
}

impl DrawResources {
    pub fn new() -> DrawResources {
        DrawResources {
            shaders: HashMap::new(),
            textures: HashMap::new(),
            fonts: FontLoader::new(),
        }
    }
    pub fn has_shader(&self, name: &str) -> bool {
        self.shaders.contains_key(name)
    }
    pub fn has_texture(&self, name: &str) -> bool {
        self.textures.contains_key(name)
    }
    pub fn texture_size(&mut self, name: &str) -> Option<Vec2> {
        self.texture(name).map(|tex| tex.size())
    }
    pub fn texture_id(&mut self, name: &str) -> Option<u32> {
        self.texture(name).map(|tex| tex.id())
    }
    pub fn texture(&mut self, name: &str) -> Option<&mut RgbaTexture> {
        if !self.textures.contains_key(name) {
            for extension in ["", ".png", ".jpg", ".bmp", ".tif", ".gif"].iter() {
                if let Ok(tex) = RgbaTexture::from_file(&(name.to_owned() + extension)) {
                    self.textures.insert(name.to_owned(), tex);
                    break;
                }
            }
        }

        self.textures.get_mut(name)
    }
    pub fn create_defaults(&mut self) -> Result<(), ShaderCompileErr> {
        let col_shader = DrawShader::compile(
            "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 1) in vec4 clr;
    
    uniform mat4 proj;
    
    out vec4 va_clr;
    
    void main()
    {
        gl_Position = proj * vec4(pos, 1);
        
        va_clr = clr;
    }",
            "#version 420 core
    
    in vec4 va_clr;
    
    out vec4 color;
    
    void main()
    {
        color = va_clr;
    }",
        )?;
        let tex_shader = DrawShader::compile(
            "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 1) in vec4 clr;
    layout(location = 2) in vec2 tpt;
    
    uniform mat4 proj;
    
    out vec4 va_clr;
    out vec2 va_tpt;
    
    void main()
    {
        gl_Position = proj * vec4(pos, 1);
        
        va_clr = clr;
        va_tpt = tpt;
    }",
            "#version 420 core
    
    in vec4 va_clr;
    in vec2 va_tpt;
    
    uniform sampler2D tex;
    
    out vec4 color;
    
    void main()
    {
        color = va_clr * texture(tex, va_tpt);
    }",
        )?;
        self.shaders.insert("col_shader".to_owned(), col_shader);
        self.shaders.insert("tex_shader".to_owned(), tex_shader);

        Ok(())
    }
}

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
    pub fn add_clr_convex<FP>(&mut self, pos_fun: FP, clr: Vec4, n: usize, antialias: bool)
    where
        FP: Fn(f32) -> Vec2,
    {
        let pts: Vec<Vec2> = (0..n).map(|i| pos_fun(i as f32 / n as f32)).collect();
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
        })
    }
    pub fn add_clr_rect(&mut self, rct: Rect, clr: Vec4) {
        if clr.w == 0.0 {
            return;
        }

        self.objects.push(DrawObject {
            pts: offset(rct.triangulate_3d(), self.offset),
            clr: DrawColor::Const(clr),
            tpt: None,
            tex: None,
            transparent: clr.w < 1.0,
            depth: self.offset.z,
        })
    }
    pub fn add_tex_rect(&mut self, place_rct: Rect, cutout_rect: Rect, tex_name: &str, clr: Vec4) {
        if tex_name.is_empty() || clr.w == 0.0 {
            return;
        }

        // println!("Adding tex \"{}\" at {:?} with offset {:?}", tex_name, rct.pos(), self.offset);

        self.objects.push(DrawObject {
            pts: offset(place_rct.triangulate_3d(), self.offset),
            clr: DrawColor::Const(clr),
            tpt: Some(cutout_rect.triangulate()),
            tex: self.draw_resources.texture_id(tex_name),
            transparent: true,
            depth: self.offset.z,
        })
    }
    pub fn add_text(
        &mut self,
        text: &str,
        font: &str,
        size: Vec2,
        clr: Vec4,
        align: Align,
        font_size: f32,
    ) {
        let font = self.draw_resources.fonts.font_family(&font).unwrap();
        let (bb_rects, uv_rects) = font.layout_paragraph(
            &text,
            f32::round(font_size),
            f32::round(font_size),
            align,
            size,
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
        })
    }

    fn append_render_seq(
        &self,
        beg: usize,
        end: usize,
        render_seq: &mut RenderSequence,
        render_target: &RenderTarget,
    ) {
        let pts: Vec<Vec3> = self.objects[beg..end]
            .iter()
            .map(|o| o.pts.clone())
            .flatten()
            .map(|p| p * render_target.gui_scale)
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

        let pts_count: usize = self.objects[beg..end].iter().map(|o| o.pts.len()).sum();
        let pbuf = Buffer::from_vec(pts);
        let cbuf = Buffer::from_vec(clr);
        let mut vao = VertexArray::new();
        vao.attrib_buffer(0, &pbuf);
        vao.attrib_buffer(1, &cbuf);
        render_seq.buffers.push(pbuf.as_base_type());
        render_seq.buffers.push(cbuf.as_base_type());

        let mut uniforms = vec![Uniform::Matrix4(
            "proj".to_owned(),
            Mat4::ortho(
                0.0,
                render_target.size.y,
                render_target.size.x,
                0.0,
                1.0,
                -1.0,
            ),
        )];
        let shader_name;
        if let (Some(tex), Some(_)) = (&self.objects[beg].tex, &self.objects[beg].tpt) {
            let tpt: Vec<Vec2> = self.objects[beg..end]
                .iter()
                .map(|o| o.tpt.clone().unwrap())
                .flatten()
                .collect();
            let tbuf = Buffer::from_vec(tpt);
            vao.attrib_buffer(2, &tbuf);
            shader_name = "tex_shader".to_owned();
            render_seq.buffers.push(tbuf.as_base_type());

            uniforms.push(Uniform::Texture2D("tex".to_owned(), tex.clone()));
        } else {
            shader_name = "col_shader".to_owned();
        }
        render_seq.commands.push(RenderCommand {
            vao,
            mode: DrawMode::Triangles,
            first: 0,
            count: pts_count,
            shader: shader_name,
            uniforms,
            transparent: self.objects[beg].transparent,
        });
    }

    pub fn into_render_sequence(mut self, render_target: &RenderTarget) -> RenderSequence {
        let cmp_dobj = |o1: &DrawObject, o2: &DrawObject| {
            if o1.depth != o2.depth && (o1.transparent || o2.transparent) {
                o1.depth.partial_cmp(&o2.depth).unwrap()
            } else {
                o1.transparent
                    .cmp(&o2.transparent)
                    .then(o1.tex.cmp(&o2.tex))
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

            self.append_render_seq(i, j, &mut r, render_target);
            i = j;
        }
        r
    }
}

#[derive(Debug)]
pub enum Uniform {
    Vector2(String, Vec2),
    Vector4(String, Vec4),
    Matrix4(String, Mat4),
    Texture2D(String, u32),
}

#[derive(Debug)]
pub struct RenderSequence {
    buffers: Vec<Buffer<f32>>,
    commands: Vec<RenderCommand>,
}

impl RenderSequence {
    pub fn new() -> RenderSequence {
        RenderSequence {
            buffers: vec![],
            commands: vec![],
        }
    }
    pub fn execute(&self, resources: &mut DrawResources) {
        for cmd in &self.commands {
            cmd.vao.bind();
            let shader = resources.shaders.get_mut(&cmd.shader).unwrap();
            shader.bind();
            for uniform in &cmd.uniforms {
                match uniform {
                    Uniform::Vector2(id, value) => {
                        shader.set_uniform(id, *value);
                    }
                    Uniform::Vector4(id, value) => {
                        shader.set_uniform(id, *value);
                    }
                    Uniform::Matrix4(id, value) => {
                        shader.set_uniform(id, *value);
                    }
                    Uniform::Texture2D(id, value) => {
                        shader.uniform_tex2d_id(id, *value);
                    }
                }
            }
            cmd.execute();
        }
    }
}

#[derive(Debug)]
struct RenderCommand {
    pub vao: VertexArray,
    pub mode: DrawMode,
    pub first: usize,
    pub count: usize,
    pub shader: String,
    pub uniforms: Vec<Uniform>,
    pub transparent: bool,
}

impl RenderCommand {
    fn execute(&self) {
        unsafe {
            if self.transparent {
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            } else {
                gl::Disable(gl::BLEND);
            }
            gl::DrawArrays(self.mode.into(), self.first as GLint, self.count as GLsizei);
        }
    }
}
