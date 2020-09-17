use std::f32::consts::PI;
use std::ops::{Neg, Shl};

use gui::{CallbackExecutor, GuiCallback, WidgetAdder, WidgetParser};
use tools::*;

use super::align::*;
use super::draw::*;
use super::widget::*;

#[derive(Default, Clone)]
pub struct SkipCell {}

impl_widget_building_for!(SkipCell);
impl Widget for SkipCell {
    fn size(&self) -> Vec2px {
        Vec2px::zero()
    }
}

#[derive(Default, Clone)]
pub struct VertLayoutPriv {
    size: Vec2px,
}

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
pub struct PanelPrivate {
    total_size: Vec2px,
    child_id: u32,
}

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
pub struct GridLayoutPrivate {
    real_size: Vec2px,
    child_id: usize,
    child_pos: Vec2px,
    col_widths_unit: Vec<f32>,
    row_heights_unit: Vec<f32>,
}

#[derive(Default, Clone)]
pub struct GridLayout {
    pub col_widths: Vec<GuiDimension>,
    pub row_heights: Vec<GuiDimension>,
    pub size: WidgetSize,
    pub private: GridLayoutPrivate,
}

impl_widget_building_for!(GridLayout);
impl Widget for GridLayout {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
        if self.row_heights.is_empty() {
            self.row_heights.push(GuiDimension::Relative(1.0));
        }
        if self.col_widths.is_empty() {
            self.col_widths.push(GuiDimension::Relative(1.0));
        }
        let s = self.size();
        let tot_rel_w: f32 = self.col_widths.iter().map(|d| d.relative()).sum();
        let tot_abs_w: f32 = self.col_widths.iter().map(|d| d.absolute()).sum();
        let tot_rel_h: f32 = self.row_heights.iter().map(|d| d.relative()).sum();
        let tot_abs_h: f32 = self.row_heights.iter().map(|d| d.absolute()).sum();
        let unit_per_rel_w = if tot_rel_w == 0.0 {
            self.private.real_size.x = tot_abs_w;
            1.0
        } else {
            (s.x - tot_abs_w) / tot_rel_w
        };
        let unit_per_rel_h = if tot_rel_h == 0.0 {
            self.private.real_size.y = tot_abs_h;
            1.0
        } else {
            (s.y - tot_abs_h) / tot_rel_h
        };
        self.private.col_widths_unit = self
            .col_widths
            .iter()
            .map(|w| w.to_units(unit_per_rel_w))
            .collect();
        self.private.row_heights_unit = self
            .row_heights
            .iter()
            .map(|h| h.to_units(unit_per_rel_h))
            .collect();
    }
    fn place_child(&mut self, _child_size: Vec2px, _child_descent: f32) -> WidgetPosition {
        let p = self.private.child_pos;

        if (self.private.child_id + 1) % self.col_widths.len() == 0 {
            self.private.child_pos.x = 0.0;
            self.private.child_pos.y +=
                self.private.row_heights_unit[self.private.child_id / self.col_widths.len()];
        } else {
            self.private.child_pos.x +=
                self.private.col_widths_unit[self.private.child_id % self.col_widths.len()];
        }
        self.private.child_id += 1;
        p.into()
    }
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: Vec2px::new(
                self.private.col_widths_unit[self.private.child_id % self.col_widths.len()],
                self.private.row_heights_unit[self.private.child_id / self.col_widths.len()],
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

#[derive(Debug, Default, Clone)]
pub struct PaddingPrivate {
    all_size: Vec2px,
    stacking_depth: f32,
}

#[derive(Debug, Default, Clone)]
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
                FontSize::Em(x) => x * 20.0 * gui_scale,
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

#[derive(Default, Clone)]
pub struct TextPrivate {
    real_size: Vec2px,
}

#[derive(Clone)]
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
            self.size(),
            self.color,
            self.align,
            self.font_size.to_pixels(self.size().minxy(), 1.0),
        );
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Default, Clone)]
pub struct ButtonPrivate {
    state: ButtonState,
    real_size: Vec2px,
}

#[derive(Clone)]
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
            font: "sans-serif".to_owned(),
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
    fn child_constraint(&self) -> Option<WidgetConstraints> {
        Some(WidgetConstraints {
            max_size: self.size(),
        })
    }
    fn on_press(
        &mut self,
        _local_cursor_pos: Vec2px,
        _executor: &mut CallbackExecutor,
    ) -> EventResponse {
        self.private.state = ButtonState::Pressed;
        EventResponse::HandledRedraw
    }
    fn on_release(&mut self, executor: &mut CallbackExecutor) -> EventResponse {
        executor.execute(&self.callback, self);
        self.private.state = ButtonState::Hovered;
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
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        build_draw_for_button(
            builder,
            self.background.clone(),
            self.private.state,
            self.size(),
        );
        builder.add_text(
            &self.text,
            &self.font,
            self.size(),
            self.text_color,
            Default::default(),
            self.font_size.to_pixels(self.size().minxy(), 1.0),
        );
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

fn build_draw_for_button(
    builder: &mut DrawBuilder,
    background: ButtonBckg,
    state: ButtonState,
    size: Vec2px,
) {
    let clr = match background {
        ButtonBckg::Cirlce(c) | ButtonBckg::Fill(c) | ButtonBckg::RoundRect(c, _) => {
            let intensity = c.intensity();
            let bckg_clr_direction = if intensity < 0.5 {
                Vec4::new(1.0, 1.0, 1.0, c.w)
            } else {
                Vec4::new(0.0, 0.0, 0.0, c.w)
            };
            let bckg_clr = match state {
                ButtonState::Normal => c,
                ButtonState::Hovered => c * 0.9 + bckg_clr_direction * 0.1,
                ButtonState::Pressed => c * 0.95 + bckg_clr_direction * 0.05,
            };
            bckg_clr
        }
        ButtonBckg::Image(_, c_normal, c_hovered, c_pressed) => match state {
            ButtonState::Normal => c_normal,
            ButtonState::Hovered => c_hovered,
            ButtonState::Pressed => c_pressed,
        },
        ButtonBckg::None => {
            return;
        }
    };
    match background {
        ButtonBckg::Cirlce(_) => {
            let radius = size.minxy();
            let offset = size / 2.0;
            let cirlce = |r| Vec2px::pol(radius, r * PI * 2.0) + size / 2.0 + offset;
            builder.add_clr_convex_fun(cirlce, clr, (radius * 2.0 * PI).floor() as usize, true);
        }
        ButtonBckg::Fill(_) => {
            builder.add_clr_rect(Rect::from_min_max(Vec2::origin(), size.as_vec2()), clr);
        }
        ButtonBckg::RoundRect(_, radius) => {
            let n = ((radius * 2.0 * PI / 4.0).ceil() * 4.0) as usize;
            let mut pts = Vec::with_capacity(n);
            let s = size / 2.0 - Vec2px::new_xy(radius);
            let mut r0 = 0.0;

            for offset in [
                Vec2px::new(1.0, 1.0),
                Vec2px::new(-1.0, 1.0),
                Vec2px::new(1.0, -1.0),
                Vec2px::new(-1.0, -1.0),
            ]
            .iter()
            {
                let circle_mid = size / 2.0 + s * *offset;

                for i in 0..n / 4 {
                    let t = i as f32 / (n / 4) as f32;
                    pts.push(Vec2px::pol(radius, r0 + t * PI / 2.0) + circle_mid);
                }

                r0 += PI / 2.0;
            }
            builder.add_clr_convex(pts, clr, n, false);
        }
        ButtonBckg::Image(name, _, _, _) => {
            builder.add_tex_rect(
                Rect::from_min_max(Vec2::origin(), size.as_vec2()),
                Rect::unit(),
                &name,
                clr,
            );
        }
        ButtonBckg::None => {}
    }
}

#[derive(Default, Copy, Clone)]
pub struct LinearBarPrivate {
    state: ButtonState,
    real_size: Vec2px,
}

#[derive(Clone)]
pub struct LinearBar {
    pub value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub size: WidgetSize,
    pub background: ButtonBckg,
    pub callback: GuiCallback<LinearBar>,
    pub private: LinearBarPrivate,
}

impl Default for LinearBar {
    fn default() -> LinearBar {
        LinearBar {
            value: 0.5,
            minimum: 0.0,
            maximum: 1.0,
            size: Default::default(),
            background: ButtonBckg::Fill(Vec4::grey(0.1)),
            callback: Default::default(),
            private: Default::default(),
        }
    }
}

impl_widget_building_for!(LinearBar);
impl Widget for LinearBar {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn on_press(
        &mut self,
        local_cursor_pos: Vec2px,
        executor: &mut CallbackExecutor,
    ) -> EventResponse {
        self.update_value(local_cursor_pos.x);
        executor.execute(&self.callback, &self);

        self.private.state = ButtonState::Pressed;
        EventResponse::HandledRedraw
    }
    fn on_release(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        self.private.state = ButtonState::Hovered;
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
    fn on_cursor_move(
        &mut self,
        local_cursor_pos: Vec2px,
        executor: &mut CallbackExecutor,
    ) -> EventResponse {
        match self.private.state {
            ButtonState::Pressed => {
                self.update_value(local_cursor_pos.x);
                executor.execute(&self.callback, &self);

                EventResponse::HandledRedraw
            }
            _ => EventResponse::Pass,
        }
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        build_draw_for_button(
            builder,
            self.background.clone(),
            ButtonState::Normal,
            self.size(),
        );
        build_draw_for_button(
            builder,
            self.background.clone(),
            self.private.state,
            self.size() * Vec2px::new(self.ratio(), 1.0),
        );
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

impl LinearBar {
    pub fn ratio(&self) -> f32 {
        (self.value - self.minimum) / (self.maximum - self.minimum)
    }

    pub fn new_colored(
        value: f32,
        min: f32,
        max: f32,
        color: Vec4,
        callback: GuiCallback<LinearBar>,
    ) -> LinearBar {
        LinearBar {
            value,
            minimum: min,
            maximum: max,
            background: ButtonBckg::Fill(color),
            callback,
            ..Default::default()
        }
    }

    fn update_value(&mut self, curosr_x: f32) {
        let ratio = curosr_x / self.size().x;
        let ratio = if ratio < 0.0 {
            0.0
        } else if ratio > 1.0 {
            1.0
        } else {
            ratio
        };

        self.value = ratio * (self.maximum - self.minimum) + self.minimum;
    }
}

#[derive(Default, Clone)]
pub struct ImagePrivate {
    real_size: Vec2px,
}

#[derive(Default, Clone)]
pub struct Image {
    pub size: WidgetSize,
    pub name: String,
    pub cutout: Rect,
    pub rotation: f32,
    pub private: ImagePrivate,
}

impl_widget_building_for!(Image);
impl Widget for Image {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let size = self.size().to_pixels(1.0);
        builder.add_tex_rect_rot(
            Rect::from_min_max(Vec2::origin(), size),
            self.cutout,
            &self.name,
            Vec4::WHITE,
            self.rotation,
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

#[derive(Default, Clone)]
pub struct SquarePrivate {
    inner_size: Vec2px,
    outer_size: Vec2px,
    stacking_depth: f32,
}

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
pub struct OuterSquarePrivate {
    inner_size: Vec2px,
    outer_size: Vec2px,
    stacking_depth: f32,
}

#[derive(Default, Clone)]
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

#[derive(Clone)]
pub struct OverlayPrivate {
    size: Vec2px,
    stacking_depth: f32,
}

#[derive(Clone)]
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

#[derive(Default, Clone)]
pub struct LinesPrivate {
    real_size: Vec2px,
}

#[derive(Default, Clone)]
pub struct Lines {
    pub size: WidgetSize,
    pub lines: Vec<(GuiPoint, GuiPoint)>,
    pub color: Vec4,
    pub private: LinesPrivate,
}

impl_widget_building_for!(Lines);
impl Widget for Lines {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let s = self.size();
        for (a, b) in &self.lines {
            builder.add_line_strip(vec![a.to_units(s), b.to_units(s)], self.color);
        }
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

#[allow(unused_must_use)]
pub mod gui_primitives {
    use super::*;
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::iter::FromIterator;

    pub fn build_table<S, T>(height: f32, table: &HashMap<S, T>)
    where
        S: Display + PartialOrd,
        T: Display + PartialOrd,
    {
        build_table_proto(height, table, Default::default(), Default::default());
    }

    pub fn build_table_proto<S, T>(
        height: f32,
        table: &HashMap<S, T>,
        name_text_proto: Text,
        val_text_proto: Text,
    ) where
        S: Display + PartialOrd,
        T: Display + PartialOrd,
    {
        let items = table.len();
        -GridLayout {
            row_heights: vec![GuiDimension::Units(height); items],
            col_widths: vec![GuiDimension::Relative(1.0), GuiDimension::Relative(1.0)],
            ..Default::default()
        } << {
            let mut vec = Vec::from_iter(table.iter());
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for (name, val) in vec.iter() {
                -Text {
                    text: format!("{}", name),
                    ..name_text_proto.clone()
                };
                -Text {
                    text: format!("{}", val),
                    ..val_text_proto.clone()
                };
            }
        };
    }
}

#[derive(Default, Clone)]
pub struct SplinePrivate {
    real_size: Vec2px,
    hover: Option<usize>,
    grab: Option<usize>,
    grab_offset: Vec3,
}

#[derive(Clone, Default)]
pub struct SplineEditor {
    pub color: Vec4,
    pub size: WidgetSize,
    pub private: SplinePrivate,
    pub points: Vec<Vec3>,
    pub callback: GuiCallback<SplineEditor>,
}

impl_widget_building_for!(SplineEditor);
impl Widget for SplineEditor {
    fn constraint(&mut self, self_constraint: WidgetConstraints) {
        self.private.real_size = self.size.to_units(self_constraint.max_size);
    }
    fn on_press(
        &mut self,
        local_cursor_pos: Vec2px,
        executor: &mut CallbackExecutor,
    ) -> EventResponse {
        let p = Vec3::new(local_cursor_pos.x, 0.0, local_cursor_pos.y);

        let (j, d, o) = self.closest_to(p);

        if d < 15.0 {
            self.set_point(j, p + o);
            executor.execute(&self.callback, &self);
            self.private.grab = Some(j);
            self.private.grab_offset = o;
            EventResponse::HandledRedraw
        } else {
            EventResponse::Handled
        }
    }
    fn on_release(&mut self, _executor: &mut CallbackExecutor) -> EventResponse {
        self.private.grab = None;
        self.private.hover = None;
        EventResponse::Handled
    }

    fn on_cursor_move(
        &mut self,
        local_cursor_pos: Vec2px,
        executor: &mut CallbackExecutor,
    ) -> EventResponse {
        let p = Vec3::new(local_cursor_pos.x, 0.0, local_cursor_pos.y);

        if let Some(id) = self.private.grab {
            self.set_point(id, p + self.private.grab_offset);
            executor.execute(&self.callback, &self);
            EventResponse::HandledRedraw
        } else {
            let (j, d, _) = self.closest_to(p);
            let hov = if d < 15.0 { Some(j) } else { None };

            let resp = if hov != self.private.hover {
                EventResponse::HandledRedraw
            } else {
                EventResponse::Handled
            };

            self.private.hover = hov;
            resp
        }
    }
    fn on_draw_build(&self, builder: &mut DrawBuilder) {
        let spline = Spline::fit_cubic((0.0..1.0).linspace(self.points.len()), self.points.clone());
        let pts = spline
            .quantize(100)
            .iter()
            .map(|p| Vec2px::from_pixels(p.xz(), 1.0))
            .collect();

        // println!("{:?}", pts);

        builder.add_clr_rect(
            Rect::from_pos_size(Vec2::origin(), self.size().to_pixels(1.0)),
            Vec4::WHITE.with_w(0.2),
        );
        builder.add_line_strip(pts, self.color);
        for i in 0..self.points.len() {
            let p = self.points[i];
            let c = if self.private.grab == Some(i) {
                Vec4::grey(0.4)
            } else if self.private.grab == None && self.private.hover == Some(i) {
                Vec4::grey(0.6)
            } else {
                Vec4::WHITE
            };

            builder.add_tex(Vec2px::from_pixels(p.xz(), 1.0), "images/dot", c, 1.0 / 6.0);
        }
    }
    fn size(&self) -> Vec2px {
        self.private.real_size
    }
}

impl SplineEditor {
    fn set_point(&mut self, id: usize, mut pos: Vec3) {
        let s = self.size();
        if pos.x < 0.0 {
            pos.x = 0.0;
        }
        if pos.z < 0.0 {
            pos.z = 0.0;
        }
        if pos.x > s.x {
            pos.x = s.x;
        }
        if pos.z > s.y {
            pos.z = s.y;
        }
        self.points[id] = pos;
    }

    fn closest_to(&self, p: Vec3) -> (usize, f32, Vec3) {
        let mut d = (self.points[0] - p).length();
        let mut o = self.points[0] - p;
        let mut j = 0;
        for i in 1..self.points.len() {
            let dc = (self.points[i] - p).length();
            if dc < d {
                j = i;
                d = dc;
                o = self.points[i] - p
            }
        }

        (j, d, o)
    }
}
