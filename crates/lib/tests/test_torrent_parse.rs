use rip_lib::prelude::*;

#[test]
fn test_torrent_parse() {
    let path = std::env::current_dir()
        .unwrap()
        .join("./tests/torrents/big-buck-bunny.torrent");
    let contents = std::fs::read(path).unwrap();
    let torrent = Torrent::from_bcode(&contents).unwrap();
    println!("{torrent:?}");
}
