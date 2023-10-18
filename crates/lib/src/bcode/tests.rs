use crate::prelude::*;

#[test]
fn bcode_integer() {
    let integer = Value::Integer(Integer(-32));
    let encoded = encode(&integer);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(integer, decoded);
}

#[test]
fn bcode_byte_string() {
    let byte_string = Value::ByteString(ByteString(b"3:abc".to_vec()));
    let encoded = encode(&byte_string);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(byte_string, decoded);
}

#[test]
fn bcode_list() {
    let list = Value::List(List(vec![
        Value::Integer(Integer(-32)),
        Value::ByteString(ByteString(b"3:abc".to_vec())),
    ]));
    let encoded = encode(&list);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(list, decoded);
}

#[test]
fn bcode_dictionary() {
    // TODO
}
