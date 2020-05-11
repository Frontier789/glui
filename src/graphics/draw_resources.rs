use std::collections::HashMap;
use tools::{
    DrawShader, Font, FontLoader, FontLoaderError, RgbaTexture, ShaderCompileError, Texture, Vec2,
};

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
    pub fn font_family(&mut self, name: &str) -> Result<&mut Font, FontLoaderError> {
        self.fonts.font_family(name)
    }
    pub fn has_shader(&self, name: &str) -> bool {
        self.shaders.contains_key(name)
    }
    pub fn shader(&mut self, name: &str) -> Option<&mut DrawShader> {
        self.shaders.get_mut(name)
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
    pub fn create_defaults(&mut self) -> Result<(), ShaderCompileError> {
        let col_shader = DrawShader::compile(COL_VERT_SOURCE, COL_FRAG_SOURCE)?;
        let tex_shader = DrawShader::compile(TEX_VERT_SOURCE, TEX_FRAG_SOURCE)?;
        self.shaders.insert("col_shader".to_owned(), col_shader);
        self.shaders.insert("tex_shader".to_owned(), tex_shader);

        Ok(())
    }
}

const COL_VERT_SOURCE: &'static str = "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 1) in vec4 clr;
    
    uniform mat4 proj;
    
    out vec4 va_clr;
    
    void main()
    {
        gl_Position = proj * vec4(pos, 1);
        
        va_clr = clr;
    }";
const COL_FRAG_SOURCE: &'static str = "#version 420 core
    
    in vec4 va_clr;
    
    out vec4 color;
    
    void main()
    {
        color = va_clr;
    }";

const TEX_VERT_SOURCE: &'static str = "#version 420 core
    
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
