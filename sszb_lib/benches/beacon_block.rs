use alloy_primitives::{Address, FixedBytes, B256, U256};
use bytes::buf::{Buf, BufMut};
use itertools::Itertools as _;
use ssz_types::{BitList, BitVector, FixedVector, VariableList as List};
use sszb::*;
use sszb_derive::{SszbDecode, SszbEncode};
use tree_hash::*;
use tree_hash_derive::TreeHash;

type ByteList<N> = List<u8, N>;
type ByteVector<N> = FixedVector<u8, N>;
pub type SignatureBytes = Sig; // ByteVector<typenum::U96>;
type PublicKeyBytes = PKBytes; // [u8; 48];
type KZGCommitment = [u8; 48];
type H32 = [u8; 4];
type H160 = Address;
type H256 = B256;

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug)]
pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: SignatureBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct SignedBeaconBlockHeader {
    pub message: BeaconBlockHeader,
    pub signature: SignatureBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct BeaconBlockHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: H256,
    pub state_root: H256,
    pub body_root: H256,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug)]
pub struct BeaconBlock {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: H256,
    pub state_root: H256,
    pub body: BeaconBlockBody,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug)]
pub struct BeaconBlockBody {
    pub randao_reveal: SignatureBytes,
    pub eth1_data: Eth1Data,
    pub graffiti: FixedBytes<32>,
    pub proposer_slashings: List<ProposerSlashing, typenum::U16>,
    pub attester_slashings: List<AttesterSlashing, typenum::U2>,
    pub attestations: List<Attestation, typenum::U128>,
    pub deposits: List<Deposit, typenum::U16>,
    pub voluntary_exits: List<SignedVoluntaryExit, typenum::U16>,
    pub sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload,
    pub bls_to_execution_changes: List<SignedBlsToExecutionChange, typenum::U16>,
    pub blob_kzg_commitments: List<KZGCommitment, typenum::U4096>,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug)]
pub struct Eth1Data {
    pub deposit_root: H256,
    pub deposit_count: u64,
    pub block_hash: H256,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct ProposerSlashing {
    pub signed_header_1: SignedBeaconBlockHeader,
    pub signed_header_2: SignedBeaconBlockHeader,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct Checkpoint {
    pub epoch: u64,
    pub root: H256,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct AttestationData {
    pub slot: u64,
    pub index: u64,
    pub beacon_block_root: H256,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct IndexedAttestation {
    pub attesting_indices: List<u64, typenum::U2048>,
    pub data: AttestationData,
    pub signature: SignatureBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct AttesterSlashing {
    pub attestation_1: IndexedAttestation,
    pub attestation_2: IndexedAttestation,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct Attestation {
    pub aggregation_bits: BitList<typenum::U2048>,
    pub data: AttestationData,
    pub signature: SignatureBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct DepositData {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: H256,
    pub amount: u64,
    pub signature: SignatureBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct Deposit {
    pub proof: FixedVector<H256, typenum::U32>,
    pub data: DepositData,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct VoluntaryExit {
    pub epoch: u64,
    pub validator_index: u64,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: SignatureBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug)]
pub struct SyncAggregate {
    pub sync_committee_bits: BitVector<typenum::U512>,
    pub sync_committee_signature: SignatureBytes,
}

pub type Transaction = ByteList<typenum::U1073741824>;

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct Withdrawal {
    pub index: u64,
    pub validator_index: u64,
    pub address: H160,
    pub amount: u64,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug)]
pub struct ExecutionPayload {
    pub parent_hash: H256,
    pub fee_recipient: H160,
    pub state_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: FixedBytes<256>,
    pub prev_randao: H256,
    pub block_number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: ByteList<typenum::U32>,
    pub base_fee_per_gas: U256,
    pub block_hash: H256,
    pub transactions: List<Transaction, typenum::U1048576>,
    pub withdrawals: List<Withdrawal, typenum::U16>,

    // New in Deneb
    pub blob_gas_used: u64,
    pub excess_blob_gas: u64,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct SignedBlsToExecutionChange {
    pub message: BlsToExecutionChange,
    pub signature: SignatureBytes,
}

#[derive(Clone, SszbEncode, SszbDecode, PartialEq, Debug, TreeHash)]
pub struct BlsToExecutionChange {
    pub validator_index: u64,
    pub from_bls_pubkey: PublicKeyBytes,
    pub to_execution_address: H160,
}
