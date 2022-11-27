pub mod command;
pub mod config;
pub mod resource;

use crate::command::{Command, Run};
use clap::Parser;
use colored::Colorize;
use std::process::ExitCode;

fn main() -> ExitCode {
    return match Command::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!(
                "{}",
                format!("Pmr Error: {}", e.to_string().red().bold())
                    .to_string()
                    .red()
            );
            ExitCode::FAILURE
        }
    };
}
