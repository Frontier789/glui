extern crate downcast_rs;
use self::downcast_rs::Downcast;
use self::downcast_rs::impl_downcast;
use std::fmt::Debug;

pub trait Component: Downcast + Debug {
    fn clone(&self) -> Self where Self: Sized;
}

impl_downcast!(Component);
