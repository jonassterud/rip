use rip_lib::prelude::*;

#[test]
fn test_peer_message() {
    let mut bytes = Vec::new();
    bytes.append(&mut 13_u32.to_be_bytes().to_vec());
    bytes.push(6);
    bytes.append(&mut 10_u32.to_be_bytes().to_vec());
    bytes.append(&mut 20_u32.to_be_bytes().to_vec());
    bytes.append(&mut 30_u32.to_be_bytes().to_vec());

    let message = PeerMessage::try_from_bytes(&bytes).unwrap();
    let back_to_bytes = message.as_bytes();

    assert_eq!(PeerMessage::Request(10, 20, 30), message);
    assert_eq!(bytes, back_to_bytes);
}