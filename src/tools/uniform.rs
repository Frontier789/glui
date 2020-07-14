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

impl Uniform {
    pub fn from<U>(name: &str, val: U) -> Uniform
        where U: UniformCompatible
    {
        val.into_uniform(name)
    }
}

pub trait UniformCompatible {
    fn into_uniform(self, name: &str) -> Uniform;
}

impl UniformCompatible for f32 {
    fn into_uniform(self, name: &str) -> Uniform {
        Uniform::Float(name.to_string(), self)
    }
}

impl UniformCompatible for Vec2 {
    fn into_uniform(self, name: &str) -> Uniform {
        Uniform::Vector2(name.to_string(), self)
    }
}

impl UniformCompatible for Vec3 {
    fn into_uniform(self, name: &str) -> Uniform {
        Uniform::Vector3(name.to_string(), self)
    }
}

impl UniformCompatible for Vec4 {
    fn into_uniform(self, name: &str) -> Uniform {
        Uniform::Vector4(name.to_string(), self)
    }
}

impl UniformCompatible for Mat4 {
    fn into_uniform(self, name: &str) -> Uniform {
        Uniform::Matrix4(name.to_string(), self)
    }
}
