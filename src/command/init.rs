use crate::command::{Init, Run};
use crate::config::write::write_toml;
use crate::config::Config;
use crate::resource::Resource;

use colored::Colorize;
use question::{Answer, Question};
use std::io::Result;

impl Run for Init {
    fn run(&self) -> Result<()> {
        let question: [&str; 3] = [
            &"task management".blue().bold().to_string(),
            &"time tracking".blue().bold().to_string(),
            &"version management (git)".blue().bold().to_string(),
        ];

        let mut answers: Vec<Option<Resource>> = vec![];

        for item in question.into_iter().enumerate() {
            let (_i, type_string): (usize, &str) = item;
            let fmt_question = format!("Do you want to add {}? [{}]", type_string, "y/n".yellow());
            let answer = Question::new(&fmt_question)
                .default(Answer::YES)
                .yes_no()
                .confirm();

            if answer == Answer::YES {
                println!("{} url: ", type_string);
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer)?;

                answers.push(Some(Resource {
                    name: type_string.to_string(),
                    value: buffer.to_string(),
                }));
            } else {
                answers.push(None)
            }
        }

        let mut iter = answers.into_iter();
        let config = Config::new(
            iter.nth(0).unwrap_or(None),
            iter.nth(1).unwrap_or(None),
            iter.nth(2).unwrap_or(None),
        );

        return match write_toml(&config) {
            Ok(_) => {
                println!("Resources written to `.pm.toml` successfully!");

                Ok(())
            }
            Err(e) => panic!("Couldn't write config to toml {:?}", e),
        };
    }
}
