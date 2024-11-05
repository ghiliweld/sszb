use ethereum_types::H256;
// use typenum::{Logarithm2, NonZero, Unsigned};

pub mod hash_impls;

use crate::SszEncode;

pub trait SszHash: SszEncode {
    // type PackingFactor: Unsigned + NonZero + Logarithm2;

    fn hash_tree_root(&self) -> H256;
}
