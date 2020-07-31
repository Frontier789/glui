use std::ops::Range;

pub fn linsapce(a: f32, b: f32, n: usize) -> Vec<f32> {
    let mut v = vec![];
    for i in 0..n {
        v.push(i as f32 / ((n - 1) as f32) * (b - a) + a);
    }
    v
}

pub trait LinSpace {
    fn linspace(self, n: usize) -> Vec<f32>
    where
        Self: Sized;
}

impl LinSpace for Range<f32> {
    fn linspace(self, n: usize) -> Vec<f32>
    where
        Self: Sized,
    {
        linsapce(self.start, self.end, n)
    }
}
