pub mod named;
pub mod linked;

use crate::resource::named::Named;
use crate::resource::linked::Linked;

use std::io::Result;

enum ResourceTypes {}

struct ResourceType {}

pub struct Resource {
    name: String,
    url: String,
}

impl Named for Resource {
    fn name(&self) -> String {
        self.name
    }
}

impl Linked for Resource {
    fn url(&self) -> String {
        self.url
    }

    fn open() -> Result<()> {
       Ok(()) 
    }
}
