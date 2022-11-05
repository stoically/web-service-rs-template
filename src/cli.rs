//! CLI.

use clap::Parser;

/// Web service.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

/// Commands.
#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Spawn the HTTP server.
    Spawn,
}

pub fn parse() -> Args {
    Args::parse()
}
