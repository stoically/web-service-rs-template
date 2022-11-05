use clap::{Parser, Subcommand};

/// Shell tasks following the xtask pattern.
///
/// See https://github.com/matklad/cargo-xtask.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

/// Commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    Docker {
        #[clap(subcommand)]
        command: Docker,
    },
}

/// Docker commands.
#[derive(Debug, Subcommand)]
pub enum Docker {
    /// Build the docker image. Requires buildkit. See https://docs.docker.com/build/buildkit/#getting-started.
    Build,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Docker { command } => match command {
            Docker::Build => {
                std::process::Command::new("docker")
                    .args(&["build", "--rm", "-t", "template-web-service:latest", "."])
                    .spawn()?
                    .wait_with_output()?;
            }
        },
    }

    Ok(())
}
