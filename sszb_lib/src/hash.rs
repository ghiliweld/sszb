use ethereum_types::H256;
use typenum::{Logarithm2, NonZero, Unsigned};

pub trait SszHash {
    type PackingFactor: Unsigned + NonZero + Logarithm2;

    fn hash_tree_root(&self) -> H256;
}
