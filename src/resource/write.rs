use std::fs::OpenOptions;
use std::io::{Result, Write};

use super::{Config, Resource};

pub fn write_toml(config: &Config) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(".pm.toml")
        .unwrap();

    let arr = [&config.tasks, &config.time, &config.git];

    for res in arr {
        match res {
            Some(r) => {
                let toml_string = toml::to_string(r).unwrap();
                file.write_all(&toml_string.as_bytes())
                    .expect("Unable to write to file");
            }
            None => (),
        }
    }

    Ok(())
}
