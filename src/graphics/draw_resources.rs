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
    pub diffuse_phong: DrawShader,
    pub colored: DrawShader,
    pub textured: DrawShader,
    pub phong: DrawShader,
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
            DrawShaderSelector::UniformColored => &self.shaders.uniform_color,
            DrawShaderSelector::Colored => &self.shaders.colored,
            DrawShaderSelector::Textured => &self.shaders.textured,
            DrawShaderSelector::DiffusePhong => &self.shaders.diffuse_phong,
            DrawShaderSelector::Phong => &self.shaders.phong,
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
        let dif_shader = DrawShader::compile(DIF_VERT_SOURCE, DIF_FRAG_SOURCE)?;
        let phong_shader = DrawShader::compile(PHONG_VERT_SOURCE, PHONG_FRAG_SOURCE)?;

        Ok(DefaultDrawShaders {
            uniform_color: uni_col_shader,
            colored: col_shader,
            textured: tex_shader,
            diffuse_phong: dif_shader,
            phong: phong_shader,
        })
    }

    #[allow(non_snake_case)]
    pub fn MVP(&self) -> Mat4 {
        self.projection_matrix * self.view_matrix * self.model_matrix
    }

    #[allow(non_snake_case)]
    pub fn MV(&self) -> Mat4 {
        self.view_matrix * self.model_matrix
    }

    #[allow(non_snake_case)]
    pub fn VP(&self) -> Mat4 {
        self.projection_matrix * self.view_matrix
    }

    #[allow(non_snake_case)]
    pub fn normal_matrix_MV(&self) -> Mat4 {
        (self.view_matrix * self.model_matrix).inverse().transpose()
    }

    pub fn normal_matrix_model(&self) -> Mat4 {
        self.model_matrix.inverse().transpose()
    }
}

const COL_VERT_SOURCE: &'static str = "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 1) in vec4 clr;
    
    uniform mat4 MVP;
    
    out vec4 va_clr;
    
    void main()
    {
        gl_Position = MVP * vec4(pos, 1);
        
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
    
    uniform mat4 MVP;
    
    void main()
    {
        gl_Position = MVP * vec4(pos, 1);
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
    
    uniform mat4 MVP;
    uniform mat4 uv_matrix;
    
    out vec4 va_clr;
    out vec2 va_tpt;
    
    void main()
    {
        gl_Position = MVP * vec4(pos, 1);
        
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

const DIF_VERT_SOURCE: &'static str = "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 3) in vec3 nrm;
    
    uniform mat4 MVP;
    uniform mat4 normal_model;
    
    out vec3 va_nrm;
    
    void main()
    {
        gl_Position = MVP * vec4(pos, 1);
        
        va_nrm = vec3(normal_model * vec4(nrm, 0));
    }";
const DIF_FRAG_SOURCE: &'static str = "#version 420 core
    
    uniform vec3 light_direction = normalize(vec3(1,1,1));
    
    in vec3 va_nrm;
    
    out vec4 color;
    
    void main()
    {
        float d = dot(normalize(va_nrm), light_direction);
        color = vec4(vec3(max(d,0) + 0.1), 1);
    }";

const PHONG_VERT_SOURCE: &'static str = "#version 420 core
    
    layout(location = 0) in vec3 pos;
    layout(location = 3) in vec3 nrm;
    
    uniform mat4 model;
    uniform mat4 MVP;
    uniform mat4 normal_model;
    
    out vec3 va_nrm;
    out vec3 va_pos;
    
    void main()
    {
        gl_Position = MVP * vec4(pos, 1);
        
        va_nrm = vec3(normal_model * vec4(nrm, 0));
        va_pos = vec3(model * vec4(pos, 1));
    }";
const PHONG_FRAG_SOURCE: &'static str = "#version 420 core
    
    uniform vec3 light_direction = normalize(vec3(1,1,1));
    uniform vec3 cam_pos;
    uniform vec3 Ka = vec3(0.1);
    uniform vec3 Kd = vec3(1.0);
    uniform vec3 Ks = vec3(0.2);
    uniform float Ns = 9.0;

    in vec3 va_nrm;
    in vec3 va_pos;
    
    out vec4 color;
    
    void main()
    {
        vec3 n = normalize(va_nrm);

        vec3 V = normalize(cam_pos - va_pos);
        vec3 R = reflect( -light_direction, n);
        vec3 specular = Ks * pow(max(dot(V, R), 0.0), Ns);

        vec3 diffuse = max(dot(n,light_direction),0) * Kd;

        color = vec4(Ka + diffuse + specular,1);
    }";
