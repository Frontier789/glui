use super::super::Component;
use graphics::RenderSequence;
use tools::Mat4;

#[derive(Debug, Component)]
pub struct DrawComponent {
    pub render_seq: RenderSequence,
    pub model_matrix: Mat4,
}
