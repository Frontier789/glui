use super::Vec2;
use super::Vec3;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new() -> Rect {
        Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }
    
    pub fn unit() -> Rect {
        Rect {
            left: 0.0,
            top: 0.0,
            right: 1.0,
            bottom: 1.0,
        }
    }
    
    pub fn from_pos_size(pos: Vec2, size: Vec2) -> Rect {
        Rect {
            left: pos.x,
            top: pos.y,
            right: pos.x + size.x,
            bottom: pos.y + size.y,
        }
    }
    
    pub fn from_min_max(min: Vec2, max: Vec2) -> Rect {
        Rect {
            left: min.x,
            top: min.y,
            right: max.x,
            bottom: max.y,
        }
    }
    
    pub fn mid(&self) -> Vec2 {
        Vec2::new(
            (self.left + self.right) / 2.0,
            (self.top + self.bottom) / 2.0,
        )
    }
    
    pub fn min_wh(&self) -> f32 {
        f32::min(self.width(), self.height())
    }
    
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }
    
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.right - self.left, self.bottom - self.top)
    }
    
    pub fn offset(self, o: Vec2) -> Rect {
        Rect {
            left: self.left + o.x,
            top: self.top + o.y,
            right: self.right + o.x,
            bottom: self.bottom + o.y,
        }
    }
    
    pub fn triangulate(self) -> Vec<Vec2> {
        vec![
            Vec2::new(self.left, self.top),
            Vec2::new(self.right, self.top),
            Vec2::new(self.right, self.bottom),
            Vec2::new(self.left, self.top),
            Vec2::new(self.right, self.bottom),
            Vec2::new(self.left, self.bottom),
        ]
    }
    
    pub fn triangulate_3d(self) -> Vec<Vec3> {
        vec![
            Vec3::new(self.left, self.top, 0.0),
            Vec3::new(self.right, self.top, 0.0),
            Vec3::new(self.right, self.bottom, 0.0),
            Vec3::new(self.left, self.top, 0.0),
            Vec3::new(self.right, self.bottom, 0.0),
            Vec3::new(self.left, self.bottom, 0.0),
        ]
    }
    
    pub fn contains(&self, p: Vec2) -> bool {
        self.left <= p.x && self.right >= p.x && self.top <= p.y && self.bottom >= p.y
    }
}