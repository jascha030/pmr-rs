use std::fs::File;
use std::io::{BufReader, Error, Read};

use super::{Open, Run};
use dialoguer::console::Term;

use crate::config::choices::Choices;
use crate::config::list::List;
use crate::config::Config;
use crate::resource::linked::Linked;

fn read_toml_config() -> Result<String, Error> {
    let file: File = File::open(".pm.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_config() -> Result<Config, Error> {
    match read_toml_config() {
        Ok(contents) => {
            let config: Config = toml::from_str(&contents).unwrap();

            Ok(config)
        }
        Err(e) => Err(e)
    }
}

impl Run for Open {
    fn run(&self) -> Result<(), Error> {
        match get_config() {
            Ok(config) => {
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
            Err(e) => return Err(e)
        }

        Ok(())
    }
}
