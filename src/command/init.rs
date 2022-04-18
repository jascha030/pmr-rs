use std::io::Result;
use crate::command::{Run, Init};

impl Run for Init {
    fn run(&self) -> Result<()> {
        //let mut buffer String::new();
        //let mut stdin = io::stdin();
        //stdin.read_line(&mut buffer)?;
        
        Ok(())
    }
}

