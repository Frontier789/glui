use super::Vec2;
use super::Vec3;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Rect {
    pub left: f32,
    pub up: f32,
    pub right: f32,
    pub down: f32,
}

impl Rect {
    pub fn new() -> Rect {
        Rect {
            left: 0.0,
            up: 0.0,
            right: 0.0,
            down: 0.0,
        }
    }
    
    pub fn unit() -> Rect {
        Rect {
            left: 0.0,
            up: 0.0,
            right: 1.0,
            down: 1.0,
        }
    }
    
    pub fn from_pos_size(pos: Vec2, size: Vec2) -> Rect {
        Rect {
            left: pos.x,
            up: pos.y,
            right: pos.x + size.x,
            down: pos.y + size.y,
        }
    }
    
    pub fn from_min_max(min: Vec2, max: Vec2) -> Rect {
        Rect {
            left: min.x,
            up: min.y,
            right: max.x,
            down: max.y,
        }
    }
    
    pub fn mid(&self) -> Vec2 {
        Vec2::new(
            (self.left + self.right) / 2.0,
            (self.up + self.down) / 2.0,
        )
    }
    
    pub fn min_wh(&self) -> f32 {
        f32::min(self.width(), self.height())
    }
    
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    
    pub fn height(&self) -> f32 {
        self.down - self.up
    }
    
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.right - self.left, self.down - self.up)
    }
    
    pub fn offset(self, o: Vec2) -> Rect {
        Rect {
            left: self.left + o.x,
            up: self.up + o.y,
            right: self.right + o.x,
            down: self.down + o.y,
        }
    }
    
    pub fn triangulate(self) -> Vec<Vec2> {
        vec![
            Vec2::new(self.left, self.up),
            Vec2::new(self.right, self.up),
            Vec2::new(self.right, self.down),
            Vec2::new(self.left, self.up),
            Vec2::new(self.right, self.down),
            Vec2::new(self.left, self.down),
        ]
    }
    
    pub fn triangulate_3d(self) -> Vec<Vec3> {
        vec![
            Vec3::new(self.left, self.up, 0.0),
            Vec3::new(self.right, self.up, 0.0),
            Vec3::new(self.right, self.down, 0.0),
            Vec3::new(self.left, self.up, 0.0),
            Vec3::new(self.right, self.down, 0.0),
            Vec3::new(self.left, self.down, 0.0),
        ]
    }
    
    pub fn contains(&self, p: Vec2) -> bool {
        self.left <= p.x && self.right >= p.x && self.up <= p.y && self.down >= p.y
    }
}