extern crate downcast_rs;
use self::downcast_rs::impl_downcast;
use self::downcast_rs::Downcast;
use std::fmt::Debug;

pub trait Component: Downcast + Debug {}

impl_downcast!(Component);
