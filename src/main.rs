use std::path::Path;

use error_stack::Result;
use template_web_service::{cli, Config, Error, WebService};

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), Error> {
    let args = cli::parse();

    match args.command {
        cli::Command::Spawn => {
            let config_path = Path::new("./config.yaml");
            let web_service = if config_path.exists() {
                let config = Config::new_from_yaml(&config_path.to_string_lossy())?;
                WebService::new_with_config(config).await?
            } else {
                WebService::new().await?
            };
            web_service.spawn().await?;
        }
        cli::Command::Config { command } => match command {
            cli::Config::GenerateYaml => Config::generate_yaml().await?,
        },
    }

    Ok(())
}
