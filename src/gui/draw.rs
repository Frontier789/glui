use gl::types::*;
use tools::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct GLVerion {
    major: usize,
    minor: usize,
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

impl PartialOrd for GLVerion {
    fn partial_cmp(&self, other: &GLVerion) -> std::option::Option<std::cmp::Ordering> {
        if self.major != other.major {
            return self.major.partial_cmp(&other.major);
        }
        return self.minor.partial_cmp(&other.minor);
    }
}

#[derive(Debug)]
struct DrawObject {
    pts: Vec<Vec2>,
    clr: Vec3,
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
    pub fn add_rectangle(&mut self, left_up: Vec2, right_down: Vec2, clr: Vec3) {
        self.objects.push(DrawObject {
            pts: vec![
                Vec2::new(left_up.x, left_up.y) + self.offset,
                Vec2::new(right_down.x, left_up.y) + self.offset,
                Vec2::new(right_down.x, right_down.y) + self.offset,
                Vec2::new(left_up.x, left_up.y) + self.offset,
                Vec2::new(right_down.x, right_down.y) + self.offset,
                Vec2::new(left_up.x, right_down.y) + self.offset,
            ],
            clr,
        })
    }

    pub fn to_render_sequence(
        self,
        render_target: &RenderTarget,
        cache: Option<RenderSequence>,
    ) -> RenderSequence {
        let pts: Vec<Vec2> = self
            .objects
            .iter()
            .map(|o| o.pts.clone())
            .flatten()
            .map(|p| p * render_target.gui_scale)
            .collect();
        let clr = self
            .objects
            .iter()
            .map(|o| vec![o.clr; 6])
            .flatten()
            .collect();
        let n = pts.len();
        
        match cache {
            None => {
                let pbuf = Buffer::from_vec(pts);
                let cbuf = Buffer::from_vec(clr);
                let mut vao = VertexArray::new();
                vao.attrib_buffer(0, &pbuf);
                vao.attrib_buffer(1, &cbuf);
                let cmd = RenderCommand {
                    vao,
                    mode: DrawMode::Triangles,
                    first: 0,
                    count: n,
                };

                let mut shader = DrawShader::compile(
                    "#version 420 core
            
            layout(location = 0) in vec2 pos;
            layout(location = 1) in vec3 clr;
            
            uniform mat4 proj;
            
            out vec3 va_clr;
            
            void main()
            {
                gl_Position = proj * vec4(pos, 0, 1);
                
                va_clr = clr;
            }",
                    "#version 420 core
            
            in vec3 va_clr;
            
            out vec4 color;
            
            void main()
            {
                color = vec4(va_clr, 1);
            }",
                )
                .unwrap();

                shader.set_uniform(
                    "proj",
                    Mat4::ortho(
                        0.0,
                        render_target.size.y,
                        render_target.size.x,
                        0.0,
                        1.0,
                        -1.0,
                    ),
                );
                RenderSequence {
                    buffers: vec![pbuf.as_base_type(), cbuf.as_base_type()],
                    commands: vec![cmd],
                    shader,
                }
            }
            Some(rs) => {
                
                let pbuf = Buffer::from_vec(pts);
                let cbuf = Buffer::from_vec(clr);
                let mut vao = VertexArray::new();
                vao.attrib_buffer(0, &pbuf);
                vao.attrib_buffer(1, &cbuf);
                let cmd = RenderCommand {
                    vao,
                    mode: DrawMode::Triangles,
                    first: 0,
                    count: n,
                };

                let mut shader = rs.shader;

                shader.set_uniform(
                    "proj",
                    Mat4::ortho(
                        0.0,
                        render_target.size.y,
                        render_target.size.x,
                        0.0,
                        1.0,
                        -1.0,
                    ),
                );
                RenderSequence {
                    buffers: vec![pbuf.as_base_type(), cbuf.as_base_type()],
                    commands: vec![cmd],
                    shader,
                }
            }
        }
    }
}

pub struct RenderSequence {
    buffers: Vec<Buffer<f32>>,
    commands: Vec<RenderCommand>,
    shader: DrawShader,
}

impl RenderSequence {
    pub fn execute(&self) {
        for cmd in &self.commands {
            cmd.execute(&self.shader);
        }
    }
}

struct RenderCommand {
    pub vao: VertexArray,
    pub mode: DrawMode,
    pub first: usize,
    pub count: usize,
}

impl RenderCommand {
    fn execute(&self, shader: &DrawShader) {
        self.vao.bind();
        shader.prepare_draw();
        unsafe {
            gl::DrawArrays(self.mode.into(), self.first as GLint, self.count as GLsizei);
        }
    }
}
