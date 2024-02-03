use std::{error::Error, fs};

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub seed: u64,
}

impl Config {
    pub fn from_file(path: &String) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&file_content)?)
    }
}
