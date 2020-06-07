use tools::DrawShader;

#[derive(Debug, PartialEq)]
pub enum DrawShaderSelector {
    DefaultUniformColor,
    DefaultColored,
    DefaultTextured,
    Custom(DrawShader),
}

impl From<DrawShader> for DrawShaderSelector {
    fn from(shader: DrawShader) -> Self {
        DrawShaderSelector::Custom(shader)
    }
}
