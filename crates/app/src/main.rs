mod cli;

use rip_lib::prelude::*;
use cli::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let agent = Agent::new()?;

    if let Some(paths) = args.torrents {
        for path in paths {
            let torrent = Torrent::from_file(&path)?;
            agent.add_torrent(torrent)?;
        }
    }

    agent.download(&args.out).await?;

    Ok(())
}
