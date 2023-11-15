use dirs;
use serde::Deserialize;
use std::path::PathBuf;

use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Test {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub test: Test,
}

impl Default for Setting {
    fn default() -> Self {
        let mut config_path = dirs::config_local_dir().unwrap().join("config.toml");
        if !config_path.exists() {
            config_path = PathBuf::from("./config.toml");
        }
        let mut file = match File::open(config_path) {
            Ok(f) => f,
            Err(e) => panic!("error is config file {}", e),
        };
        let mut str = String::new();
        match file.read_to_string(&mut str) {
            Ok(_) => {}
            Err(e) => panic!("error is config file {}", e),
        };

        toml::from_str(&str).expect("Parsing the configuration file failed")
    }
}

impl Setting {
    pub fn new() -> Self {
        Self::default()
    }
}
