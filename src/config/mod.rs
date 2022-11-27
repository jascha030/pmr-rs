use self::choices::Choices;
use self::list::List;
use crate::resource::named::Named;
use crate::resource::Resource;
use serde_derive::{Deserialize, Serialize};

pub mod choices;
pub mod list;
pub mod write;

#[derive(Serialize, Deserialize, Debug)]
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

impl Choices for Config {
    fn choices(&self) -> Vec<String> {
        let mut choices: Vec<String> = vec![];

        if self.tasks.is_some() {
            choices.push(self.tasks.as_ref().unwrap().name())
        }

        if self.time.is_some() {
            choices.push(self.time.as_ref().unwrap().name())
        }

        if self.git.is_some() {
            choices.push(self.git.as_ref().unwrap().name())
        }

        if !choices.is_empty() {
            choices.push("All".to_string())
        }

        choices
    }
}

impl List for Config {
    fn count(&self) -> usize {
        return self.list().len();
    }

    fn list(&self) -> Vec<&Resource> {
        let mut list: Vec<&Resource> = vec![];

        if self.tasks.is_some() {
            list.push(self.tasks.as_ref().unwrap())
        }

        if self.time.is_some() {
            list.push(self.time.as_ref().unwrap())
        }

        if self.git.is_some() {
            list.push(self.git.as_ref().unwrap())
        }

        list
    }
}
