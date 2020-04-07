#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone,Copy,Hash)]
pub struct Entity {
    id: usize,
}

impl Entity {
    pub(super) fn from_id(id: usize) -> Entity {
        Entity {
            id
        }
    }
}
