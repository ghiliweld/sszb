use crate::BYTES_PER_LENGTH_OFFSET;
use bytes::buf::Buf;

pub mod decode_impls;

// error types and offset decoding code borrowed from the sigma prime team:
// https://github.com/sigp/ethereum_ssz/blob/main/ssz/src/decode.rs#L12
#[derive(Debug, PartialEq, Clone)]
pub enum DecodeError {
    /// The bytes supplied were too short to be decoded into the specified type.
    InvalidByteLength { len: usize, expected: usize },
    /// The given bytes were too short to be read as a length prefix.
    InvalidLengthPrefix { len: usize, expected: usize },
    /// A length offset pointed to a byte that was out-of-bounds (OOB).
    ///
    /// A bytes may be OOB for the following reasons:
    ///
    /// - It is `>= bytes.len()`.
    /// - When decoding variable length items, the 1st offset points "backwards" into the fixed
    /// length items (i.e., `length[0] < BYTES_PER_LENGTH_OFFSET`).
    /// - When decoding variable-length items, the `n`'th offset was less than the `n-1`'th offset.
    OutOfBoundsByte { i: usize },
    /// An offset points “backwards” into the fixed-bytes portion of the message, essentially
    /// double-decoding bytes that will also be decoded as fixed-length.
    ///
    /// https://notes.ethereum.org/ruKvDXl6QOW3gnqVYb8ezA?view#1-Offset-into-fixed-portion
    OffsetIntoFixedPortion(usize),
    /// The first offset does not point to the byte that follows the fixed byte portion,
    /// essentially skipping a variable-length byte.
    ///
    /// https://notes.ethereum.org/ruKvDXl6QOW3gnqVYb8ezA?view#2-Skip-first-variable-byte
    OffsetSkipsVariableBytes(usize),
    /// An offset points to bytes prior to the previous offset. Depending on how you look at it,
    /// this either double-decodes bytes or makes the first offset a negative-length.
    ///
    /// https://notes.ethereum.org/ruKvDXl6QOW3gnqVYb8ezA?view#3-Offsets-are-decreasing
    OffsetsAreDecreasing(usize),
    /// An offset references byte indices that do not exist in the source bytes.
    ///
    /// https://notes.ethereum.org/ruKvDXl6QOW3gnqVYb8ezA?view#4-Offsets-are-out-of-bounds
    OffsetOutOfBounds(usize),
    /// A variable-length list does not have a fixed portion that is cleanly divisible by
    /// `BYTES_PER_LENGTH_OFFSET`.
    InvalidListFixedBytesLen(usize),
    /// Some item has a `ssz_fixed_len` of zero. This is illegal.
    ZeroLengthItem,
    /// The given bytes were invalid for some application-level reason.
    BytesInvalid(String),
}

/// Reads a `BYTES_PER_LENGTH_OFFSET`-byte length from `bytes`, where `bytes.len() >=
/// BYTES_PER_LENGTH_OFFSET`.
pub fn read_offset_from_buf(buf: &mut impl Buf) -> Result<usize, DecodeError> {
    let len = buf.remaining();
    let expected = BYTES_PER_LENGTH_OFFSET;

    if len < expected {
        Err(DecodeError::InvalidLengthPrefix { len, expected })
    } else {
        Ok(buf.get_u32_le() as usize)
    }
}

pub fn read_offset_from_slice(bytes: &[u8]) -> Result<usize, DecodeError> {
    decode_offset(bytes.get(0..BYTES_PER_LENGTH_OFFSET).ok_or(
        DecodeError::InvalidLengthPrefix {
            len: bytes.len(),
            expected: BYTES_PER_LENGTH_OFFSET,
        },
    )?)
}

/// Decode bytes as a little-endian usize, returning an `Err` if `bytes.len() !=
/// BYTES_PER_LENGTH_OFFSET`.
fn decode_offset(bytes: &[u8]) -> Result<usize, DecodeError> {
    let len = bytes.len();
    let expected = BYTES_PER_LENGTH_OFFSET;

    if len != expected {
        Err(DecodeError::InvalidLengthPrefix { len, expected })
    } else {
        let mut array: [u8; BYTES_PER_LENGTH_OFFSET] = std::default::Default::default();
        array.clone_from_slice(bytes);

        Ok(u32::from_le_bytes(array) as usize)
    }
}

/// Performs checks on the `offset` based upon the other parameters provided.
///
/// ## Detail
///
/// - `offset`: the offset bytes (e.g., result of `read_offset(..)`).
/// - `previous_offset`: unless this is the first offset in the SSZ object, the value of the
/// previously-read offset. Used to ensure offsets are not decreasing.
/// - `num_bytes`: the total number of bytes in the SSZ object. Used to ensure the offset is not
/// out of bounds.
/// - `num_fixed_bytes`: the number of fixed-bytes in the struct, if it is known. Used to ensure
/// that the first offset doesn't skip any variable bytes.
///
/// ## References
///
/// The checks here are derived from this document:
///
/// https://notes.ethereum.org/ruKvDXl6QOW3gnqVYb8ezA?view
pub fn sanitize_offset(
    offset: usize,
    previous_offset: Option<usize>,
    num_bytes: usize,
    num_fixed_bytes: Option<usize>,
) -> Result<usize, DecodeError> {
    if num_fixed_bytes.map_or(false, |fixed_bytes| offset < fixed_bytes) {
        Err(DecodeError::OffsetIntoFixedPortion(offset))
    } else if previous_offset.is_none()
        && num_fixed_bytes.map_or(false, |fixed_bytes| offset != fixed_bytes)
    {
        Err(DecodeError::OffsetSkipsVariableBytes(offset))
    } else if offset > num_bytes {
        Err(DecodeError::OffsetOutOfBounds(offset))
    } else if previous_offset.map_or(false, |prev| prev > offset) {
        Err(DecodeError::OffsetsAreDecreasing(offset))
    } else {
        Ok(offset)
    }
}

pub trait SszDecode: Sized {
    fn is_ssz_static() -> bool;
    fn ssz_fixed_len() -> usize;
    fn ssz_max_len() -> usize;

    // Decoding happens in lockstep, where either:
    // - the static type is decoded from the fixed portion at the beginning of the data (fixed_bytes)
    // - or the fixed portion contains an offset to the end of the data (variable_bytes), pointing to a dynamic type
    //
    // To decode a type in one pass, we must split the input data at the end of the fixed portion.
    // Hence the fixed_bytes and variable_bytes parameters. At each decoding step, we read either (or both)
    // inputs depending on the type.
    //
    // We accept any input that implements Buf, which takes care of advancing the buffer for us.
    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError>;

    // dev facing helper function for decoding a (static or variable) type from a slice
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        if Self::is_ssz_static() {
            let (mut fixed_bytes, mut variable_bytes) = bytes.split_at(bytes.len());
            Self::ssz_read(&mut fixed_bytes, &mut variable_bytes)
        } else {
            let (mut fixed_bytes, mut variable_bytes) = bytes.split_at(0);
            Self::ssz_read(&mut fixed_bytes, &mut variable_bytes)
        }
    }
}
