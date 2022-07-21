use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub stake_balance: Balance, //tong so luong token deposite
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub unstake_balance: Balance, //so luong unstake,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight,
}
//so luong token bi lock lai o 1 epoch, eg: unstake o epoch 100, toi epoch 101 will be back user wallet
//Time line  t1-------------t2-----------now
//Balance   100k            200k               --> tinh lai pre_reward khi user deposite

#[derive(BorshDeserialize, BorshSerialize)]
pub enum UpgradeableAccount {
    Current(Account),
}

impl From<UpgradeableAccount> for Account {
    fn from(upgradeable_account: UpgradeableAccount) -> Self {
        match upgradeable_account {
            UpgradeableAccount::Current(account) => account,
        }
    }
    // add code here
}

impl From<Account> for UpgradeableAccount {
    fn from(account: Account) -> Self {
        UpgradeableAccount::Current(account)
    }
    // add code here
}

//get account info and return json object
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountJson {
    pub account_id: AccountId,
    pub stake_balance: U128,
    pub unstake_balance: U128,
    pub reward: U128,
    pub can_withdraw: bool,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight,
    pub current_epoch: EpochHeight,
}

impl AccountJson {
    pub fn from(account_id: AccountId, new_reward: Balance, account: Account) -> Self {
        AccountJson {
            account_id,
            stake_balance: U128(account.stake_balance),
            unstake_balance: U128(account.unstake_balance),
            reward: U128(account.pre_reward + new_reward), // new_reward: now --> T2
            can_withdraw: account.unstake_available_epoch <= env::epoch_height(),
            unstake_start_timestamp: account.unstake_start_timestamp,
            unstake_available_epoch: account.unstake_available_epoch,
            current_epoch: env::epoch_height(),
        }
    }
}
