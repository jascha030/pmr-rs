pub mod command;
pub mod config;
pub mod resource;

use crate::command::Run;
use clap::Parser;
use std::io::Result;
use std::panic;

use self::command::Command;

fn main() -> Result<()> {
    return match Command::parse().run() {
        Ok(_) => Ok(()),
        Err(e) => panic!("Something went wrong: {:?}", e),
    };
}
