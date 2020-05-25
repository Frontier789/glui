use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum Indices {
    None,
    Range(Range<usize>),
    Vec(Vec<u32>),
}

impl Default for Indices {
    fn default() -> Self {
        Self::None
    }
}

impl From<Range<usize>> for Indices {
    fn from(range: Range<usize>) -> Self {
        Indices::Range(range)
    }
}
impl From<(usize, usize)> for Indices {
    fn from(range: (usize, usize)) -> Self {
        Indices::Range(range.0..range.1)
    }
}
impl From<Vec<u32>> for Indices {
    fn from(vec: Vec<u32>) -> Self {
        Indices::Vec(vec)
    }
}
