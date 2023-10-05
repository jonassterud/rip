mod cli;

use cli::*;
use rip_lib::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut agent = Agent::new()?;

    if let Some(paths) = args.torrents {
        agent.add_torrents(paths).await?;
    }

    agent.download(&args.out).await?;

    Ok(())
}
