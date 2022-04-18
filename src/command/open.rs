use std::io::Result;

use super::{Open, Run};

impl Run for Open {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
