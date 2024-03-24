use std::{error::Error, fs};

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct NodeConfig {
    pub name: String,
    pub peers: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct RawNodes {
    pub nodes: Vec<NodeConfig>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum NodesConfig {
    #[serde(rename = "raw")]
    Raw(RawNodes),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub seed: u64,
    pub nodes_config: NodesConfig,
}

impl Config {
    pub fn from_file(path: &String) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&file_content)?)
    }
}
