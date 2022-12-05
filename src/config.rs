//! Configuration types.

use std::net::SocketAddr;

use error_stack::{IntoReport, Result, ResultExt};
use sample_config::SampleConfig;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tracing_subscriber::filter::LevelFilter;

use crate::Error;

/// Configuration.
///
/// We don't use the `config` crate because of type safety. See <https://github.com/mehcode/config-rs/issues/365>.
//
// TODO: Generate the setter methods via macro.
#[derive(Debug, Default, Deserialize, Serialize, SampleConfig)]
pub struct Config {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

impl Config {
    /// Construct new [`Config`] with default values.
    #[tracing::instrument]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct new [`Config`] by reading a yaml file from disk.
    pub fn new_from_yaml(path: &str) -> Result<Self, Error> {
        let config = serde_yaml::from_str(
            &std::fs::read_to_string(path)
                .into_report()
                .change_context(Error::Io)?,
        )
        .into_report()
        .change_context(Error::SerdeYaml)?;

        Ok(config)
    }

    /// Write default configuration to `config.yaml` in the current directory.
    pub async fn generate_yaml() -> Result<(), Error> {
        let config = Self::default().generate_sample_yaml();
        tokio::fs::write("./config.yaml", config)
            .await
            .into_report()
            .change_context(Error::Config)?;

        Ok(())
    }
}

/// HTTP configuration.
#[derive(Debug, Deserialize, Serialize, SampleConfig)]
pub struct HttpConfig {
    pub addr: SocketAddr,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            addr: ([0, 0, 0, 0], 10100).into(),
        }
    }
}

/// Database configuration.
#[derive(Debug, Deserialize, Serialize, SampleConfig)]
pub struct DatabaseConfig {
    pub uri: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            uri: "postgresql://postgres:password@localhost:5432/template-web-service".to_owned(),
        }
    }
}

/// Logging configuration.
#[serde_as]
#[derive(Debug, Deserialize, Serialize, SampleConfig)]
pub struct LogConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub level_filter: LevelFilter,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level_filter: LevelFilter::INFO,
        }
    }
}
