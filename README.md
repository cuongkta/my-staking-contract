# Run Test
````
cargo test

```


# Deploy and Update

```
 near dev-deploy  out/my-staking-contract.wasm 

```


# Call 

```
near call dev-1658532518731-29971022106934 new_default_config '{"owner_id": "tonnyta.testnet", "ft_contract_id": "ft_tonnyta.testnet"}' --accountId tonnyta.testnet

```


# View 

```
near view dev-1658532518731-29971022106934 get_pool_info
```



# Migrate when change struct of contract

```
near dev-deploy ./out/my-staking-contract.wasm  --initFunction migrate --initArgs '{}'

```


# Register new account 

```
near call dev-1658789183800-49068626170913 storage_deposit '{"account_id": "tonnyta.tesnet"}' --deposit 0.01 --accountId tonnyta.testnet

```

# View account info 

```
near view  dev-1658789183800-49068626170913 get_account_info '{"account_id": "tonnyta.tesnet"}' 

```



# Staking  in ft contract 

```
near call ft-k6.tonnyta.testnet ft_transfer_call '{"receiver_id": "dev-1658789183800-49068626170913", "amount": "1000000000000000000000000", "msg":""}' --accountId tonnyta.testnet --depositYocto 1 --gas 60000000000000

```



# Note: there are some reasons that above staking not update account info util we create and staking other account 


	# create other account - note accout is exist in near account

	```
	near call dev-1658789183800-49068626170913  storage_deposit '{"account_id": "mocdiep.testnet"}' --deposit 0.01 --accountId tonnyta.testnet

	```

	# get token from other account in ft contract

	```
	near call ft-k6.tonnyta.testnet ft_transfer '{"receiver_id": "mocdiep.testnet", "amount":"10000000000000000000"}' --accountId tonnyta.testnet --amount 0.000000000000000000000001

	```


# After staking, we will harvest reward

```
near call  dev-1658789183800-49068626170913 harvest --accountId mocdiep.testnet --depositYocto 1 --gas 300000000000000

```


#Unstake token 

```
near call dev-1658789183800-49068626170913 unstake '{"amount": "1000000"}' --accountId tonnyta.testnet --depositYocto 1

```


# Withdraw token 

```
near call dev-1658789183800-49068626170913 withdraw --accountId tonnyta.testnet --depositYocto 1 --gas 60000000000000

```



---------------------- Not releate but will be noted -------------------------

# Create sub account 

```
near create-account ft-k6.tonnyta.testnet --masterAccount tonnyta.testnet --initialBalance 4
```


# Deploy contract


```
near deploy --wasmFile out/vbi-ft.wasm  --initFunction  new_default_meta --initArgs '{"owner_id" :"tonnyta.testnet", "total_supply": "100000000000000000000000000000000"}' --accountId ft-k6.tonnyta.testnet

```



# View token info 

```
near view ft-k6.tonnyta.testnet ft_metadata

```


# Create account to deploy staking-contract

```
 near create-account staking.tonnyta.testnet  --masterAccount tonnyta.testnet --initialBalance 4
```


```
near deploy --wasmFile out/my-staking-contract.wasm  --initFunction new_default_config --initArgs '{"owner_id": "tonnyta.testnet", "ft_contract_id": "ft-k6.tonnyta.testnet"}' --accountId staking.tonnyta.testnet

```


# Deposite token for staking-contract 

```
near call ft-k6.tonnyta.testnet storage_deposit --deposit 0.01 --accountId staking.tonnyta.testnet

```


#Transfer  for staking-contract


```
near call ft-k6.tonnyta.testnet ft_transfer '{"receiver_id": "staking.tonnyta.testnet", "amount": "1000000000000000000000000"}' --accountId tonnyta.testnet  --depositYocto 1 

```


# View account id received or not 


```
near view ft-k6.tonnyta.testnet ft_balance_of '{"account_id": "staking.tonnyta.testnet"}'

```



Harvest will return token in ft token of account 

Unstake (stake_balance =- amount and wait for available epoch to withdraw)   --> WithDraw ( receiver account will receive in ft token = un_stake token)