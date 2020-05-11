use tools::{Mat4, Vec2, Vec4};

#[derive(Debug, Clone)]
pub enum Uniform {
    Vector2(String, Vec2),
    Vector4(String, Vec4),
    Matrix4(String, Mat4),
    Texture2D(String, u32),
}
