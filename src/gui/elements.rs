use std::f32::consts::PI;
use std::ops::{Neg, Shl};

use gui::{CallbackExecutor, GuiCallback, WidgetAdder, WidgetParser};
use tools::*;

use super::align::*;
use super::draw::*;
use super::widget::*;

#[derive(Default)]
pub struct VertLayoutPriv {
    size: Vec2px,
}

#[derive(Default)]
pub struct VertLayout {
    pub padding: Vec2px,
    pub private: VertLayoutPriv,
}

impl_widget_building_for!(VertLayout);
impl Widget for VertLayout {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.size.x = self_constraint.max_size.x;
    }
    fn place_child(&mut self, child_size: Vec2px, _child_descent: f32) -> WidgetPosition {
        let y = self.private.size.y;
        self.private.size.y += child_size.y + self.padding.y;
        Vec2px::new(0.0, y).into()
    }

    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: Vec2px::new(self.private.size.x, std::f32::INFINITY),
        })
    }

    fn size(&self) -> Vec2px {
        self.private.size - Vec2px::new(0.0, self.padding.y)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PanelDirection {
    Left,
    Right,
    Top,
    Bottom,
}

impl Default for PanelDirection {
    fn default() -> PanelDirection {
        PanelDirection::Top
    }
}

impl PanelDirection {
    pub fn rot(self) -> PanelDirection {
        match self {
            PanelDirection::Left => PanelDirection::Top,
            PanelDirection::Top => PanelDirection::Right,
            PanelDirection::Right => PanelDirection::Bottom,
            PanelDirection::Bottom => PanelDirection::Left,
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
    pub size: GuiDimension,
    pub private: PanelPrivate,
}

impl_widget_building_for!(FixedPanel);
impl Widget for FixedPanel {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.total_size = self_constraint.max_size;
    }
    fn place_child(&mut self, _child_size: Vec2px, _child_descent: f32) -> WidgetPosition {
        let ci = self.private.child_id;
        self.private.child_id += 1;
        let s = self.size();
        match ci {
            0 => match self.dir {
                PanelDirection::Left | PanelDirection::Top => Vec2px::zero(),
                PanelDirection::Right => Vec2px::new(s.x - self.size.to_units(s.x), 0.0),
                PanelDirection::Bottom => Vec2px::new(0.0, s.y - self.size.to_units(s.y)),
            },
            1 => match self.dir {
                PanelDirection::Right | PanelDirection::Bottom => Vec2px::zero(),
                PanelDirection::Left => Vec2px::new(self.size.to_units(s.x), 0.0),
                PanelDirection::Top => Vec2px::new(0.0, self.size.to_units(s.y)),
            },
            _ => Vec2px::zero(),
        }
        .into()
    }

    fn child_constraint(&self) -> Option<WidgetConstraints> {
        let ci = self.private.child_id;

        match ci {
            0 | 1 => Some(WidgetConstraints {
                max_size: self.child_space(ci),
            }),
            _ => Some(WidgetConstraints {
                max_size: Vec2px::zero(),
            }),
        }
    }

    fn size(&self) -> Vec2px {
        self.private.total_size
    }
}

impl FixedPanel {
    fn child_space(&self, ci: u32) -> Vec2px {
        let s = self.size();
        let px = self.size.to_units(s.x);
        let py = self.size.to_units(s.y);

        use self::PanelDirection::{Bottom, Left, Right, Top};

        match (self.dir, ci) {
            (Left, 0) | (Right, 0) => Vec2px::new(px, s.y),
            (Left, 1) | (Right, 1) => Vec2px::new(s.x - px, s.y),
            (Top, 0) | (Bottom, 0) => Vec2px::new(s.x, py),
            (Top, 1) | (Bottom, 1) => Vec2px::new(s.x, s.y - py),
            _ => Vec2px::zero(),
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

impl_widget_building_for!(GridLayout);
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
    fn place_child(&mut self, _child_size: Vec2px, _child_descent: f32) -> WidgetPosition {
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
        p.into()
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: Vec2px::new(
                self.private.col_widths[self.private.child_id % self.col_widths.len()],
                self.private.row_heights[self.private.child_id / self.col_widths.len()],
            ),
        })
    }

    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaddingValue {
    Default,
    Relative(f32),
    Units(f32),
}

impl Default for PaddingValue {
    fn default() -> PaddingValue {
        PaddingValue::Default
    }
}

impl PaddingValue {
    pub fn to_units(self, size: f32) -> f32 {
        match self {
            PaddingValue::Default => 0.0,
            PaddingValue::Relative(r) => size * r,
            PaddingValue::Units(x) => x,
        }
    }
}

#[derive(Debug, Default)]
pub struct PaddingPrivate {
    all_size: Vec2px,
    stacking_depth: f32,
}

#[derive(Debug, Default)]
pub struct Padding {
    pub left: PaddingValue,
    pub right: PaddingValue,
    pub top: PaddingValue,
    pub bottom: PaddingValue,
    pub private: PaddingPrivate,
}

impl_widget_building_for!(Padding);
impl Widget for Padding {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.all_size = self_constraint.max_size;
    }
    fn place_child(&mut self, child_size: Vec2px, child_descent: f32) -> WidgetPosition {
        let s = self.size();
        let sd = self.private.stacking_depth;
        self.private.stacking_depth += child_descent + 0.01;

        let padx = self.left.to_units(s.x);
        let pady = self.top.to_units(s.y);

        WidgetPosition::new(
            (s - self.pad_size() - child_size) / 2.0 + Vec2px::new(padx, pady),
            sd,
        )
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: self.size() - self.pad_size(),
        })
    }

    fn size(&self) -> Vec2px {
        self.private.all_size
    }
}

impl Padding {
    fn pad_size(&self) -> Vec2px {
        let s = self.size();
        Vec2px::new(
            self.left.to_units(s.x) + self.right.to_units(s.x),
            self.top.to_units(s.y) + self.bottom.to_units(s.y),
        )
    }
    pub fn absolute(amount: f32) -> Padding {
        Padding {
            left: PaddingValue::Units(amount),
            right: PaddingValue::Units(amount),
            top: PaddingValue::Units(amount),
            bottom: PaddingValue::Units(amount),
            ..Default::default()
        }
    }
    pub fn units(left: f32, right: f32, top: f32, bottom: f32) -> Padding {
        Padding {
            left: PaddingValue::Units(left),
            right: PaddingValue::Units(right),
            top: PaddingValue::Units(top),
            bottom: PaddingValue::Units(bottom),
            ..Default::default()
        }
    }
    pub fn ratios(left: f32, right: f32, top: f32, bottom: f32) -> Padding {
        Padding {
            left: PaddingValue::Relative(left),
            right: PaddingValue::Relative(right),
            top: PaddingValue::Relative(top),
            bottom: PaddingValue::Relative(bottom),
            ..Default::default()
        }
    }
    pub fn relative(ratio: f32) -> Padding {
        Padding {
            left: PaddingValue::Relative(ratio),
            right: PaddingValue::Relative(ratio),
            top: PaddingValue::Relative(ratio),
            bottom: PaddingValue::Relative(ratio),
            ..Default::default()
        }
    }
    pub fn relative_x(ratio: f32) -> Padding {
        Padding {
            left: PaddingValue::Relative(ratio),
            right: PaddingValue::Relative(ratio),
            top: PaddingValue::Default,
            bottom: PaddingValue::Default,
            ..Default::default()
        }
    }
    pub fn relative_y(ratio: f32) -> Padding {
        Padding {
            left: PaddingValue::Default,
            right: PaddingValue::Default,
            top: PaddingValue::Relative(ratio),
            bottom: PaddingValue::Relative(ratio),
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FontSize {
    Em(f32),
    Relative(f32),
    RelativeSteps(f32, (f32, f32), f32),
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize::Em(1.0)
    }
}

impl FontSize {
    pub fn relative_steps(ratio: f32, range: (f32, f32), step: f32) -> Self {
        FontSize::RelativeSteps(ratio, range, step)
    }
    pub fn to_pixels(&self, text_area: f32, gui_scale: f32) -> f32 {
        f32::max(
            match self {
                FontSize::Em(x) => x * 24.0 * gui_scale,
                FontSize::Relative(r) => r * text_area * gui_scale,
                FontSize::RelativeSteps(r, (mn, mx), s) => {
                    let stepped = f32::round(r * text_area / s) * s;
                    num::clamp(stepped, *mn, *mx)
                }
            },
            1.0,
        )
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
    pub align: Align,
    pub font: String,
    pub font_size: FontSize,
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
            font_size: Default::default(),
            private: Default::default(),
        }
    }
}

impl_widget_building_for!(Text);
impl Widget for Text {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        builder.add_text(
            &self.text,
            &self.font,
            self.size().to_pixels(1.0),
            self.color,
            self.align,
            self.font_size.to_pixels(self.size().minxy(), 1.0),
        );
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
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

#[derive(PartialEq, Clone)]
pub enum ButtonBckg {
    None,
    Fill(Vec4),
    Image(String, Vec4, Vec4, Vec4),
    RoundRect(Vec4, f32),
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
    pub font_size: FontSize,
    pub background: ButtonBckg,
    pub callback: GuiCallback<Button>,
    pub private: ButtonPrivate,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            size: Default::default(),
            text: Default::default(),
            text_color: Vec4::WHITE,
            font: Default::default(),
            font_size: Default::default(),
            background: ButtonBckg::Fill(Vec4::grey(0.1)),
            callback: Default::default(),
            private: Default::default(),
        }
    }
}

impl_widget_building_for!(Button);
impl Widget for Button {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn expand(&self) -> Vec<Box<dyn Widget>> {
        vec![Box::new(Text {
            text: self.text.clone(),
            color: self.text_color,
            font: self.font.clone(),
            font_size: self.font_size,
            ..Default::default()
        })]
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: self.size(),
        })
    }
    fn on_press(&mut self, _executor: CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Pressed;
        EventResponse::HandledRedraw
    }
    fn on_release(&mut self, mut executor: CallbackExecutor) -> EventResponse {
        executor.execute(&self.callback, self);
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    fn on_cursor_enter(&mut self, _executor: CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    fn on_cursor_leave(&mut self, _executor: CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Normal;
        EventResponse::HandledRedraw
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let clr = match self.background {
            ButtonBckg::Cirlce(c) | ButtonBckg::Fill(c) | ButtonBckg::RoundRect(c, _) => {
                let intensity = c.intensity();
                let bckg_clr_direction = if intensity < 0.5 {
                    Vec4::new(1.0, 1.0, 1.0, c.w)
                } else {
                    Vec4::new(0.0, 0.0, 0.0, c.w)
                };
                let bckg_clr = match self.private.state {
                    ButtonState::Normal => c,
                    ButtonState::Hovered => c * 0.9 + bckg_clr_direction * 0.1,
                    ButtonState::Pressed => c * 0.95 + bckg_clr_direction * 0.05,
                };
                bckg_clr
            }
            ButtonBckg::Image(_, c_normal, c_hovered, c_pressed) => match self.private.state {
                ButtonState::Normal => c_normal,
                ButtonState::Hovered => c_hovered,
                ButtonState::Pressed => c_pressed,
            },
            ButtonBckg::None => {
                return;
            }
        };
        let size = self.size().to_pixels(1.0);
        match self.background.clone() {
            ButtonBckg::Cirlce(_) => {
                let radius = size.minxy();
                let offset = size / 2.0;
                let cirlce = |r| Vec2::pol(radius, r * PI * 2.0) + size / 2.0 + offset;
                builder.add_clr_convex(cirlce, clr, (radius * 2.0 * PI).floor() as usize, true);
            }
            ButtonBckg::Fill(_) => {
                builder.add_clr_rect(Rect::from_min_max(Vec2::origin(), size), clr);
            }
            ButtonBckg::RoundRect(_, radius) => {
                let round_rect = |r| {
                    let s = size / 2.0 - Vec2::new_xy(radius);

                    let circle_mid = size / 2.0
                        + match r {
                            x if x < 0.25 => s,
                            x if x < 0.5 => s * Vec2::new(-1.0, 1.0),
                            x if x < 0.75 => s * Vec2::new(-1.0, -1.0),
                            _ => s * Vec2::new(1.0, -1.0),
                        };
                    Vec2::pol(radius, r * PI * 2.0) + circle_mid
                };
                builder.add_clr_convex(round_rect, clr, (radius * 2.0 * PI).floor() as usize, true);
            }
            ButtonBckg::Image(name, _, _, _) => {
                builder.add_tex_rect(
                    Rect::from_min_max(Vec2::origin(), size),
                    Rect::unit(),
                    &name,
                    clr,
                );
            }
            ButtonBckg::None => {}
        }
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

#[derive(Default)]
pub struct TogglePrivate {
    state: ButtonState,
    real_size: Vec2px,
}

pub struct Toggle {
    pub on: bool,
    pub size: WidgetSize,
    pub on_text: String,
    pub off_text: String,
    pub text_color: Vec4,
    pub font: String,
    pub font_size: FontSize,
    pub background: ButtonBckg,
    pub callback: GuiCallback<Toggle>,
    pub private: TogglePrivate,
}

impl Default for Toggle {
    fn default() -> Toggle {
        Toggle {
            on: false,
            size: Default::default(),
            on_text: Default::default(),
            off_text: Default::default(),
            text_color: Vec4::WHITE,
            font: Default::default(),
            font_size: Default::default(),
            background: ButtonBckg::Fill(Vec4::grey(0.1)),
            callback: Default::default(),
            private: Default::default(),
        }
    }
}

impl_widget_building_for!(Toggle);
impl Widget for Toggle {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn on_press(&mut self, _executor: CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Pressed;
        EventResponse::HandledRedraw
    }
    fn on_release(&mut self, mut executor: CallbackExecutor) -> EventResponse {
        self.on = !self.on;
        executor.execute(&self.callback, self);
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    fn on_cursor_enter(&mut self, _executor: CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Hovered;
        EventResponse::HandledRedraw
    }
    fn on_cursor_leave(&mut self, _executor: CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Normal;
        EventResponse::HandledRedraw
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let clr = match self.background {
            ButtonBckg::Cirlce(c) | ButtonBckg::Fill(c) | ButtonBckg::RoundRect(c, _) => {
                let intensity = c.intensity();
                let bckg_clr_direction = if intensity < 0.5 {
                    Vec4::new(1.0, 1.0, 1.0, c.w)
                } else {
                    Vec4::new(0.0, 0.0, 0.0, c.w)
                };
                let bckg_clr = match self.private.state {
                    ButtonState::Normal => c,
                    ButtonState::Hovered => c * 0.9 + bckg_clr_direction * 0.1,
                    ButtonState::Pressed => c * 0.95 + bckg_clr_direction * 0.05,
                };
                bckg_clr
            }
            ButtonBckg::Image(_, c_normal, c_hovered, c_pressed) => match self.private.state {
                ButtonState::Normal => c_normal,
                ButtonState::Hovered => c_hovered,
                ButtonState::Pressed => c_pressed,
            },
            ButtonBckg::None => {
                return;
            }
        };
        let size = self.size().to_pixels(1.0);
        match self.background.clone() {
            ButtonBckg::Cirlce(_) => {
                let radius = size.minxy();
                let offset = size / 2.0;
                let cirlce = |r| Vec2::pol(radius, r * PI * 2.0) + size / 2.0 + offset;
                builder.add_clr_convex(cirlce, clr, (radius * 2.0 * PI).floor() as usize, true);
            }
            ButtonBckg::Fill(_) => {
                builder.add_clr_rect(Rect::from_min_max(Vec2::origin(), size), clr);
            }
            ButtonBckg::RoundRect(_, radius) => {
                let round_rect = |r| {
                    let s = size / 2.0 - Vec2::new_xy(radius);

                    let circle_mid = size / 2.0
                        + match r {
                            x if x < 0.25 => s,
                            x if x < 0.5 => s * Vec2::new(-1.0, 1.0),
                            x if x < 0.75 => s * Vec2::new(-1.0, -1.0),
                            _ => s * Vec2::new(1.0, -1.0),
                        };
                    Vec2::pol(radius, r * PI * 2.0) + circle_mid
                };
                builder.add_clr_convex(round_rect, clr, (radius * 2.0 * PI).floor() as usize, true);
            }
            ButtonBckg::Image(name, _, _, _) => {
                builder.add_tex_rect(
                    Rect::from_min_max(Vec2::origin(), size),
                    Rect::unit(),
                    &name,
                    clr,
                );
            }
            ButtonBckg::None => {}
        }

        builder.add_text(
            if self.on {
                &self.on_text
            } else {
                &self.off_text
            },
            &self.font,
            self.size().to_pixels(1.0),
            self.text_color,
            Default::default(),
            self.font_size.to_pixels(self.size().minxy(), 1.0),
        );
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

#[derive(Default)]
pub struct ImagePrivate {
    real_size: Vec2px,
}

#[derive(Default)]
pub struct Image {
    pub size: WidgetSize,
    pub name: String,
    pub cutout: Rect,
    pub private: ImagePrivate,
}

impl_widget_building_for!(Image);
impl Widget for Image {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let size = self.size().to_pixels(1.0);
        builder.add_tex_rect(
            Rect::from_min_max(Vec2::origin(), size),
            self.cutout,
            &self.name,
            Vec4::WHITE,
        );
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

impl Image {
    pub fn from(file: &str) -> Image {
        Image {
            name: file.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct SquarePrivate {
    inner_size: Vec2px,
    outer_size: Vec2px,
    stacking_depth: f32,
}

#[derive(Default)]
pub struct Square {
    pub private: SquarePrivate,
}

impl_widget_building_for!(Square);
impl Widget for Square {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.outer_size = self_constraint.max_size;
        self.private.inner_size = Vec2px::new_xy(self_constraint.max_size.minxy());
    }
    fn place_child(&mut self, child_size: Vec2px, child_descent: f32) -> WidgetPosition {
        let sd = self.private.stacking_depth;
        self.private.stacking_depth += child_descent;

        WidgetPosition::new(self.private.outer_size * 0.5 - child_size * 0.5, sd)
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: self.private.inner_size,
        })
    }

    fn size(&self) -> Vec2px {
        self.private.outer_size
    }
}

#[derive(Default)]
pub struct OuterSquarePrivate {
    inner_size: Vec2px,
    outer_size: Vec2px,
    stacking_depth: f32,
}

#[derive(Default)]
pub struct OuterSquare {
    pub private: OuterSquarePrivate,
}

impl_widget_building_for!(OuterSquare);
impl Widget for OuterSquare {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.inner_size = self_constraint.max_size;
        self.private.outer_size = Vec2px::new_xy(self_constraint.max_size.maxxy());
    }
    fn place_child(&mut self, child_size: Vec2px, child_descent: f32) -> WidgetPosition {
        let sd = self.private.stacking_depth;
        self.private.stacking_depth += child_descent;

        WidgetPosition::new(self.private.inner_size * 0.5 - child_size * 0.5, sd)
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: self.private.inner_size,
        })
    }

    fn size(&self) -> Vec2px {
        self.private.inner_size
    }
}

pub struct OverlayPrivate {
    size: Vec2px,
    stacking_depth: f32,
}

pub struct Overlay {
    pub color: Vec4,
    pub private: OverlayPrivate,
}

impl Default for Overlay {
    fn default() -> Self {
        Overlay {
            color: Vec4::new(0.0, 0.0, 0.0, 0.3),
            private: OverlayPrivate {
                size: Default::default(),
                stacking_depth: 0.01,
            },
        }
    }
}

impl Overlay {
    pub fn from(color: Vec4) -> Overlay {
        Overlay {
            color,
            ..Default::default()
        }
    }
}

impl_widget_building_for!(Overlay);
impl Widget for Overlay {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.size = self_constraint.max_size;
    }
    fn place_child(&mut self, child_size: Vec2px, child_descent: f32) -> WidgetPosition {
        let sd = self.private.stacking_depth;
        self.private.stacking_depth += child_descent;

        WidgetPosition::new(self.private.size * 0.5 - child_size * 0.5, sd)
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: self.private.size,
        })
    }

    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let size = self.size().to_pixels(1.0);
        builder.add_clr_rect(Rect::from_min_max(Vec2::origin(), size), self.color);
    }
    fn size(&self) -> Vec2px {
        self.private.size
    }
}
