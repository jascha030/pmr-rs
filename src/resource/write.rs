use std::fs::OpenOptions;
use std::io::{Result, Write};

use super::Config;

pub fn write_toml(config: &Config) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(".pm.toml")
        .unwrap();

    let toml_string = toml::to_string(&config).unwrap();

    file.write_all(&toml_string.as_bytes())
        .expect("Unable to write to file");

    Ok(())
}
