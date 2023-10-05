use super::*;

#[test]
fn test_custom_struct() {
    let mut parser = ValueParser{
        data: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        i: 0
    };

    assert_eq!(parser.at().unwrap(), &1);
    assert_eq!(parser.take(0..4).unwrap(), &[1, 2, 3, 4]);
    assert_eq!(parser.find(5).unwrap(), 0);
    assert_eq!(parser.find(10).unwrap(), 5);
    assert_eq!(parser.take(1..2).unwrap(), &[6]);
}

#[test]
fn decode_integer() {
    println!("{:?}", Value::from_bytes("i32eX".as_bytes()).unwrap());
}

#[test]
fn decode_byte_string() {
    println!("{:?}", Value::from_bytes("3:abcX".as_bytes()).unwrap());
}

#[test]
fn decode_list() {
    println!("{:?}", Value::from_bytes("li32ei54e3:abc1:ee".as_bytes()).unwrap());
}

#[test]
fn decode_dictionary() {
    println!(
        "{:?}",
        Value::from_bytes("d3:bar4:spam3:fooi42ee".as_bytes()).unwrap()
    );
}

#[test]
fn decode_arbitrary() {
    println!(
        "{:?}",
        Value::from_bytes("l4:abcdi599494949494eld3:bar4:spam3:fooi42eei32eee".as_bytes()).unwrap()
    );
}
