# StarTerra - Gamified Launchpad - Smart Contracts

This monorepository contains the source code for the StarTerra smart contracts on the [Terra](https://terra.money) blockchain.

You can find more information about the architecture, usage and function of the smart contracts in the official [documentation](https://docs.starterra.io/) page.

## Contracts

| Contract                       | Reference | Description | 
|--------------------------------| --------- | ------------|
| [`cw20`](./contracts/cw20)            | [doc](https://docs.starterra.io/technology/smart-contracts/token-contract) | StarTerra token (CW20). |
| [`airdrop-genesis`](./contracts/airdrop-genesis) | [doc](https://docs.starterra.io/technology/smart-contracts/airdrop-genesis) | Contract to perform genesis airdrop for community.         |
| [`kyc-vault`](./contracts/kyc-vault)       | [doc](https://docs.starterra.io/technology/smart-contracts/kyc-vault) | Onchain vault to store data related to KYC and terms of use . |
| [`staking-gateway`](./contracts/staking-gateway) | [doc](https://docs.starterra.io/technology/smart-contracts/staking-gateway)  | Returns the staking pool address which caller belongs to. |
| [`vesting-gateway`](./contracts/vesting-gateway) | [doc](https://docs.starterra.io/technology/smart-contracts/vesting-gateway) | Returns the vesting contract address which caller belongs to.|
| [`ido`](./contracts/ido)             | [doc](https://docs.starterra.io/technology/smart-contracts/ido)  | Contract to verify if caller is eligable for joining IDO. |

## Development

### Environment Setup

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via https://rustup.rs/

2. Run the following:

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://www.docker.com/) is installed

### Unit / Integration Tests

Each contract contains Rust unit tests embedded within the contract source directories. You can run:

```sh
cargo test
```

### Building contracts

```sh
cargo build
```

### Compiling for production

For production builds, run the following:

```sh
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.11.3
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will be available inside the `artifacts/` directory.

## License

Copyright 2021 StarTerra (_ST Development Limited_)

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the
License. You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0. Unless required by
applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

See the License for the specific language governing permissions and limitations under the License.