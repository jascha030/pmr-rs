mod command;
mod resource;

use std::io::Result;
use std::panic;

use self::resource::write::write_toml;
use self::resource::{Config, Resource};

fn main() -> Result<()> {
    let arr = [Resource {
        name: "ewa".to_string(),
        value: "https://google.com".to_string(),
    }];

    let config = Config {
        tasks: Resource {
            name: "test-pm".to_string(),
            value: "http://google.com".to_string(),
        },
        time: Resource {
            name: "test-time".to_string(),
            value: "https://app.everhour.com".to_string(),
        },
    };

    return match write_toml(&arr) {
        Ok(_) => Ok(()),
        Err(e) => panic!("No gucci, no bueno {:?}", e),
    };

    //return match Command::parse().run() {
    //    Ok(_) => Ok(()),
    //    Err(e) => panic!("Ewa fakka man niffo {:?}", e),
    //};
}
