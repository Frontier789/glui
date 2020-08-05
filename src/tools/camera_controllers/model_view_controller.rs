use mecs::{
    GlutinButton, GlutinDeviceEvent, GlutinElementState, GlutinKey, GlutinScrollDelta,
    GlutinWindowEvent,
};
use std::collections::HashMap;
use std::f32::consts::PI;
use tools::{CameraController, CameraParameters, CameraSpatialParams, Vec2};

#[derive(Debug)]
pub struct ModelViewController {
    screen_size: Vec2,
    key_pressed: HashMap<GlutinKey, bool>,
    button_pressed: HashMap<GlutinButton, bool>,
    real_spatial: CameraSpatialParams,
    drag_offset: Vec2,
    rotation_direction: Option<Vec2>,
    pub disable_roll: bool,
    pub enable_upside_down: bool,
}

impl CameraController for ModelViewController {
    fn on_window_event(&mut self, cam: &mut CameraParameters, event: &GlutinWindowEvent) -> bool {
        match event {
            GlutinWindowEvent::MouseInput { state, button, .. } => {
                self.button_pressed
                    .insert(*button, *state == GlutinElementState::Pressed);

                if *state == GlutinElementState::Released && *button == GlutinButton::Left {
                    self.rotation_direction = None;

                    if self.is_key_pressed(GlutinKey::LAlt) {
                        self.real_spatial.snap_view(0.9);
                        cam.spatial = self.real_spatial;
                        true
                    } else {
                        false
                    }
                } else if *state == GlutinElementState::Pressed && *button == GlutinButton::Left {
                    self.drag_offset = Vec2::zero();
                    true
                } else {
                    false
                }
            }
            GlutinWindowEvent::MouseWheel { delta, .. } => {
                if let GlutinScrollDelta::LineDelta(_x, y) = delta {
                    let d = self.real_spatial.target_to_pos();
                    let m = 1.1f32.powf(-*y);

                    self.real_spatial.set_pos(self.real_spatial.target + d * m);
                }

                cam.spatial = self.real_spatial;

                true
            }
            _ => false,
        }
    }
    fn on_device_event(&mut self, cam: &mut CameraParameters, event: &GlutinDeviceEvent) -> bool {
        match event {
            GlutinDeviceEvent::Key(input) => {
                if let Some(keycode) = input.virtual_keycode {
                    self.key_pressed
                        .insert(keycode, input.state == GlutinElementState::Pressed);

                    if keycode == GlutinKey::LAlt && input.state == GlutinElementState::Released {
                        self.real_spatial.snap_view(0.9);
                        cam.spatial = self.real_spatial;
                        true
                    } else if keycode == GlutinKey::LAlt && !self.is_key_pressed(GlutinKey::LAlt) {
                        cam.spatial = self.real_spatial;
                        cam.spatial.snap_view(0.9);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            GlutinDeviceEvent::MouseMotion { delta } => {
                let left = self.is_button_pressed(GlutinButton::Left);
                let right = self.is_button_pressed(GlutinButton::Right);
                if left && !right {
                    let mut delta = Vec2::new(delta.0 as f32, delta.1 as f32);
                    if self.disable_roll {
                        delta.x *= self.real_spatial.cos_pitch();
                    }
                    if self.rotation_direction == None {
                        self.drag_offset += delta;
                        if self.drag_offset.length() > 20.0 {
                            let dir = self.drag_offset.sgn();
                            self.rotation_direction =
                                Some(dir.snap_if_close(0.92388, Vec2::base_directions_and_diag()));
                        }
                    }
                    if let Some(dir) = self.rotation_direction {
                        if self.is_key_pressed(GlutinKey::LControl) {
                            delta = dir * delta.dot(dir);
                        }
                    }

                    if self.rotation_direction != None || !self.is_key_pressed(GlutinKey::LControl)
                    {
                        let pitch = self.real_spatial.pitch();
                        let delta_y = if !self.enable_upside_down
                            && -delta.y * 0.01 + pitch < -PI / 2.0 * 0.99
                        {
                            PI / 2.0 * 0.99 + pitch
                        } else if !self.enable_upside_down
                            && -delta.y * 0.01 + pitch > PI / 2.0 * 0.99
                        {
                            -PI / 2.0 * 0.99 + pitch
                        } else {
                            delta.y * 0.01
                        };

                        let d = self.real_spatial.target_to_pos();
                        let d = d.rotate(self.real_spatial.u(), delta.x * 0.01);
                        let d = d.rotate(self.real_spatial.r(), delta_y);

                        self.real_spatial.set_pos(self.real_spatial.target + d);
                    }

                    if self.disable_roll {
                        self.real_spatial.cancel_roll(self.enable_upside_down);
                    }
                }
                if right && !left {
                    let s = self.screen_size.x
                        / ((cam.fov / 2.0).tan() * self.real_spatial.pos_to_target().length()
                            / 2.0);
                    let d = self.real_spatial.r() * delta.0 as f32 / s
                        - self.real_spatial.u() * delta.1 as f32 / s;

                    self.real_spatial.move_by(-d * 4.0);
                }
                if right && left {
                    let d = self.real_spatial.target_to_pos();
                    let m = 1.1f32 * delta.1 as f32 / self.screen_size.y;

                    self.real_spatial.move_by(d * m);
                }

                cam.spatial = self.real_spatial;

                if self.is_key_pressed(GlutinKey::LAlt) {
                    cam.spatial.snap_view(0.9);
                }

                left || right
            }
            _ => false,
        }
    }
    fn init(&self, cam: &mut CameraParameters) {
        cam.spatial = self.real_spatial;
    }
    // fn update(&mut self, cam: &mut CameraParameters, delta: Duration) {
    //     // println!("Updated at {:?}", Instant::now());
    //     let v =
    //         self.velocity.z * cam.v() + self.velocity.y * cam.u() + self.velocity.x * (-cam.l());
    //     cam.move_by(v * self.speed * delta.as_secs_f32());
    //
    //     if self.pressed[&GlutinKey::Q] && !self.pressed[&GlutinKey::E] {
    //         cam.rotate_y(self.speed * delta.as_secs_f32() * 0.5);
    //     }
    //     if !self.pressed[&GlutinKey::Q] && self.pressed[&GlutinKey::E] {
    //         cam.rotate_y(-self.speed * delta.as_secs_f32() * 0.5);
    //     }
    // }
}

impl ModelViewController {
    pub fn spatial_mut(&mut self) -> &mut CameraSpatialParams {
        &mut self.real_spatial
    }
    fn is_key_pressed(&self, key: GlutinKey) -> bool {
        match self.key_pressed.get(&key) {
            Some(pressed) => *pressed,
            None => false,
        }
    }
    fn is_button_pressed(&self, button: GlutinButton) -> bool {
        match self.button_pressed.get(&button) {
            Some(pressed) => *pressed,
            None => false,
        }
    }
    pub fn new(screen_size: Vec2) -> ModelViewController {
        ModelViewController {
            screen_size,
            key_pressed: Default::default(),
            button_pressed: Default::default(),
            real_spatial: Default::default(),
            drag_offset: Vec2::zero(),
            rotation_direction: None,
            disable_roll: false,
            enable_upside_down: false,
        }
    }
}
//
// fn cam_target() -> RenderSequence {
//     let pbuf = Buffer::from_vec(vec![
//         Vec3::origin(),
//         Vec3::new(1.0, 0.0, 0.0),
//         Vec3::origin(),
//         Vec3::new(0.0, 1.0, 0.0),
//         Vec3::origin(),
//         Vec3::new(0.0, 0.0, 1.0),
//     ]);
//     let cbuf = Buffer::from_vec(vec![
//         Vec4::new(1.0, 0.0, 0.0, 1.0),
//         Vec4::new(1.0, 0.0, 0.0, 1.0),
//         Vec4::new(0.0, 1.0, 0.0, 1.0),
//         Vec4::new(0.0, 1.0, 0.0, 1.0),
//         Vec4::new(0.0, 0.0, 1.0, 1.0),
//         Vec4::new(0.0, 0.0, 1.0, 1.0),
//     ]);
//     let mut vao = VertexArray::new();
//     vao.attrib_buffer(0, &pbuf);
//     vao.attrib_buffer(1, &cbuf);
//
//     let mut render_seq = RenderSequence::new();
//
//     render_seq.add_buffer(pbuf.into_base_type());
//     render_seq.add_buffer(cbuf.into_base_type());
//
//     render_seq.add_command(RenderCommand {
//         vao,
//         mode: DrawMode::Lines,
//         shader: DrawShaderSelector::DefaultColored,
//         uniforms: vec![],
//         transparent: false,
//         instances: 1,
//     });
//
//     render_seq
// }
