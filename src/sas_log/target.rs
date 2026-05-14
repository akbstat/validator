use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetConfig {
    pub target: Vec<String>,
    pub target_pattern: Vec<String>,
    pub white_list: Vec<String>,
    pub white_list_pattern: Vec<String>,
}

impl TargetConfig {
    pub fn new<P: AsRef<Path>>(filepath: P) -> anyhow::Result<TargetConfig> {
        let bytes = fs::read(filepath)?;
        Ok(serde_json::from_slice::<TargetConfig>(&bytes)?)
    }
}
