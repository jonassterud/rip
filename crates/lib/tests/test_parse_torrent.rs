use rip_lib::prelude::*;

#[test]
fn test_parse_torrent() {
    let path = std::env::current_dir()
        .unwrap()
        .join("./tests/torrents/ubuntu-23.04-desktop-amd64.iso.torrent");
    let contents = std::fs::read(path).unwrap();
    let torrent = Torrent::from_bytes(&contents).unwrap();
    
    println!("{:?}", torrent);
}
