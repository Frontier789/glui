extern crate rand;

use self::rand::Rng;
use std::ops::Range;
use tools::Vec2;

pub fn linsapce(a: f32, b: f32, n: usize) -> Vec<f32> {
    let mut v = vec![];
    for i in 0..n {
        v.push(i as f32 / ((n - 1) as f32) * (b - a) + a);
    }
    v
}

pub fn linsapce_f64(a: f64, b: f64, n: usize) -> Vec<f64> {
    let mut v = vec![];
    for i in 0..n {
        v.push(i as f64 / ((n - 1) as f64) * (b - a) + a);
    }
    v
}

pub trait LinSpace<T> {
    fn linspace(self, n: usize) -> Vec<T>
    where
        Self: Sized;
}

impl LinSpace<f32> for Range<f32> {
    fn linspace(self, n: usize) -> Vec<f32>
    where
        Self: Sized,
    {
        linsapce(self.start, self.end, n)
    }
}
impl LinSpace<f64> for Range<f64> {
    fn linspace(self, n: usize) -> Vec<f64>
    where
        Self: Sized,
    {
        linsapce_f64(self.start, self.end, n)
    }
}

pub trait Smoothstep {
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self;
}

impl Smoothstep for f32 {
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        let t = ((self - edge0) / (edge1 - edge0)).min(1.0).max(0.0);
        t * t * (3.0 - 2.0 * t)
    }
}

impl Smoothstep for f64 {
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        let t = ((self - edge0) / (edge1 - edge0)).min(1.0).max(0.0);
        t * t * (3.0 - 2.0 * t)
    }
}

pub trait Randable {
    fn unit_rand() -> Self;
}

impl Randable for f32 {
    fn unit_rand() -> Self {
        rand::thread_rng().gen_range(0.0, 1.0)
    }
}
impl Randable for f64 {
    fn unit_rand() -> Self {
        rand::thread_rng().gen_range(0.0, 1.0)
    }
}
impl Randable for Vec2 {
    fn unit_rand() -> Self {
        Vec2::new(
            f32::unit_rand(),
            f32::unit_rand(),
        )
    }
}
