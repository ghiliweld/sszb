mod decode;
mod encode;
mod ghilhouse_impls;
mod hash;
mod sig;

pub const BYTES_PER_LENGTH_OFFSET: usize = 4;
pub const N: usize = 1_000;

pub use decode::{
    decode_impls::*, read_offset_from_buf, read_offset_from_slice, sanitize_offset, DecodeError,
    SszDecode,
};
pub use encode::*;
pub use hash::SszHash;

pub use ghilhouse_impls::*;
pub use sig::*;
