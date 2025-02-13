use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct TargetConfig {
    pub data: Vec<String>,
}

impl TargetConfig {
    pub fn new<P: AsRef<Path>>(filepath: P) -> anyhow::Result<TargetConfig> {
        let bytes = fs::read(filepath)?;
        Ok(serde_json::from_slice::<TargetConfig>(&bytes)?)
    }
}
