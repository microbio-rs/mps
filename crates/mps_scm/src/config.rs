use serde::Deserialize;

use super::github;
use mps_config::{load, AppConfigError};

#[derive(Debug, Clone, Deserialize)]
pub struct MpsScmConfig {
    pub github: github::GithubConfig,
}

impl MpsScmConfig {
    pub fn load(config_path: &str) -> Result<Self, AppConfigError> {
        Ok(load(config_path)?)
    }
}
