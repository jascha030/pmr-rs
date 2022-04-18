use crate::command::{Init, Run};
use std::io::Result;

impl Run for Init {
    fn run(&self) -> Result<()> {
        //let mut buffer String::new();
        //let mut stdin = io::stdin();
        //stdin.read_line(&mut buffer)?;

        Ok(())
    }
}
