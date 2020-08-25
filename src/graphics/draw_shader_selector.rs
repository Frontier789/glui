use tools::DrawShader;

#[derive(Debug, PartialEq, Clone)]
pub enum DrawShaderSelector {
    UniformColored,
    Colored,
    Textured,
    DiffusePhong,
    Phong,
    Custom(DrawShader),
}

impl From<DrawShader> for DrawShaderSelector {
    fn from(shader: DrawShader) -> Self {
        DrawShaderSelector::Custom(shader)
    }
}
