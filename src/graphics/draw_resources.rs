use graphics::DrawShaderSelector;
use mecs::WindowInfo;
use std::collections::HashMap;
use std::time::Instant;
use tools::{
    DrawShader, Font, FontLoader, FontLoaderError, Mat4, RgbaTexture, ShaderCompileError, Texture,
    Vec2,
};

pub struct DefaultDrawShaders {
    pub uniform_color: DrawShader,
    pub colored: DrawShader,
    pub textured: DrawShader,
}

pub struct DrawResources {
    textures: HashMap<String, RgbaTexture>,
    fonts: FontLoader,
    pub shaders: DefaultDrawShaders,
    pub projection_matrix: Mat4,
    pub view_matrix: Mat4,
    pub model_matrix: Mat4,
    pub inv_view_matrix: Mat4,
    pub inv_projection_matrix: Mat4,
    pub uv_matrix: Mat4,
    pub clock: Instant,
    pub window_info: WindowInfo,
}

impl DrawResources {
    pub fn new(window_info: WindowInfo) -> Result<DrawResources, ShaderCompileError> {
        Ok(DrawResources {
            shaders: Self::create_default_shaders()?,
            textures: HashMap::new(),
            fonts: FontLoader::new(),
            projection_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            inv_projection_matrix: Mat4::identity(),
            inv_view_matrix: Mat4::identity(),
            model_matrix: Mat4::identity(),
            uv_matrix: Mat4::identity(),
            clock: Instant::now(),
            window_info,
        })
    }
    pub fn font_family(&mut self, name: &str) -> Result<&mut Font, FontLoaderError> {
        self.fonts.font_family(name)
    }
    pub fn get_shader<'a>(&'a self, selector: &'a DrawShaderSelector) -> &'a DrawShader {
        match selector {
            DrawShaderSelector::DefaultUniformColor => &self.shaders.uniform_color,
            DrawShaderSelector::DefaultColored => &self.shaders.colored,
            DrawShaderSelector::DefaultTextured => &self.shaders.textured,
            DrawShaderSelector::Custom(shader) => shader,
        }
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
            // println!("Resource manager needs to load {}", name);
            for extension in ["", ".png", ".jpg", ".bmp", ".tif", ".gif"].iter() {
                if let Ok(tex) = RgbaTexture::from_file(&(name.to_owned() + extension)) {
                    // println!("Success! {}", name);
                    self.textures.insert(name.to_owned(), tex);
                    break;
                }
            }
        }

        self.textures.get_mut(name)
    }

    fn create_default_shaders() -> Result<DefaultDrawShaders, ShaderCompileError> {
        let col_shader = DrawShader::compile(COL_VERT_SOURCE, COL_FRAG_SOURCE)?;
        let uni_col_shader = DrawShader::compile(UNI_COL_VERT_SOURCE, UNI_COL_FRAG_SOURCE)?;
        let tex_shader = DrawShader::compile(TEX_VERT_SOURCE, TEX_FRAG_SOURCE)?;

        Ok(DefaultDrawShaders {
            uniform_color: uni_col_shader,
            colored: col_shader,
            textured: tex_shader,
        })
    }

    #[allow(non_snake_case)]
    pub fn MVP(&self) -> Mat4 {
        self.projection_matrix * self.view_matrix * self.model_matrix
    }
}

const COL_VERT_SOURCE: &'static str = "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 1) in vec4 clr;
    
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    
    out vec4 va_clr;
    
    void main()
    {
        gl_Position = projection * view * model * vec4(pos, 1);
        
        va_clr = clr;
    }";
const COL_FRAG_SOURCE: &'static str = "#version 420 core
    
    in vec4 va_clr;
    
    out vec4 color;
    
    void main()
    {
        color = va_clr;
    }";
const UNI_COL_VERT_SOURCE: &'static str = "#version 420 core
    
    layout(location = 0) in vec3 pos;
    
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    
    void main()
    {
        gl_Position = projection * view * model * vec4(pos, 1);
    }";
const UNI_COL_FRAG_SOURCE: &'static str = "#version 420 core
    
    uniform vec4 color;
    
    out vec4 clr;
    
    void main()
    {
        clr = color;
    }";

const TEX_VERT_SOURCE: &'static str = "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 1) in vec4 clr;
    layout(location = 2) in vec2 tpt;
    
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    uniform mat4 uv_matrix;
    
    out vec4 va_clr;
    out vec2 va_tpt;
    
    void main()
    {
        gl_Position = projection * view * model * vec4(pos, 1);
        
        va_clr = clr;
        va_tpt = (uv_matrix * vec4(tpt,0,1)).xy;
    }";
const TEX_FRAG_SOURCE: &'static str = "#version 420 core
    
    in vec4 va_clr;
    in vec2 va_tpt;
    
    uniform sampler2D tex;
    
    out vec4 color;
    
    void main()
    {
        color = va_clr * texture(tex, va_tpt);
    }";
