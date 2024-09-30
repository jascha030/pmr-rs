use clap::Parser;
use std::io::Error;

pub mod add;
pub mod error;
pub mod init;
pub mod open;

#[derive(Parser)]
#[clap(
    name = "PMR",
    author = "Jascha030 <contact@jaschavanaalst.nl>",
    version = "0.1.0",
    about = "Manage Project Management Resources with a TOML file.",
    long_about = None
)]
pub enum Command {
    Init(Init),
    Add(Add),
    Open(Open),
}

#[derive(Parser)]
pub struct Init {}

#[derive(Parser)]
pub struct Add {
    #[clap(short, long)]
    name: Option<String>,
    value: Vec<String>,
}

#[derive(Parser)]
pub struct Open {
    #[clap(short, long)]
    all: bool,
}

pub trait Run {
    fn run(&self) -> Result<(), Error>;
}

impl Run for Command {
    fn run(&self) -> Result<(), Error> {
        match self {
            Command::Init(cmd) => cmd.run(),
            Command::Add(cmd) => cmd.run(),
            Command::Open(cmd) => cmd.run(),
        }
    }
}
