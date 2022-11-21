use serde_derive::{Serialize, Deserialize};

use crate::resource::Resource;

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


