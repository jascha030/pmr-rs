pub mod linked;
pub mod named;

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
