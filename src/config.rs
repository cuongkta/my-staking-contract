use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

// Cong thuc tinh thuong cho user
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub reward_numerator: u32,
    pub reward_denumerator: u64,
}

impl Default for Config {
    fn default() -> Self {
        // APR 15% to 18%
        Self {
            reward_numerator: 715,
            reward_denumerator: 10000000000,
        } //reward per block
    }
}
// APR  = (token *715/10000000) * total_block
