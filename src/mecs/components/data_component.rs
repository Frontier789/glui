use super::super::Component;
use std::fmt::Debug;

#[derive(Debug)]
pub struct DataComponent<T> {
    pub data: T,
}

impl<T> Component for DataComponent<T> where T: Debug + 'static {}

impl<T> DataComponent<T> {
    pub fn new(data: T) -> DataComponent<T> {
        DataComponent { data }
    }
}

impl<T> Default for DataComponent<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
