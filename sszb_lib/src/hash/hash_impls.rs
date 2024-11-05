use crate::{SszEncode, SszHash};
use ethereum_types::H256;

impl SszHash for u8 {
    // type PackingFactor = U32;

    fn hash_tree_root(&self) -> H256 {
        let mut hash = H256::zero();
        hash.as_mut()[0] = *self;
        hash
    }
}

macro_rules! uint_ssz_hash {
    ($type: ident) => {
        impl SszHash for $type {
            // type PackingFactor = U8;

            fn hash_tree_root(&self) -> H256 {
                let mut hash = H256::zero();
                self.ssz_write(&mut hash.as_bytes_mut());
                hash
            }
        }
    };
}

uint_ssz_hash!(u16);
uint_ssz_hash!(u32);
uint_ssz_hash!(u64);
uint_ssz_hash!(u128);

impl SszHash for bool {
    // type PackingFactor = U32;

    fn hash_tree_root(&self) -> H256 {
        let mut hash = H256::zero();
        hash.as_mut()[0] = (*self).into();
        hash
    }
}

// impl SszHash for Address {}

// impl<const N: usize> SszHash for FixedBytes<N> {}

// impl SszHash for Bloom {}

// impl SszHash for U256 {}

// impl SszHash for U128 {}

// impl SszHash for H32 {}

// impl SszHash for H160 {}

// impl SszHash for H256 {}

// impl<N: Unsigned + Clone> SszHash for BitVector<N> {}

// impl<N: Unsigned + Clone> SszHash for BitList<N> {}

// impl<T: SszHash> SszHash for Arc<T> {}
