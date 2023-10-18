use rip_lib::prelude::*;

#[tokio::test]
#[ignore = "avoid spam"]
async fn test_torrent_tracker() {
    let this_dir = std::env::current_dir().unwrap().join("./tests");
    let torrent_path = this_dir.join("./torrents/ubuntu-23.04-desktop-amd64.iso.torrent");
    let out_path = this_dir.join("./torrents");

    let mut agent = Agent::new().unwrap();
    agent.add_torrents(vec![torrent_path]).await.unwrap();
    agent.download(&out_path).await.unwrap();
}
