use std::fs::File;
use std::io::{Result, BufReader, Read};
use std::path::Path;

use crate::config::Config;

use super::{Open, Run};

impl Run for Open {
    fn run(&self) -> Result<()> {
        let valid: bool = Path::new(".pm.toml").exists();

        if valid == true {
            let file: File = File::open(".pm.toml")?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();

            buf_reader.read_to_string(&mut contents)?;
            let config: Config = toml::from_str(&contents).unwrap();

            println!("{:?}", config);
        } else {
            println!("No `.pm.toml` file found in current directory.");
        }

        Ok(())
    }
}
