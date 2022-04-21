mod command;
mod resource;

use std::io::Result;
use std::panic;

use self::resource::Resource;
use self::resource::write::write_toml;

fn main() -> Result<()> {
    let arr = [Resource {
        name: "ewa".to_string(),
        url: "https://google.com".to_string(),
    }];

    return match write_toml(&arr) {
        Ok(_) => Ok(()),
        Err(e) => panic!("No gucci, no bueno {:?}", e)
    }

    //return match Command::parse().run() {
    //    Ok(_) => Ok(()),
    //    Err(e) => panic!("Ewa fakka man niffo {:?}", e),
    //};
}
