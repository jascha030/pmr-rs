pub mod linked;
pub mod named;
pub mod read;
pub mod write;

use crate::resource::linked::Linked;
use crate::resource::named::Named;
use open;
use serde_derive::{Deserialize, Serialize};
use std::io::Result;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub tasks: Resource,
    pub time: Resource,
    pub git: Resource,
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
