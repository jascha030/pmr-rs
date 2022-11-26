use crate::resource::Resource;

pub trait List {
    fn count(&self) -> usize;

    fn list(&self) -> Vec<&Resource>;
}
