use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct CommandError {
    details: String,
}

impl CommandError {
    pub fn new(msg: &str) -> CommandError {
        CommandError {
            details: msg.to_string(),
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CommandError {
    fn description(&self) -> &str {
        &self.details
    }
}
