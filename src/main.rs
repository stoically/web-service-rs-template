use error_stack::Result;
use template_web_service::{cli, Config, Error, WebService};

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), Error> {
    let args = cli::parse();

    match args.command {
        cli::Command::Spawn => {
            let web_service = WebService::new().await?;
            web_service.spawn().await?;
        }
        cli::Command::Config { command } => match command {
            cli::Config::GenerateYaml => Config::generate_yaml().await?,
        },
    }

    Ok(())
}
