use anyhow::Result;
use hass::Entities;
// counter
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
mod hass;
mod prompts;

#[tokio::main]
async fn main() -> Result<()> {
    // Init tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Rusty MCP Server");

    let service = Entities::new().serve(stdio()).await.inspect_err(|e|{
        tracing::error!("serving error: {:?}",e);
    })?;

    service.waiting().await?;

    Ok(())
}
