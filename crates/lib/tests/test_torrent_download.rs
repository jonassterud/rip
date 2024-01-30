use rip_lib::prelude::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "avoid spam"]
async fn test_torrent_download() {
    let this_dir = std::env::current_dir().unwrap().join("./tests");
    let torrent_path = this_dir.join("./torrents/cosmos-laundromat.torrent");
    let out_path = this_dir.join("./torrents");

    let mut agent = Agent::new().unwrap();
    agent.add_torrents(vec![torrent_path]).await.unwrap();
    agent.download(&out_path).await.unwrap();

    std::io::read_to_string(std::io::stdin()).unwrap();
}
