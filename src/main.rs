mod command;
mod resource;
mod config;

use std::io::Result;
use std::panic;

use self::resource::write::write_toml;
use self::resource::{Config, Resource};

fn main() -> Result<()> {
    let config = Config::new(
        Some(Resource {
            name: "ewa".to_string(),
            value: "https://google.com".to_string(),
        }),
        Some(Resource {
            name: "test-time".to_string(),
            value: "https://app.everhour.com".to_string(),
        }),
        None,
    );

    return match write_toml(&config) {
        Ok(_) => Ok(()),
        Err(e) => panic!("No gucci, no bueno {:?}", e),
    };

    //return match Command::parse().run() {
    //    Ok(_) => Ok(()),
    //    Err(e) => panic!("Ewa fakka man niffo {:?}", e),
    //};
}
