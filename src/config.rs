use std::{fs::{self}, process::exit};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ConfigNote {
    pub route: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ConfigCategory {
    pub name: String,
    pub notes: Vec<ConfigNote>
}

#[derive(Deserialize)]
pub struct ConfigNotes {
    pub categories: Vec<ConfigCategory>
}

#[derive(Deserialize)]
pub struct Config {
    pub notes: ConfigNotes
}

pub fn get_config() -> Config {
    let filename = "settings/settings.toml";
    let contents = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    let data: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Could not read file due to toml configuration`{}` with err `{}`", filename, e);
            exit(1);
        }
    };
    data
}