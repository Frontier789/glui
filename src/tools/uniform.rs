use tools::{Mat4, Vec2, Vec3, Vec4};

#[derive(Debug, Clone)]
pub enum Uniform {
    Float(String, f32),
    Vector2(String, Vec2),
    Vector3(String, Vec3),
    Vector4(String, Vec4),
    Matrix4(String, Mat4),
    TextureCube(String, u32),
    Texture2D(String, u32),
}
