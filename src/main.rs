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
struct Init {}

#[derive(Parser, Debug)]
struct Open {}

pub trait Run {
    fn run(&self) -> Result<()>;
}

fn main() -> Result<()> {
    //return match Command::parse().run() {
    //    Ok(_) => Ok(()),
    //    Err(e) => Err(TeleportError::new(&e.to_string())),
    //};

    Ok(())
}
