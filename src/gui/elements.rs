use super::*;
use std::f32::consts::PI;

#[derive(Default)]
pub struct VertLayoutPriv {
    size: Vec2px,
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
            0 => match self.dir {
                PanelDirection::Left | PanelDirection::Up => Vec2px::zero(),
                PanelDirection::Right => Vec2px::new(self.private.total_size.x - self.size, 0.0),
                PanelDirection::Down => Vec2px::new(0.0, self.private.total_size.y - self.size),
            },
            1 => match self.dir {
                PanelDirection::Right | PanelDirection::Down => Vec2px::zero(),
                PanelDirection::Left => Vec2px::new(self.size, 0.0),
                PanelDirection::Up => Vec2px::new(0.0, self.size),
            },
            _ => Vec2px::zero(),
        }
    }

    fn size(&self) -> Vec2px {
        self.private.total_size
    }
}

impl FixedPanel {
    fn occupied_size(&self, zero_perp_size: bool) -> Vec2px {
        let s = if zero_perp_size {
            Vec2px::zero()
        } else {
            self.private.total_size
        };
        match self.dir {
            PanelDirection::Left | PanelDirection::Right => Vec2px::new(self.size, s.y),
            PanelDirection::Up | PanelDirection::Down => Vec2px::new(s.x, self.size),
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
        let sw = s.x / self.col_widths.iter().sum::<f32>();
        let sh = s.y / self.row_heights.iter().sum::<f32>();
        self.private.row_heights = self.row_heights.iter().map(|h| h * sh).collect();
        self.private.col_widths = self.col_widths.iter().map(|w| w * sw).collect();
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

        if (self.private.child_id + 1) % self.col_widths.len() == 0 {
            self.private.child_pos.x = 0.0;
            self.private.child_pos.y +=
                self.private.row_heights[self.private.child_id / self.col_widths.len()];
        } else {
            self.private.child_pos.x +=
                self.private.col_widths[self.private.child_id % self.col_widths.len()];
        }
        self.private.child_id += 1;
        p
    }
}

#[derive(Default)]
pub struct PaddingPrivate {
    real_size: Vec2px,
}

#[derive(Default)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub private: PaddingPrivate,
}

impl Widget for Padding {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self_constraint.max_size;
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: self.size() - Vec2px::new(self.left + self.right, self.top + self.bottom),
        })
    }

    fn place_child(&mut self, _child_size: Vec2px) -> Vec2px {
        Vec2px::new(self.left, self.top)
    }
}

#[derive(Default)]
pub struct TextPrivate {
    real_size: Vec2px,
}

pub struct Text {
    pub text: String,
    pub color: Vec4,
    pub size: WidgetSize,
    pub align: font::Align,
    pub font: String,
    pub private: TextPrivate,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            text: Default::default(),
            color: Default::default(),
            size: Default::default(),
            align: Default::default(),
            font: "sans-serif".to_owned(),
            private: Default::default(),
        }
    }
}

impl Widget for Text {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        builder.add_text(&self.text, &self.font, self.size().to_pixels(1.0), self.color, self.align);
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

#[derive(PartialEq)]
pub enum ButtonBckg {
    None,
    Fill(Vec4),
    RoundRect(Vec4,f32),
    Cirlce(Vec4),
}

#[derive(Default)]
pub struct ButtonPrivate {
    state: ButtonState,
    real_size: Vec2px,
}

pub struct Button {
    pub size: WidgetSize,
    pub text: String,
    pub text_color: Vec4,
    pub font: String,
    pub background: ButtonBckg,
    pub callback: GuiCallback,
    pub private: ButtonPrivate,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            size: Default::default(),
            text: Default::default(),
            text_color: Vec4::WHITE,
            font: Default::default(),
            background: ButtonBckg::Fill(Vec4::grey(0.1)),
            callback: Default::default(),
            private: Default::default(),
        }
    }
}

impl Widget for Button {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
    fn expand(&self) -> Vec<Box<dyn Widget>> {
        vec![Box::new(Text {
            text: self.text.clone(),
            color: self.text_color,
            font: self.font.clone(),
            ..Default::default()
        })]
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let clr = match self.background {
            ButtonBckg::Cirlce(c) => c,
            ButtonBckg::Fill(c) => c,
            ButtonBckg::RoundRect(c,_r) => c,
            ButtonBckg::None => {
                return;
            }
        };
        
        let intensity = clr.intensity();
        let bckg_clr_direction = if intensity < 0.5 {Vec4::WHITE} else {Vec4::BLACK};
        
        let bckg_clr = match self.private.state {
            ButtonState::Normal => clr,
            ButtonState::Hovered => clr * Vec4::grey(0.9) + bckg_clr_direction * Vec4::grey(0.1),
            ButtonState::Pressed => clr * Vec4::grey(0.95) + bckg_clr_direction * Vec4::grey(0.05),
        };
        
        let size = self.size().to_pixels(1.0);
        
        match self.background {
            ButtonBckg::Cirlce(_) => {
                let radius = size.minxy();
                let offset = size / 2.0;
                let cirlce = |r| {
                    Vec2::pol(radius, r * PI * 2.0) + size / 2.0 + offset
                };
                
                builder.add_clr_convex(cirlce, bckg_clr, (radius * 2.0 * PI).floor() as usize, true);
            },
            ButtonBckg::Fill(_) => {
                builder.add_clr_rect(Rect::from_min_max(Vec2::origin(), size), bckg_clr);
            },
            ButtonBckg::RoundRect(_,radius) => {
                let round_rect = |r| {
                    let s = size / 2.0 - Vec2::new_xy(radius);
                    
                    let circle_mid = size / 2.0 + match r {
                        x if x < 0.25 => s,
                        x if x < 0.5 => s * Vec2::new(-1.0,1.0),
                        x if x < 0.75 => s * Vec2::new(-1.0,-1.0),
                        _ => s * Vec2::new(1.0,-1.0),
                    };
                    
                    Vec2::pol(radius, r * PI * 2.0) + circle_mid
                };
                
                builder.add_clr_convex(round_rect, bckg_clr, (radius * 2.0 * PI).floor() as usize, true);
            },
            ButtonBckg::None => {}
        }
    }
    fn on_release(&mut self, executor: &mut CallbackExecutor) -> EventResponse {
        executor.execute(&self.callback);
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    fn on_press(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Pressed;
        EventResponse::HandledRedraw
    }
    fn on_cursor_enter(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    fn on_cursor_leave(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Normal;
        EventResponse::HandledRedraw
    }
}
