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