use crate::*;

#[near_bindgen]
impl StakingContract {
    pub(crate) fn internal_unstake(&mut self, account_id: AccountId, amount: u128) {
        let upgradeable_account = self.accounts.get(&account_id).unwrap();
        let mut account = Account::from(upgradeable_account);
        assert!(
            amount <= account.stake_balance,
            "ERROR_AMOUNT_MUST_LESS_THAN_STAKE"
        );
        let new_reward = self.internal_calculate_account_reward(&account);
        // update account data
        account.pre_reward = new_reward;
        account.stake_balance -= amount;
        account.last_block_balance_change = env::block_index();
        account.unstake_balance += amount;
        account.unstake_start_timestamp = env::block_timestamp();
        account.unstake_available_epoch = env::epoch_height() + 1; //epoch tiep theo co the rut -widthdraw

        if account.unstake_balance == 0 {
            self.total_stake_balance -= 1;
        }

        self.accounts
            .insert(&account_id, &UpgradeableAccount::from(account));

        let new_contract_reward = self.internal_calculate_global_reward();
        self.pre_reward += new_contract_reward;
        self.last_block_balance_change = env::block_index();
        self.total_stake_balance -= amount;
    }

    pub(crate) fn internal_deposit_and_stake(&mut self, account_id: AccountId, amount: u128) {
        let upgradable_account = self.accounts.get(&account_id);
        assert!(upgradable_account.is_some(), "ERROR_ACCOUNT_NOT_FOUND");
        assert_eq!(self.paused, false, "ERROR_CONTRACT_PAUSED");
        assert_eq!(
            self.ft_contract_id,
            env::predecessor_account_id(),
            "ERROR_INVALID_FT_CONTRACT_ID"
        );

        let mut account = Account::from(upgradable_account.unwrap());
        env::log(b"Account is");
        env::log(account_id.to_string().as_bytes());
        env::log(account.stake_balance.to_string().as_bytes());
        if account.stake_balance == 0 {
            self.total_staker += 1;
        }

        let new_reward = self.internal_calculate_account_reward(&account);

        //update account data
        account.pre_reward += new_reward;
        account.stake_balance += amount;
        account.last_block_balance_change = env::block_index();
        self.accounts
            .insert(&account_id, &UpgradeableAccount::from(account));

        //Update pool data

        let new_contract_reward = self.internal_calculate_global_reward();
        self.total_stake_balance += amount;
        self.pre_reward += new_contract_reward;
        self.last_block_balance_change = env::block_index();
    }

    // Register account
    pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
        let account = Account {
            stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            unstake_balance: 0,
            unstake_start_timestamp: 0,
            unstake_available_epoch: 0,
        };
        self.accounts
            .insert(&account_id, &UpgradeableAccount::from(account));
    }

    pub(crate) fn internal_calculate_account_reward(&self, account: &Account) -> Balance {
        let lasted_block = if self.paused {
            self.pause_in_block
        } else {
            env::block_index()
        };

        let diff_block = lasted_block - account.last_block_balance_change;
        let reward =
            (account.stake_balance * self.config.reward_numerator as u128 * diff_block as u128)
                / self.config.reward_denumerator as u128;
        reward
    }

    pub(crate) fn internal_calculate_global_reward(&self) -> Balance {
        let lasted_block = if self.paused {
            self.pause_in_block
        } else {
            env::block_index()
        };

        let diff_block = lasted_block - self.last_block_balance_change;
        let reward =
            (self.total_stake_balance * self.config.reward_numerator as u128 * diff_block as u128)
                / self.config.reward_denumerator as u128;
        reward
    }
}
