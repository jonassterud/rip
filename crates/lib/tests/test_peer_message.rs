use rip_lib::prelude::*;

#[test]
fn test_peer_message() {
    let mut bytes = Vec::new();
    bytes.append(&mut 15_u32.to_be_bytes().to_vec());
    bytes.push(7);
    bytes.append(&mut 10_u32.to_be_bytes().to_vec());
    bytes.append(&mut 20_u32.to_be_bytes().to_vec());
    bytes.append(&mut vec![1, 2, 3, 4, 5, 6]);

    let message = PeerMessage::try_from_bytes(&bytes).unwrap();
    let back_to_bytes = message.as_bytes();

    assert_eq!(PeerMessage::Piece(10, 20, vec![1, 2, 3, 4, 5, 6]), message);
    assert_eq!(bytes, back_to_bytes);
}
