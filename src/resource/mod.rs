pub mod linked;
pub mod named;
pub mod read;

use crate::resource::linked::Linked;
use crate::resource::named::Named;
use open;
use serde_derive::{Deserialize, Serialize};
use std::io::Result;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub tasks: Option<Resource>,
    pub time: Option<Resource>,
    pub git: Option<Resource>,
}

impl Config {
    pub fn new(tasks: Option<Resource>, time: Option<Resource>, git: Option<Resource>) -> Config {
        Config { tasks, time, git }
    }
}

impl Named for Resource {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl Linked for Resource {
    fn value(&self) -> String {
        self.value.to_string()
    }

    fn open(&self) -> Result<()> {
        open::that(self.value()).unwrap();

        Ok(())
    }
}
