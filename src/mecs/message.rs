use crate::downcast_rs::Downcast;
use std::fmt::Debug;

pub trait Message: Downcast + Debug {
    
}

impl_downcast!(Message);
