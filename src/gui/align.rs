#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Align {
    pub horizontal: HAlign,
    pub vertical: VAlign,
}

impl Align {
    pub fn from(horizontal: HAlign, vertical: VAlign) -> Align {
        Align {
            horizontal,
            vertical,
        }
    }
    pub fn left() -> Align {
        Align {
            horizontal: HAlign::Left,
            vertical: VAlign::Center,
        }
    }
    pub fn right() -> Align {
        Align {
            horizontal: HAlign::Right,
            vertical: VAlign::Center,
        }
    }
    pub fn top() -> Align {
        Align {
            horizontal: HAlign::Center,
            vertical: VAlign::Top,
        }
    }
    pub fn bottom() -> Align {
        Align {
            horizontal: HAlign::Center,
            vertical: VAlign::Bottom,
        }
    }
}

impl Default for Align {
    fn default() -> Align {
        Align::from(HAlign::Center, VAlign::Center)
    }
}
