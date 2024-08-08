pub mod command;
pub mod config;
pub mod resource;

use crate::command::{Command, Run};
use clap::Parser;
use colored::Colorize;
use std::process::ExitCode;

fn main() -> ExitCode {
    match Command::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "Pmr Error:".red(), e.to_string().red().bold());
            ExitCode::FAILURE
        }
    }
}
