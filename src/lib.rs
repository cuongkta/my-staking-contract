use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserializer, Serializer};
use near_sdk::{env, near_bindgen, AccountId, Balance, BlockHeight, EpochHeight, PanicOnDefault};

use crate::account::*;
use crate::config::*;
mod account;
mod config;
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config,               // Cau hinh tra thuong
    pub total_stake_balance: Balance, // Tong so luong dang stake
    pub total_paid_reward_balance: Balance,
    pub total_staker: Balance,
    pub pre_reward: Balance,
    pub last_bloc_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, Account>,
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {}
