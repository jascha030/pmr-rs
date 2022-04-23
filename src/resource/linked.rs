use std::io::Result;

pub trait Linked {
    fn value(&self) -> String;

    fn open(&self) -> Result<()>;
}
