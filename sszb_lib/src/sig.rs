use crate::{DecodeError, SszDecode, SszEncode};
use bytes::buf::{Buf, BufMut};
use sigp_bls::{PublicKeyBytes, Signature};
use tree_hash::TreeHash;

#[derive(Clone, PartialEq, Debug)]
pub struct PKBytes(PublicKeyBytes);

impl TreeHash for PKBytes {
    fn tree_hash_type() -> tree_hash::TreeHashType {
        tree_hash::TreeHashType::Vector
    }

    fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
        unreachable!("Vector should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("Vector should never be packed.")
    }

    fn tree_hash_root(&self) -> tree_hash::Hash256 {
        // We could use the tree hash implementation for `FixedVec<u8, $byte_size>`,
        // but benchmarks have show that to be at least 15% slower because of the
        // unnecessary copying and allocation (one Vec per byte)
        let values_per_chunk = tree_hash::BYTES_PER_CHUNK;
        let minimum_chunk_count = (48 + values_per_chunk - 1) / values_per_chunk;
        tree_hash::merkle_root(&self.0.serialize(), minimum_chunk_count)
    }
}

impl SszEncode for PKBytes {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        48
    }

    fn ssz_max_len() -> usize {
        48
    }

    fn ssz_bytes_len(&self) -> usize {
        48
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.0.serialize())
    }
}

impl SszDecode for PKBytes {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        48
    }

    fn ssz_max_len() -> usize {
        48
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            let res = PublicKeyBytes::deserialize(&fixed_bytes.chunk()[0..48])
                .map_err(|e| DecodeError::BytesInvalid(format!("{:?}", e)));
            fixed_bytes.advance(48);
            Ok(Self(res.unwrap()))
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Sig(Signature);

impl TreeHash for Sig {
    fn tree_hash_type() -> tree_hash::TreeHashType {
        tree_hash::TreeHashType::Vector
    }

    fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
        unreachable!("Vector should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("Vector should never be packed.")
    }

    fn tree_hash_root(&self) -> tree_hash::Hash256 {
        // We could use the tree hash implementation for `FixedVec<u8, $byte_size>`,
        // but benchmarks have show that to be at least 15% slower because of the
        // unnecessary copying and allocation (one Vec per byte)
        let values_per_chunk = tree_hash::BYTES_PER_CHUNK;
        let minimum_chunk_count = (96 + values_per_chunk - 1) / values_per_chunk;
        tree_hash::merkle_root(&self.0.serialize(), minimum_chunk_count)
    }
}

impl SszDecode for Sig {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        96
    }

    fn ssz_max_len() -> usize {
        96
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            let res = Signature::deserialize(&fixed_bytes.chunk()[0..96])
                .map_err(|e| DecodeError::BytesInvalid(format!("{:?}", e)));
            fixed_bytes.advance(96);
            Ok(Self(res.unwrap()))
        }
    }
}

impl SszEncode for Sig {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        96
    }

    fn ssz_max_len() -> usize {
        96
    }

    fn ssz_bytes_len(&self) -> usize {
        96
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.0.serialize())
    }
}
