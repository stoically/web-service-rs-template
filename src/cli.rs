//! CLI.

use clap::{Parser, Subcommand};

/// Web service.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

/// Commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    Config {
        #[clap(subcommand)]
        command: Config,
    },
    /// Spawn the HTTP server with either default configuration or by reading
    /// `config.yaml` if it exists.
    Spawn,
}

/// Config commands.
#[derive(Debug, Subcommand)]
pub enum Config {
    /// Write default configuration to `config.yaml`.
    GenerateYaml,
}

pub fn parse() -> Args {
    Args::parse()
}
