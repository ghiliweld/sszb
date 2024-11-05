mod decode;
mod encode;
mod hash;

pub const BYTES_PER_LENGTH_OFFSET: usize = 4;
pub const N: usize = 1_000;

pub use decode::{
    decode_impls::*, read_offset_from_buf, read_offset_from_slice, sanitize_offset, DecodeError,
    SszDecode,
};
pub use encode::*;
pub use hash::*;
