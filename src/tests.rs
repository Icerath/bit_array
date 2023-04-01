use crate::BitArray;

#[test]
fn test_get() {
    for _ in 0..10_000 {
        let int: u64 = rand::random();
        let bit_array = BitArray::from(int);
        let bool_array: [bool; 64] = bit_array.into();
        for i in 0..64usize {
            assert_eq!(bool_array[i], bit_array[i]);
        }
    }
}
#[allow(clippy::needless_range_loop)]
#[test]
#[cfg(feature = "std")]
fn test_as_array() {
    for _ in 0..10_000 {
        let int: u64 = rand::random();
        let bit_array = BitArray::from(int);
        let bool_array: [bool; 64] = bit_array.into();
        assert_eq!(BitArray::from(bool_array), bit_array);
        let mut bytes = format!("{bit_array:?}").into_bytes();
        bytes.reverse();
        for i in 0..64 {
            let byte = (bytes[i] - b'0') != 0;
            assert_eq!(byte, bit_array.get_unchecked(i));
            assert_eq!(byte, *bool_array.get(i).unwrap());
        }
    }
}
