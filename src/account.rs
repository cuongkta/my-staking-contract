use near_sdk::Timestamp;

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
