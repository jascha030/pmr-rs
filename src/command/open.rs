use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

use dialoguer::console::Term;

use crate::config::choices::Choices;
use crate::config::list::List;
use crate::config::Config;
use crate::resource::linked::Linked;

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
            let selection = dialoguer::Select::new()
                .items(&config.choices() as &[_])
                .interact_on_opt(&Term::stderr())?;

            let max: usize = config.count();
            match selection {
                Some(opt) => match opt == max {
                    true => {
                        for res in config.list() {
                            res.open().unwrap();
                        }
                    }
                    false => match config.list().into_iter().nth(opt) {
                        Some(r) => r.open().unwrap(),
                        None => (),
                    },
                },
                None => panic!("Something went wrong..."),
            }
        }

        Ok(())
    }
}
