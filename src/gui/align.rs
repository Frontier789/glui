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
}

impl Default for Align {
    fn default() -> Align {
        Align::from(HAlign::Center, VAlign::Center)
    }
}
