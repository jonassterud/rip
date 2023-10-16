use rip_lib::prelude::*;

#[tokio::test]
#[ignore = "avoid network spam"]
async fn test_torrent_tracker() {
    // Create agent and add torrent.
    let path = std::env::current_dir()
        .unwrap()
        .join("./tests/torrents/ubuntu-23.04-desktop-amd64.iso.torrent");
    let mut agent = Agent::new().unwrap();
    agent.add_torrents(vec![path]).await.unwrap();
    
}
