use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

use dialoguer::console::Term;

use crate::config::Config;
use crate::resource::linked::Linked;
use crate::resource::{self, Resource};

use super::{Open, Run};

impl Run for Open {
    fn run(&self) -> Result<()> {
        let valid: bool = Path::new(".pm.toml").exists();

        if valid == false {
            println!("No `.pm.toml` file found in current directory.");
        } else {
            let file: File = File::open(".pm.toml")?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            let config: Config = toml::from_str(&contents).unwrap();
            let resources: [Option<Resource>; 3] = [config.tasks, config.time, config.git];
            let choices: Vec<&str> = vec!["Task management", "Time Tracking", "Git", "All"];

            let selection = dialoguer::Select::new()
                .items(&choices)
                .default(4)
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(3) => {
                    for res in resources {
                        match res {
                            Some(r) => r.open().unwrap(),
                            None => (),
                        }
                    }
                }
                Some(opt) => match resources.into_iter().nth(opt).unwrap() {
                    Some(r) => r.open().unwrap(),
                    None => (),
                },
                None => panic!("Something went wrong..."),
            }
        }

        Ok(())
    }
}
