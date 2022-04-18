pub mod init;
pub mod open;

use clap::Parser;
use std::io::Result;

#[derive(Parser, Debug)]
#[clap(
    name = "PMR",
    author = "Jascha030 <contact@jaschavanaalst.nl>",
    version = "1.0",
    about = "Manage Project Management Resources with a TOML file.", 
    long_about = None
)]
enum Command {
    Init(Init),
    Open(Open),
}

#[derive(Parser, Debug)]
pub struct Init {}

#[derive(Parser, Debug)]
pub struct Open {
    #[clap(short, long)]
    all: bool,
}

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for Command {
    fn run(&self) -> Result<()> {
        match self {
            Command::Init(cmd) => cmd.run(),
            Command::Open(cmd) => cmd.run()
        }
    }
}

