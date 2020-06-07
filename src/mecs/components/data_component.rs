use super::super::Component;
use std::fmt::Debug;

#[derive(Debug)]
pub struct DataComponent<T> {
    pub data: T,
}

impl<T> Component for DataComponent<T> where T: Debug + 'static {}
