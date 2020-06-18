extern crate downcast_rs;

use self::downcast_rs::impl_downcast;
use self::downcast_rs::Downcast;
use mecs::{GlutinDeviceEvent, GlutinWindowEvent};
use std::fmt::Debug;
use std::time::Duration;
use tools::camera_parameters::CameraParameters;

pub trait CameraController: Downcast + Debug {
    fn on_window_event(&mut self, _cam: &mut CameraParameters, _event: &GlutinWindowEvent) -> bool {
        false
    }
    fn on_device_event(&mut self, _cam: &mut CameraParameters, _event: &GlutinDeviceEvent) -> bool {
        false
    }
    fn update(&mut self, _cam: &mut CameraParameters, _delta: Duration) {}
    fn init(&self, _cam: &mut CameraParameters) {}
}

impl_downcast!(CameraController);

#[derive(Debug)]
pub struct NoController {}

impl CameraController for NoController {}
