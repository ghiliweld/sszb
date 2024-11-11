use alloy_primitives::{Address, B256, U256};
use bytes::buf::{Buf, BufMut};
use ghilhouse::List;
use itertools::Itertools as _;
use ssz_types::{BitVector, FixedVector as Vector};
use sszb::*;
use sszb_derive::{SszbDecode, SszbEncode};
use tree_hash_derive::TreeHash;

type ByteVector<N> = Vector<u8, N>;
type PublicKeyBytes = [u8; 48];
type H32 = ByteVector<typenum::U4>;

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash, Default)]
pub struct Fork {
    pub previous_version: H32,
    pub current_version: H32,
    pub epoch: u64,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash, Default)]
pub struct Checkpoint {
    pub epoch: u64,
    pub root: B256,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash, Default)]
pub struct BeaconBlockHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: B256,
    pub state_root: B256,
    pub body_root: B256,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash, Default)]
pub struct Eth1Data {
    pub deposit_root: B256,
    pub deposit_count: u64,
    pub block_hash: B256,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct Validator {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: B256,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: u64,
    pub activation_epoch: u64,
    pub exit_epoch: u64,
    pub withdrawable_epoch: u64,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct SyncCommittee {
    pub pubkeys: Vector<PublicKeyBytes, typenum::U512>,
    pub aggregate_pubkey: PublicKeyBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct ExecutionPayloadHeader {
    pub parent_hash: B256,
    pub fee_recipient: Address,
    pub state_root: B256,
    pub receipts_root: B256,
    pub logs_bloom: Vector<u8, typenum::U256>,
    pub prev_randao: B256,
    pub block_number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: List<u8, typenum::U32>,
    pub base_fee_per_gas: U256,
    pub block_hash: B256,
    pub transactions_root: B256,
    pub withdrawals_root: B256,
    pub blob_gas_used: u64,
    pub excess_blob_gas: u64,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash, Default)]
pub struct HistoricalSummary {
    pub block_summary_root: B256,
    pub state_summary_root: B256,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct BeaconState {
    // Versioning
    pub genesis_time: u64,
    pub genesis_validators_root: B256,
    pub slot: u64,
    pub fork: Fork,

    // History
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: Vector<B256, typenum::U8192>,
    pub state_roots: Vector<B256, typenum::U8192>,
    pub historical_roots: List<B256, typenum::U16777216>,

    // Ethereum 1.0 chain data
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: List<Eth1Data, typenum::U2048>,
    pub eth1_deposit_index: u64,

    // Registry
    pub validators: List<Validator, typenum::U1099511627776>,
    pub balances: List<u64, typenum::U1099511627776>,

    // Randomness
    pub randao_mixes: Vector<B256, typenum::U65536>,

    // Slashings
    pub slashings: Vector<u64, typenum::U8192>,

    // Participation (Altair and later)
    pub previous_epoch_participation: List<u8, typenum::U1099511627776>,
    pub current_epoch_participation: List<u8, typenum::U1099511627776>,

    // Finality
    pub justification_bits: BitVector<typenum::U4>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,

    // Inactivity
    pub inactivity_scores: List<u64, typenum::U1099511627776>,

    // Light-client sync committees
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee,

    // Execution
    pub latest_execution_payload_header: ExecutionPayloadHeader,

    // Capella
    pub next_withdrawal_index: u64,
    pub next_withdrawal_validator_index: u64,

    // Deneb
    pub historical_summaries: List<HistoricalSummary, typenum::U16777216>,
}
