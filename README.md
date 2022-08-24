# Art Demo

Generative art on NEAR.

---

## Environment

```sh
export CONTRACT = [contract_id] // ncd.ys24.testnet
export ACCOUNT = [account_id]
```

## Methods

`design(): void`

```sh
near call CONTRACT design --accountId ACCOUNT
```

```sh
near call CONTRACT design '{"seed": [number]}'  --accountId ACCOUNT
```

`claim_my_design(): void`

```sh
near call CONTRACT claim_my_design '{"seed": [number]}' --accountId ACCOUNT
```

`view_my_design(): void`

```sh
near call CONTRACT view_my_design --accountId ACCOUNT
```

`burn_my_design(): void`

```sh
near call CONTRACT burn_my_design --accountId ACCOUNT
```
