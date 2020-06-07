use super::super::Component;
use tools::Vec3;

#[derive(Debug, Component)]
pub struct BodyComponent {
    pub center: Vec3,
}
