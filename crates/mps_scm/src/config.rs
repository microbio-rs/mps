use std::path::Path;

use serde::Deserialize;

use super::github;

#[derive(Debug, Clone, Deserialize)]
pub struct MpsScmConfig {
    pub repos_path: String,
    pub sample_repo: String,
    pub github: github::GithubConfig,
}

impl MpsScmConfig {
    pub fn load<P: AsRef<Path>>(
        config_path: P,
    ) -> Result<Self, mps_config::AppConfigError> {
        Ok(mps_config::load(config_path)?)
    }
}
