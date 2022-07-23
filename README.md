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