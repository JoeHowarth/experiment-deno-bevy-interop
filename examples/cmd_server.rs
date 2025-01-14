use anyhow::Result;
use deno_bevy_interop::cmd_server::*;

#[tokio::main]
async fn main() -> Result<()> {
    cmd_server().await?;
    Ok(())
}
