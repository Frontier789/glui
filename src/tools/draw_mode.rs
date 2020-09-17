use super::gl::types::*;
use std::fmt;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum DrawMode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    TriangleStrip,
    TriangleFan,
    Triangles,
    LinesAdjacency,
    LineStripAdjacency,
    TrianglesAdjacency,
    TriangleStripAdjacency,
    Patches,
}

impl DrawMode {
    pub fn batchable(self) -> bool {
        match self {
            DrawMode::Points => true,
            DrawMode::LineStrip => false,
            DrawMode::LineLoop => false,
            DrawMode::Lines => true,
            DrawMode::TriangleStrip => false,
            DrawMode::TriangleFan => false,
            DrawMode::Triangles => true,
            DrawMode::LinesAdjacency => false,
            DrawMode::LineStripAdjacency => false,
            DrawMode::TrianglesAdjacency => false,
            DrawMode::TriangleStripAdjacency => false,
            DrawMode::Patches => false,
        }
    }
}

impl fmt::Display for DrawMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DrawMode::Points => "Points",
                DrawMode::LineStrip => "LineStrip",
                DrawMode::LineLoop => "LineLoop",
                DrawMode::Lines => "Lines",
                DrawMode::TriangleStrip => "TriangleStrip",
                DrawMode::TriangleFan => "TriangleFan",
                DrawMode::Triangles => "Triangles",
                DrawMode::LinesAdjacency => "LinesAdjacency",
                DrawMode::LineStripAdjacency => "LineStripAdjacency",
                DrawMode::TrianglesAdjacency => "TrianglesAdjacency",
                DrawMode::TriangleStripAdjacency => "TriangleStripAdjacency",
                DrawMode::Patches => "Patches",
            }
        )
    }
}

impl From<DrawMode> for GLenum {
    fn from(m: DrawMode) -> Self {
        match m {
            DrawMode::Points => gl::POINTS,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::LineLoop => gl::LINE_LOOP,
            DrawMode::Lines => gl::LINES,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::LinesAdjacency => gl::LINES_ADJACENCY,
            DrawMode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            DrawMode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            DrawMode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            DrawMode::Patches => gl::PATCHES,
        }
    }
}
