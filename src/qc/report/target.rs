use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TargetConfig {
    pub variable: Vec<Target>,
    pub observation: Vec<Target>,
    pub values: Vec<Target>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Target {
    pub target: String,
    pub hint: String,
}

impl TargetConfig {
    pub fn new<P: AsRef<Path>>(config: P) -> anyhow::Result<TargetConfig> {
        let content = std::fs::read_to_string(config)?;
        Ok(serde_json::from_str(&content)?)
    }
}
