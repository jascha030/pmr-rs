use std::fs::OpenOptions;
use std::io::{Result, Write};

use super::Resource;

pub fn write_toml(resources: &[Resource]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(".pm.toml")
        .unwrap();

    for res in resources {
        let toml_string = toml::to_string(&res).unwrap();
        file.write_all(&toml_string.as_bytes())
            .expect("Unable to write to file");
    }

    Ok(())
}
