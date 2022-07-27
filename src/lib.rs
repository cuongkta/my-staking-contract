use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;

use near_sdk::{
    env, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault, Promise, Timestamp, PromiseOrValue
};

use crate::account::*;
use crate::config::*;
use crate::enumeration::*;
use crate::internal::*;
use crate::util::*;
use crate::core_impl::*;
mod account;
mod config;
mod enumeration;
mod internal;
mod util;
mod core_impl;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StakingContractV1 {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config,               // Cau hinh tra thuong
    pub total_stake_balance: Balance, // Tong so luong dang stake
    pub total_paid_reward_balance: Balance,
    pub total_staker: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, UpgradeableAccount>,
    pub paused: bool, //het token cho user thi user khong deposite dc nua
    pub pause_in_block: BlockHeight//  save lai trang thai block khi owner chuyen sang pause, de tinh toan reward
}

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
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, UpgradeableAccount>,
    pub paused: bool, //het token cho user thi user khong deposite dc nua
    pub pause_in_block: BlockHeight, //  save lai trang thai block khi owner chuyen sang pause, de tinh toan reward
    pub new_data: U128
}

#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new_default_config(owner_id: AccountId, ft_contract_id: AccountId) -> Self {
        Self::new(owner_id, ft_contract_id, Config::default())
    }

    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        StakingContract {
            owner_id,
            ft_contract_id,
            config,
            total_stake_balance: 0,
            total_paid_reward_balance: 0,
            total_staker: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            accounts: LookupMap::new(StorageKey::AccountKey),
            paused: false,
            pause_in_block: 0, //tai sao lai la 0
            new_data: U128(10)
        }
    }

    // deposite token when create account
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        assert_at_least_one_yocto();
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id()); //get current account call
        let _account_stake = self.accounts.get(&account);
        if _account_stake.is_some() {
            // refund all token deposite
            refund_deposite(0);
        } else {
            // create new account
            // Refund token deposite after creating new account
            let before_storage_usage = env::storage_usage();
            self.internal_register_account(account.clone());
            env::log(b"Create Account Successfully");
            let after_storage_usage = env::storage_usage();
            let log_message = format!("Deposite {}", after_storage_usage - before_storage_usage);
            env::log(log_message.as_bytes());
            refund_deposite(after_storage_usage - before_storage_usage);
        }
    }

    //check if account is exists
    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        let account = self.accounts.get(&account_id);
        if account.is_some() {
            U128(1)
        } else {
            U128(0)
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_newdata(&self) -> U128{
        self.new_data
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self{  //migrate function for case when change structure of contract
        let contract_v1: StakingContractV1 = env::state_read().expect("Can not read contract");
    
        StakingContract { 
            owner_id: contract_v1.owner_id, 
            ft_contract_id: contract_v1.ft_contract_id, 
            config: contract_v1.config, 
            total_stake_balance: contract_v1.total_stake_balance, 
            total_paid_reward_balance: contract_v1.total_paid_reward_balance, 
            total_staker: contract_v1.total_staker, 
            pre_reward: contract_v1.pre_reward, 
            last_block_balance_change: contract_v1.last_block_balance_change, 
            accounts: contract_v1.accounts, 
            paused: contract_v1.paused, 
            pause_in_block: contract_v1.pause_in_block, 
            new_data: U128(0)
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};
    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);
        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let config: Config = Config {
            reward_numerator: 1000,
            reward_denumerator: 1000000,
        };

        let contract =
            StakingContract::new(accounts(1).to_string(), "ft.contract".to_string(), config);

        assert_eq!(contract.owner_id, accounts(1).to_string());
        assert_eq!(contract.ft_contract_id, "ft.contract".to_string());
        assert_eq!(contract.config.reward_numerator, config.reward_numerator);
        assert_eq!(contract.paused, false);
    }
}
