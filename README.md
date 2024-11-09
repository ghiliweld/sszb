# sszb
A high performance SSZ implementation in Rust. Optimised for speed, while retaining flexibility with the `Buf/BufMut` traits.

The library comes with the `SszEncode/SszDecode` traits, and trait implementations for the same types as `ethereum_ssz`.

## Installation

Add the following lines to your project's `Cargo.toml` file:
```
sszb = { package = "sszb", git = "https://github.com/ghiliweld/sszb.git" }
sszb_derive = { package = "sszb_derive", git = "https://github.com/ghiliweld/sszb.git" }
```

## Usage

```rs
use sszb::SszDecode;
use sszb_derive::{SszbDecode, SszbEncode};

#[derive(SszbEncode, SszbDecode)]
pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: SignatureBytes,
}

...

fn main() {
    let block_bytes: Vec<u8> = std::fs::read("beacon-block.ssz").unwrap();

    let beacon_block = <SignedBeaconBlock as SszDecode>::from_ssz_bytes(bytes).unwrap();

    let encoded_block = beacon_block.to_ssz();

    // if you already have a mutable buffer to write into, you can reuse it to encode your object
    // this saves on allocations
    let len = beacon_block.ssz_bytes_len();
    let mut buf: Vec<u8> = vec![0u8; len];
    let encoded_block = beacon_block.ssz_write(&mut buf.as_mut_slice()));
}
```
