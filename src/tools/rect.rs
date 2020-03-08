use super::Vec2;

pub struct Rect {
    left: f32,
    up: f32,
    right: f32,
    down: f32,
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
    
    pub fn from_pos_size(pos: Vec2, size: Vec2) -> Rect {
        Rect {
            left: pos.x,
            up: pos.y,
            right: pos.x + size.x,
            down: pos.y + size.y,
        }
    }
    
    pub fn contains(&self, p: Vec2) -> bool {
        self.left <= p.x && self.right >= p.x && self.up <= p.y && self.down >= p.y
    }
}