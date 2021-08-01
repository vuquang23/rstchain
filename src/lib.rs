pub mod block;
pub mod hashable;
pub mod blockchain;
pub mod transaction;

pub use crate::block::Block;
pub use crate::hashable::Hashable;
pub use crate::blockchain::Blockchain;
pub use crate::transaction::Transaction;
use std::time::{ SystemTime, UNIX_EPOCH };

type Hash = Vec<u8>;
type Address = String;
type BlockHash = Vec<u8>;

pub fn u32_bytes (u: &u32) -> [u8; 4] {
    let mut ret: [u8; 4] = [0; 4];
    for i in 0..4 as usize {
        ret[i] = (u >> 8 * i) as u8;
    }
    ret
}

pub fn u64_bytes (u: &u64) -> [u8; 8] {
    let mut ret: [u8; 8] = [0; 8];
    for i in 0..8 as usize {
        ret[i] = (u >> 8 * i) as u8;
    }
    ret
}

pub fn u128_bytes (u: &u128) -> [u8; 16] {
    let mut ret: [u8; 16] = [0; 16];
    for i in 0..16 as usize {
        ret[i] = (u >> 8 * i) as u8;
    }
    ret
}

pub fn difficulty_bytes_as_u128 (v: &Vec<u8>) -> u128 {
    let mut ret: u128 = 0;
    for i in 16..32usize {
        ret = ret | ((v[i] as u128) << (8 * (i - 16)));
    }
    ret
}

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}
