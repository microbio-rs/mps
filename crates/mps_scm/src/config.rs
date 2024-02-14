use serde::Deserialize;

use super::github;

#[derive(Debug, Clone, Deserialize)]
pub struct MpsScmConfig {
    pub github: github::GithubConfig,
}

impl MpsScmConfig {
    pub fn load(config_path: &str) -> Result<Self, mps_config::AppConfigError> {
        Ok(mps_config::load(config_path)?)
    }
}
