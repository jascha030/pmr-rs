pub mod linked;
pub mod named;
pub mod read;
pub mod write;

use crate::resource::linked::Linked;
use crate::resource::named::Named;
use open;
use std::io::Result;

pub struct Resource {
    name: String,
    url: String,
}

impl Named for Resource {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl Linked for Resource {
    fn url(&self) -> String {
        self.url.to_string()
    }

    fn open(&self) -> Result<()> {
        open::that(self.url()).unwrap();

        Ok(())
    }
}
