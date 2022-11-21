use crate::resource::Resource;
use serde_derive::{Deserialize, Serialize};

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
