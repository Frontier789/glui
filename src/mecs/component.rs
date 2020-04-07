use crate::downcast_rs::Downcast;
use std::fmt::Debug;

pub trait Component: Downcast + Debug {
    fn clone(&self) -> Self where Self: Sized;
}

impl_downcast!(Component);
