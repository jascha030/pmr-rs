use std::io::Result;

pub trait Linked {
    fn url(&self) -> String;

    fn open() -> Result<()>;
}
