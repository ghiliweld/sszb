use crate::SszDecode;
use bytes::buf::Buf;

pub mod check_impls;

pub enum CheckError {
    BytesInvalid,
}

pub trait SszCheck: SszDecode {
    fn ssz_check(
        fixed_bytes: &mut impl Buf,
        variable_bytes: &mut impl Buf,
    ) -> Result<Self, CheckError>;

    fn check_ssz_bytes(bytes: &[u8]) -> Result<Self, CheckError>;
}
