use std::io::{BufReader, Error, Read};
use std::fs::File;

use super::Config;

pub fn read_toml_config() -> Result<String, Error> {
    let file: File = File::open(".pm.toml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn get_config() -> Result<Config, Error> {
    match read_toml_config() {
        Ok(contents) => {
            let config: Config = toml::from_str(&contents).unwrap();

            Ok(config)
        }
        Err(e) => Err(e),
    }
}

