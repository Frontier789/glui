use super::Vec2;

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
    
    pub fn contains(&self, p: Vec2) -> bool {
        self.left <= p.x && self.right >= p.x && self.up <= p.y && self.down >= p.y
    }
}