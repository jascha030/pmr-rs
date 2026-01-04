use crate::command::{Init, Run};
use crate::config::write::write_toml;
use crate::config::Config;
use crate::resource::Resource;

use colored::Colorize;
use question::{Answer, Question};
use std::io::Result;

fn clear_stdout() {
    print!("{esc}c", esc = 27 as char);
}

fn ucfirst(s: &str) -> String {
    let mut characters = s.chars();

    match characters.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + characters.as_str(),
    }
}

impl Run for Init {
    fn run(&self) -> Result<()> {
        let question: [&str; 3] = [
            "task management",
            "time tracking",
            "version management (git)",
        ];

        let mut answers: Vec<Option<Resource>> = vec![];

        for item in question.into_iter().enumerate() {
            clear_stdout();

            let (_i, type_string): (usize, &str) = item;
            let fmt_question = format!(
                "Do you want to add {}? [{}]",
                type_string.bright_green().bold(),
                "y/n".bright_yellow()
            );

            let answer = Question::new(&fmt_question)
                .default(Answer::YES)
                .yes_no()
                .confirm();

            clear_stdout();

            if answer == Answer::YES {
                println!(
                    "{}",
                    format!("{} url: ", ucfirst(type_string)).bright_yellow()
                );
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer)?;

                answers.push(Some(Resource {
                    name: ucfirst(type_string).trim().to_string(),
                    value: buffer.trim().to_string(),
                }));
            } else {
                answers.push(None)
            }
        }

        let mut iter = answers.into_iter();
        let config = Config::new(
            iter.next().unwrap_or(None),
            iter.next().unwrap_or(None),
            iter.next().unwrap_or(None),
        );

        match write_toml(&config) {
            Ok(_) => {
                println!(
                    "🎉 {} {} {} 🎉",
                    "Resources written to".bright_green(),
                    "`.pm.toml`".bright_cyan().italic(),
                    "successfully!!".bright_green()
                );
                Ok(())
            }
            Err(e) => panic!("Couldn't write config to toml {:?}", e),
        }
    }
}
