use error_stack::Result;
use template_web_service::{cli, Error, WebService};

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), Error> {
    let args = cli::parse();
    let web_service = WebService::new().await?;

    match args.command {
        cli::Command::Spawn => web_service.spawn().await?,
    }

    Ok(())
}
