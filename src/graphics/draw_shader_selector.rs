use tools::DrawShader;

#[derive(Debug, PartialEq)]
pub enum DrawShaderSelector {
    UniformColored,
    Colored,
    Textured,
    DiffusePhong,
    Custom(DrawShader),
}

impl From<DrawShader> for DrawShaderSelector {
    fn from(shader: DrawShader) -> Self {
        DrawShaderSelector::Custom(shader)
    }
}
