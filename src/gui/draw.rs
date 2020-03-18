use super::font::*;
use gl::types::*;
use std::collections::HashMap;
use tools::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct GLVerion {
    major: usize,
    minor: usize,
}

impl PartialOrd for GLVerion {
    fn partial_cmp(&self, other: &GLVerion) -> std::option::Option<std::cmp::Ordering> {
        if self.major != other.major {
            return self.major.partial_cmp(&other.major);
        }
        return self.minor.partial_cmp(&other.minor);
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RenderTarget {
    pub size: Vec2,
    pub gui_scale: f32,
    pub gl_verison: GLVerion,
}

impl RenderTarget {
    pub fn logical_size(&self) -> Vec2px {
        Vec2px::from_pixels(self.size, self.gui_scale)
    }
    pub fn fill_from_context(mut self) -> Self {
        let mut major: GLint = 0;
        let mut minor: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
            gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
        }

        self.gl_verison = GLVerion {
            major: major as usize,
            minor: minor as usize,
        };
        self
    }
}

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
    pub fn texture_id(&mut self, name: &str) -> u32 {
        if !self.textures.contains_key(name) {
            if let Ok(tex) = RgbaTexture::from_file(name) {
                let id = tex.id();

                self.textures.insert(name.to_owned(), tex);
                return id;
            }
            return 0;
        }
        self.textures[name].id()
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
struct DrawObject {
    pts: Vec<Vec3>,
    clr: Vec4,
    tpt: Option<Vec<Vec2>>,
    tex: Option<u32>,
    transparent: bool,
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
    pub fn new(draw_resources: &mut DrawResources) -> DrawBuilder {
        DrawBuilder {
            objects: Vec::new(),
            offset: Vec3::zero(),
            draw_resources,
        }
    }
    pub fn add_clr_convex<FP>(&mut self, pos_fun: FP, clr: Vec4, n: usize)
    where
        FP: Fn(f32) -> Vec2,
    {
        let pts: Vec<Vec3> = (0..n).map(|i| Vec3::from_vec2(pos_fun(i as f32 / (n-1) as f32),0.0)).collect();
        
        let ids = (1..n-1).map(|i| vec![0,i,i+1]).flatten();
        
        self.objects.push(DrawObject {
            pts: offset(ids.map(|i| pts[i]).collect(), self.offset),
            clr,
            tpt: None,
            tex: None,
            transparent: clr.w < 1.0,
        })
    }
    pub fn add_clr_rect(&mut self, rct: Rect, clr: Vec4) {
        self.objects.push(DrawObject {
            pts: offset(rct.triangulate_3d(), self.offset),
            clr,
            tpt: None,
            tex: None,
            transparent: clr.w < 1.0,
        })
    }
    pub fn add_tex_rect(&mut self, rct: Rect, tex_name: &str) {
        self.objects.push(DrawObject {
            pts: offset(rct.triangulate_3d(), self.offset),
            clr: Vec4::WHITE,
            tpt: Some(Rect::unit().triangulate()),
            tex: Some(self.draw_resources.texture_id(tex_name)),
            transparent: false,
        })
    }
    pub fn add_text(&mut self, text: String, font: String, rct: Rect) {
        let font = self.draw_resources.fonts.font_family(&font).unwrap();
        let (bb_rects, uv_rects) = font.layout_paragraph(
            &text,
            24.0,
            24.0,
            (HAlign::Center, VAlign::Center),
            rct.size(),
        );
        let o = self.offset;
        self.objects.push(DrawObject {
            pts: bb_rects
                .iter()
                .map(|r| offset(r.triangulate_3d(), o))
                .flatten()
                .collect(),
            clr: Vec4::WHITE,
            tpt: Some(uv_rects.iter().map(|r| r.triangulate()).flatten().collect()),
            tex: Some(font.tex.id()),
            transparent: true,
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
            .map(|o| vec![o.clr; o.pts.len()])
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

    pub fn to_render_sequence(mut self, render_target: &RenderTarget) -> RenderSequence {
        let cmp_dobj = |o1: &DrawObject, o2: &DrawObject| {
            o1.transparent
                .cmp(&o2.transparent)
                .then(o1.tex.cmp(&o2.tex))
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
enum Uniform {
    Vector2(String, Vec2),
    Vector4(String, Vec4),
    Matrix4(String, Mat4),
    Texture2D(String, u32),
}

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