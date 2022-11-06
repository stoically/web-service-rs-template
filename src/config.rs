//! Configuration types.

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tracing_subscriber::filter::LevelFilter;

/// Configuration.
///
/// We don't use the `config` crate because of type safety. See <https://github.com/mehcode/config-rs/issues/365>.
//
// TODO: Generate the setter methods via macro.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
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

    /// Set the [`HttpConfig::addr`].
    pub fn set_http_addr(mut self, http_addr: SocketAddr) -> Self {
        self.http.addr = http_addr;
        self
    }

    /// Set the [`DatabaseConfig::uri`].
    pub fn set_database_uri(mut self, database_uri: String) -> Self {
        self.database.uri = database_uri;
        self
    }

    /// Set the [`LogConfig::level_filter`].
    pub fn set_log_level_filter(mut self, log_level_filter: LevelFilter) -> Self {
        self.log.level_filter = log_level_filter;
        self
    }
}

/// HTTP configuration.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub uri: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            uri: "postgresql://localhost:5432/template-web-service".to_owned(),
        }
    }
}

/// Logging configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
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
