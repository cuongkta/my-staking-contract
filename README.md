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
near call ev-1658789183800-49068626170913 new_default_config '{"owner_id": "tonnyta.testnet", "ft_contract_id": "ft-k6.tonnyta.testnet"}' --accountId tonnyta.testnet

```

Voi ft-k6.tonnyta.testnet la account deply fungule token 

# View 

```
near view ev-1658789183800-49068626170913 get_pool_info
```



# Migrate when change struct of contract

```
near dev-deploy ./out/my-staking-contract.wasm  --initFunction migrate --initArgs '{}'

```


# Create new account 

```
near call dev-1658789183800-49068626170913 storage_deposit  --deposit 0.01 --accountId tonnyta.testnet
```


# Get Account Info 

```
near view dev-1658789183800-49068626170913 get_account_info '{"account_id" :"tonnyta.testnet"}'

```


# Create new account in ft contract 

```
near call ft-k6.tonnyta.testnet storage_deposit --deposit 0.01 --accountId dev-1658789183800-49068626170913
```

# Transfer token 

```
near call ft-k6.tonnyta.testnet ft_transfer '{"receiver_id": "dev-1658789183800-49068626170913", "amount": "1000000000000000000000000"}' --accountId tonnyta.testnet  --depositYocto 1

```


# Deposite and Stake 

```
near call ft-k6.tonnyta.testnet ft_transfer_call '{"receiver_id": "dev-1658789183800-49068626170913", "amount": "1000000000000000000000000", "msg": ""}' --accountId tonnyta.testnet  --depositYocto 1 --gas 60000000000000

```
Chu y co ft_on_transfer o staking contract vi ft contract se goi sang 




#Harvest reward 

```
near call dev-1658789183800-49068626170913 harvest --accountId tonnyta.testnet --depositYocto 1 --gas 60000000000000

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