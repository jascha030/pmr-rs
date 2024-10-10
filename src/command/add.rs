use crate::command::{Add, Run};
use std::io::Result;

impl Run for Add {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
