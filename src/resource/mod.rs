pub mod named;
pub mod linked;

use crate::resource::named::Named;
use crate::resource::linked::Linked;

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

    fn open() -> Result<()> {
       Ok(()) 
    }
}
