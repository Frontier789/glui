use super::super::Component;
use graphics::RenderSequence;
use tools::Mat4;

#[derive(Debug, Component)]
pub struct DrawComponent {
    pub render_seq: RenderSequence,
    pub model_matrix: Mat4,
}

impl DrawComponent {
    pub fn from_render_seq(render_seq: RenderSequence) -> DrawComponent {
        DrawComponent {
            render_seq,
            model_matrix: Mat4::identity(),
        }
    }
}
