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
}

impl DrawResources {
    pub fn new() -> DrawResources {
        DrawResources {
            shaders: HashMap::new(),
            textures: HashMap::new(),
        }
    }
    pub fn has_shader(&self, name: &str) -> bool {
        self.shaders.contains_key(name)
    }
    pub fn has_texture(&self, name: &str) -> bool {
        self.textures.contains_key(name)
    }
    pub fn need_texture(&mut self, name: &str) -> image::ImageResult<()> {
        if !self.textures.contains_key(name) {
            let tex = RgbaTexture::from_file(name)?;
            
            self.textures.insert(
                name.to_owned(),
                tex,
            );
        }
        Ok(())
    }
    pub fn create_defaults(&mut self) -> Result<(), ShaderCompileErr> {
        let col_shader = DrawShader::compile(
            "#version 420 core
    
    layout(location = 0) in vec2 pos;
    layout(location = 1) in vec4 clr;
    
    uniform mat4 proj;
    
    out vec4 va_clr;
    
    void main()
    {
        gl_Position = proj * vec4(pos, 0, 1);
        
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
    
    layout(location = 0) in vec2 pos;
    layout(location = 1) in vec4 clr;
    layout(location = 2) in vec2 tpt;
    
    uniform mat4 proj;
    
    out vec4 va_clr;
    out vec2 va_tpt;
    
    void main()
    {
        gl_Position = proj * vec4(pos, 0, 1);
        
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
    pts: Vec<Vec2>,
    clr: Vec4,
    tpt: Option<Vec<Vec2>>,
    tex: Option<String>,
}

pub struct DrawBuilder {
    objects: Vec<DrawObject>,
    pub offset: Vec2,
}

impl DrawBuilder {
    pub fn new() -> DrawBuilder {
        DrawBuilder {
            objects: Vec::new(),
            offset: Vec2::zero(),
        }
    }
    pub fn add_clr_rect(&mut self, rct: Rect, clr: Vec4) {
        self.objects.push(DrawObject {
            pts: rct.offset(self.offset).triangulate(),
            clr,
            tpt: None,
            tex: None,
        })
    }
    pub fn add_tex_rect(&mut self, rct: Rect, tex_name: &str) {
        self.objects.push(DrawObject {
            pts: rct.offset(self.offset).triangulate(),
            clr: Vec4::WHITE,
            tpt: Some(Rect::unit().triangulate()),
            tex: Some(tex_name.to_owned()),
        })
    }

    fn append_render_seq(
        &self,
        beg: usize,
        end: usize,
        render_seq: &mut RenderSequence,
        render_target: &RenderTarget,
    ) {
        let pts: Vec<Vec2> = self.objects[beg..end]
            .iter()
            .map(|o| o.pts.clone())
            .flatten()
            .map(|p| p * render_target.gui_scale)
            .collect();

        let clr: Vec<Vec4> = self.objects[beg..end]
            .iter()
            .map(|o| vec![o.clr; 6])
            .flatten()
            .collect();
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
            
            uniforms.push(Uniform::Texture2D(
                "tex".to_owned(),
                tex.clone(),
            ));
        } else {
            shader_name = "col_shader".to_owned();
        }
        
        render_seq.commands.push(RenderCommand {
            vao,
            mode: DrawMode::Triangles,
            first: 0,
            count: (end - beg) * 6,
            shader: shader_name,
            uniforms
        });
    }

    pub fn to_render_sequence(mut self, render_target: &RenderTarget) -> RenderSequence {
        let cmp_dobj = |o1: &DrawObject, o2: &DrawObject| o1.tex.cmp(&o2.tex);
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
    Texture2D(String, String),
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
    
    pub fn update_resources(&self, resources: &mut DrawResources) {
        for cmd in &self.commands {
            for u in &cmd.uniforms {
                if let Uniform::Texture2D(_, tex) = u {
                    resources.need_texture(tex).unwrap();
                }
            }
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
                        shader.set_uniform(id, &resources.textures[value]);
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
}

impl RenderCommand {
    fn execute(&self) {
        unsafe {
            gl::DrawArrays(self.mode.into(), self.first as GLint, self.count as GLsizei);
        }
    }
}
