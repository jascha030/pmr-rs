use super::{Open, Run};
use dialoguer::console::Term;
use std::io::Error;

use crate::config::get_config;
use crate::config::list::List;
use crate::resource::linked::Linked;
use crate::config::choices::Choices;

impl Run for Open {
    fn run(&self) -> Result<(), Error> {
        let all = self.all;

        match get_config() {
            Ok(config) => {
                if all {
                    for res in config.list() {
                        res.open().unwrap();
                    }

                    return Ok(());
                }

                let selection = match dialoguer::Select::new()
                    .items(&config.choices() as &[_])
                    .interact_on_opt(&Term::stderr())
                {
                    Ok(it) => it,
                    Err(err) => panic!("Error: {:?}", err),
                };

                let max: usize = config.count();

                match selection {
                    Some(opt) => match opt == max {
                        true => {
                            for res in config.list() {
                                res.open().unwrap();
                            }
                        }
                        false => {
                            if let Some(r) = config.list().into_iter().nth(opt) {
                                r.open().unwrap()
                            }
                        }
                    },
                    None => panic!("Something went wrong..."),
                }
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
