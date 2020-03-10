use super::*;

#[derive(Default)]
pub struct VertLayoutPriv {
    size: Vec2px
}

#[derive(Default)]
pub struct VertLayout {
    pub padding: Vec2px,
    pub private: VertLayoutPriv,
}

impl Widget for VertLayout {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.size.x = self_constraint.max_size.x;
    }
    
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: Vec2px::new(self.private.size.x, std::f32::INFINITY),
        })
    }

    fn place_child(&mut self, child_size: Vec2px) -> Vec2px {
        let y = self.private.size.y;
        self.private.size.y += child_size.y + self.padding.y;
        Vec2px::new(0.0, y)
    }

    fn size(&self) -> Vec2px {
        self.private.size - Vec2px::new(0.0, self.padding.y)
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum PanelDirection {
    Left,
    Right,
    Up,
    Down,
}

impl Default for PanelDirection {
    fn default() -> PanelDirection {
        PanelDirection::Up
    }
}

impl PanelDirection {
    fn rot(self) -> PanelDirection {
        match self {
            PanelDirection::Left => PanelDirection::Up,
            PanelDirection::Up => PanelDirection::Right,
            PanelDirection::Right => PanelDirection::Down,
            PanelDirection::Down => PanelDirection::Left,
        }
    }
}

#[derive(Default)]
pub struct PanelPrivate {
    total_size: Vec2px,
    child_id: u32,
}

#[derive(Default)]
pub struct FixedPanel {
    pub dir: PanelDirection,
    pub size: f32,
    pub private: PanelPrivate,
}

impl Widget for FixedPanel {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.total_size = self_constraint.max_size;
    }
    
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        match self.private.child_id {
            0 => Some(WidgetConstraints {
                max_size: self.occupied_size(false),
            }),
            1 => Some(WidgetConstraints {
                max_size: self.private.total_size - self.occupied_size(true),
            }),
            _ => Some(WidgetConstraints {
                max_size: Vec2px::zero(),
            }),
        }
    }

    fn place_child(&mut self, _child_size: Vec2px) -> Vec2px {
        
        let ci = self.private.child_id;
        self.private.child_id += 1;
        
        match ci {
            0 => {
                match self.dir {
                    PanelDirection::Left | PanelDirection::Up => {
                        Vec2px::zero()
                    },
                    PanelDirection::Right => {
                        Vec2px::new(self.private.total_size.x - self.size, 0.0)
                    },
                    PanelDirection::Down => {
                        Vec2px::new(0.0, self.private.total_size.y - self.size)
                    },
                }
            },
            1 => {
                match self.dir {
                    PanelDirection::Right | PanelDirection::Down => {
                        Vec2px::zero()
                    },
                    PanelDirection::Left => {
                        Vec2px::new(self.size, 0.0)
                    },
                    PanelDirection::Up => {
                        Vec2px::new(0.0, self.size)
                    },
                }
            }
            _ => Vec2px::zero(),
        }
    }

    fn size(&self) -> Vec2px {
        self.private.total_size
    }
}

impl FixedPanel {
    fn occupied_size(&self, zero_perp_size: bool) -> Vec2px {
        
        let s = if zero_perp_size { Vec2px::zero() } else { self.private.total_size };
        
        match self.dir {
            PanelDirection::Left | PanelDirection::Right => { Vec2px::new(self.size, s.y) },
            PanelDirection::Up   | PanelDirection::Down  => { Vec2px::new(s.x, self.size) },
        }
    }
}

#[derive(Default)]
pub struct GridLayoutPrivate {
    real_size: Vec2px,
    child_id: usize,
    child_pos: Vec2px,
    col_widths: Vec<f32>,
    row_heights: Vec<f32>,
}

#[derive(Default)]
pub struct GridLayout {
    pub col_widths: Vec<f32>,
    pub row_heights: Vec<f32>,
    pub size: WidgetSize,
    pub private: GridLayoutPrivate,
}


impl Widget for GridLayout {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
        
        if self.row_heights.is_empty() {
            self.row_heights.push(1.0);
        }
        if self.col_widths.is_empty() {
            self.col_widths.push(1.0);
        }
        
        let s = self.size();
        let sw = s.x / self.col_widths.iter().sum::<f32>() ;
        let sh = s.y / self.row_heights.iter().sum::<f32>();
        
        self.private.row_heights = self.row_heights.iter().map(|h| h * sh).collect();
        self.private.col_widths  = self.col_widths .iter().map(|w| w * sw).collect();
    }
    
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
    
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: Vec2px::new(
                self.private.col_widths[self.private.child_id % self.col_widths.len()],
                self.private.row_heights[self.private.child_id / self.col_widths.len()],
            ),
        })
    }

    fn place_child(&mut self, _child_size: Vec2px) -> Vec2px {
        let p = self.private.child_pos;
        
        if (self.private.child_id+1) % self.col_widths.len() == 0 {
            self.private.child_pos.x = 0.0;
            self.private.child_pos.y += self.private.row_heights[self.private.child_id / self.col_widths.len()];
        } else {
            self.private.child_pos.x += self.private.col_widths[self.private.child_id % self.col_widths.len()];
        }
        
        self.private.child_id += 1;
        
        p
    }

}

enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

impl Default for ButtonState {
    fn default() -> ButtonState {
        ButtonState::Normal
    }
}

#[derive(Default)]
pub struct ButtonPrivate {
    state: ButtonState,
    real_size: Vec2px,
}

#[derive(Default)]
pub struct Button {
    pub text: String,
    pub size: WidgetSize,
    pub private: ButtonPrivate,
}

impl Widget for Button {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
    
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let c = match self.private.state {
            ButtonState::Normal  => Vec3::new(0.5, 0.0, 0.0),
            ButtonState::Hovered => Vec3::new(0.7, 0.2, 0.1),
            ButtonState::Pressed => Vec3::new(0.6, 0.1, 0.0),
        };
        
        builder.add_rectangle(
            Vec2::zero(),
            self.size().to_pixels(1.0),
            c
        );
    }
    
    fn on_release(&mut self) -> EventResponse {
        println!("Button {} was clicked!", self.text);
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    
    fn on_press(&mut self) -> EventResponse {
        self.private.state = ButtonState::Pressed;
        EventResponse::HandledRedraw
    }
    
    fn on_cursor_enter(&mut self) -> EventResponse {
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    
    fn on_cursor_leave(&mut self) -> EventResponse {
        self.private.state = ButtonState::Normal;
        EventResponse::HandledRedraw
    }
}