// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
use std::path::Path;

use config::{Config, Environment, File};
use serde::de::DeserializeOwned;
use tracing::{debug, error, info};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to load configuration: {0}")]
    Load(#[from] config::ConfigError),

    #[error("error not a file: {0}")]
    NotFile(String),

    #[error("error not a toml extension: {0}")]
    NotToml(String),

    #[error("failed convert path to str: {0}")]
    Utf8Error(String),
}

pub fn load<T, P: AsRef<Path>>(config_path: P) -> Result<T, Error>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let path_ref = config_path.as_ref();

    info!("Loading configuration from: {:?}", path_ref);

    if !path_ref.is_file() {
        return Err(Error::NotFile(format!("{path_ref:?}")));
    }

    if let Some(extension) = path_ref.extension() {
        if extension != "toml" {
            return Err(Error::NotToml(format!("{path_ref:?}")));
        }
    }

    if let Some(path) = path_ref.to_str() {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .add_source(Environment::with_prefix("MPS"))
            .build()
            .map_err(Error::Load)?;

        let app_config: T = config.try_deserialize().map_err(Error::Load)?;

        debug!("Configuration loaded successfully");

        Ok(app_config)
    } else {
        Err(Error::Utf8Error(format!("{path_ref:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    // Example usage
    #[derive(Debug, Deserialize)]
    struct AppConfig {
        #[serde(default = "default_host")]
        host: String,
        port: u16,
        #[serde(default = "default_debug")]
        debug: bool,
    }

    fn default_host() -> String {
        "127.0.0.1".to_string()
    }

    fn default_debug() -> bool {
        false
    }

    #[test]
    fn test_load_config_valid() {
        // Create a temporary configuration file for testing
        let config_path = "test_config.toml";
        std::fs::write(
            config_path,
            "host = \"127.0.0.1\"\nport = 8080\ndebug = true",
        )
        .expect("Failed to write test config file");

        // Ensure that loading the configuration is successful
        match load::<AppConfig, _>(config_path) {
            Ok(app_config) => {
                assert_eq!(app_config.host, "127.0.0.1");
                assert_eq!(app_config.port, 8080);
                assert_eq!(app_config.debug, true);
            }
            Err(err) => {
                panic!("Test failed: {:?}", err);
            }
        }

        // Clean up the temporary test configuration file
        std::fs::remove_file(config_path)
            .expect("Failed to remove test config file");
    }

    #[test]
    fn test_load_config_invalid() {
        // Create an invalid temporary configuration file for testing
        let config_path = "test_invalid_config.toml";
        std::fs::write(config_path, "invalid_config")
            .expect("Failed to write test invalid config file");

        // Ensure that loading the configuration results in an error
        match load::<AppConfig, _>(config_path) {
            Ok(_) => {
                panic!("Test failed: Expected an error, but got Ok");
            }
            Err(_) => {
                // Test passed
            }
        }

        // Clean up the temporary test invalid configuration file
        std::fs::remove_file(config_path)
            .expect("Failed to remove test invalid config file");
    }
}
