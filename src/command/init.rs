use crate::command::{Init, Run};
use crate::config::write::write_toml;
use crate::config::Config;
use crate::resource::Resource;

use std::io::Result;

impl Run for Init {
    fn run(&self) -> Result<()> {
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
        //let mut buffer String::new();
        //let mut stdin = io::stdin();
        //stdin.read_line(&mut buffer)?;
    }
}
