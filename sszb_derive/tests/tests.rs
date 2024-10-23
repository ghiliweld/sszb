use bytes::buf::{Buf, BufMut};
use itertools::Itertools as _;
use milhouse::List;
use ssz_types::BitList;
use sszb::{DecodeError, SszDecode, SszEncode};
use sszb_derive::{SszbDecode, SszbEncode};
use std::fmt::Debug;

fn assert_encode<T: SszEncode>(item: &T, bytes: &[u8]) {
    assert_eq!(SszEncode::to_ssz(item), bytes);
}

fn assert_decode<T: SszEncode + SszDecode + PartialEq + Debug>(item: &T, bytes: &[u8]) {
    assert_eq!(<T as SszDecode>::from_ssz_bytes(bytes).unwrap(), *item);
}

fn assert_encode_decode<T: SszEncode + SszDecode + PartialEq + Debug>(item: &T, bytes: &[u8]) {
    assert_encode(item, bytes);
    assert_eq!(<T as SszDecode>::from_ssz_bytes(bytes).unwrap(), *item);
}

#[derive(PartialEq, Debug, SszbDecode, SszbEncode)]
struct VariableA {
    a: u16,
    b: u32,
}

type C = typenum::U10;
const N: u16 = 10;

#[derive(PartialEq, Debug, SszbDecode, SszbEncode)]
struct VariableB {
    a: u16,
    b: List<u16, C>,
}

#[derive(PartialEq, Debug, SszbDecode, SszbEncode)]
struct VariableC {
    a: u16,
    c: BitList8,
}

pub type BitList8 = BitList<typenum::U8>;

#[test]
fn struct_tests() {
    let var_a = VariableA { a: 1, b: 32 };

    let bytes = SszEncode::to_ssz(&var_a);
    assert_encode(&var_a, &bytes);
    assert_decode(&var_a, &bytes);
    assert_encode_decode(&var_a, &bytes);

    let list = List::<u16, C>::try_from_iter(0..N).unwrap();
    let list_bytes = SszEncode::to_ssz(&list);
    assert_encode(&list, &list_bytes);
    assert_decode(&list, &list_bytes);
    assert_encode_decode(&list, &list_bytes);

    let btlst = BitList8::with_capacity(8).unwrap();
    let btlst_bytes = SszEncode::to_ssz(&btlst);
    assert_encode(&btlst, &btlst_bytes);
    assert_decode(&btlst, &btlst_bytes);
    assert_encode_decode(&btlst, &btlst_bytes);

    let var_b = VariableB { a: 2, b: list };
    let bytes = SszEncode::to_ssz(&var_b);

    assert_encode(&var_b, &bytes);
    assert_decode(&var_b, &bytes);
    assert_encode_decode(&var_b, &bytes);

    let var_c = VariableC { a: 2, c: btlst };
    let bytes_c = SszEncode::to_ssz(&var_c);

    assert_encode(&var_c, &bytes_c);
    assert_decode(&var_c, &bytes_c);
    assert_encode_decode(&var_c, &bytes_c);

    assert_eq!(
        BitList8::with_capacity(8).unwrap().to_ssz(),
        vec![0b0000_0000, 0b0000_0001],
    );

    let mut b = BitList8::with_capacity(8).unwrap();
    for i in 0..8 {
        b.set(i, true).unwrap();
    }
    assert_eq!(b.to_ssz(), vec![255, 0b0000_0001]);
}

#[test]
fn test_empty_var_b() {
    assert_eq!(
        VariableB::from_ssz_bytes(&[]).is_err_and(|e| e
            == DecodeError::InvalidByteLength {
                len: 0,
                expected: 6
            }),
        true
    );
}

#[test]
fn test_bad_offset_var_b() {
    let bytes = vec![
        2, 0, 89, 0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0,
    ];
    assert_eq!(
        VariableB::from_ssz_bytes(&bytes)
            .is_err_and(|e| e == DecodeError::OffsetsAreDecreasing(89)),
        true
    );
}

#[test]
fn test_invalid_length_var_b() {
    let bytes = vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0];
    assert_eq!(
        VariableB::from_ssz_bytes(&bytes).is_err_and(|e| e
            == DecodeError::InvalidByteLength {
                len: 16,
                expected: 10
            }),
        true
    );
}
