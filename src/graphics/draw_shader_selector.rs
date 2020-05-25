use tools::DrawShader;

#[derive(Debug, PartialEq)]
pub enum DrawShaderSelector {
    DefaultColored,
    DefaultTextured,
    Custom(DrawShader),
}
