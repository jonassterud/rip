use super::*;

#[test]
fn decode_integer() {
    println!("{:?}", Value::from_bytes("i32e".as_bytes()).unwrap());
}

#[test]
fn decode_byte_string() {
    println!("{:?}", Value::from_bytes("3:abc".as_bytes()).unwrap());
}

#[test]
fn decode_list() {
    println!("{:?}", Value::from_bytes("li32e3:abce".as_bytes()).unwrap());
}

#[test]
fn decode_dictionary() {
    println!(
        "{:?}",
        Value::from_bytes("d3:bar4:spam3:fooi42ee".as_bytes()).unwrap()
    );
}
