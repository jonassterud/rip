use crate::prelude::*;

#[test]
fn bcode_integer() {
    let integer = -32_isize;
    let encoded = encode::<Integer>(&integer);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(Value::from(Integer::from(integer)), decoded);
}

#[test]
fn bcode_byte_string() {
    let byte_string = b"3:abc".to_vec();
    let encoded = encode::<ByteString>(&byte_string);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(Value::from(ByteString::from(byte_string)), decoded);
}

#[test]
fn bcode_list() {
    let v1 = Integer::from(-32_isize);
    let v2 = ByteString::from(b"abc".to_vec());
    let list: Vec<Value> = vec![v1.into(), v2.into()];

    let encoded = encode::<List>(&list);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(Value::from(List::from(list)), decoded);
}

#[test]
fn bcode_dictionary() {
    // TODO
}
