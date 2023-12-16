#[test]
#[ignore = "just for testing"]
fn test_bit_operations() {
    let mut left_bitfield = vec![0_u8, 0_u8, 0_u8, 0_u8];
    let right_bitfield = vec![0_u8, 0_u8, 4_u8, 0_u8];

    let index = 21;
    let (byte_index, bit_index) = (index / 8, index % 8);
    left_bitfield[byte_index] |= 128_u8 >> bit_index;

    assert_eq!(left_bitfield, right_bitfield);
}
