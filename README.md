
## Setup
Soroban contracts are small programs written in the Rust programming language.

To build and develop contracts you need only a couple prerequisites:

A Rust toolchain
An editor that supports Rust
Soroban CLI

### Install Rust
Install rustup with the following command.
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Target
Install the `wasm32-unknown-unknown` target.

```
rustup target add wasm32-unknown-unknown
```

### Install the Soroban CLI
The Soroban CLI can execute Soroban contracts in the same environment the contract will execute on network, however in a local sandbox.

Install the Soroban CLI using cargo install.
```
cargo install --locked --version 0.7.0 soroban-cli
```

## Build Project
### Run Test
Run cargo test and watch the contract run. You should see the following output:
`cargo test`
```running 1 test
test test::test ... ok
```

### Build
To build a Soroban contract to deploy or run, use the `cargo build` command.
```
cargo build --target wasm32-unknown-unknown --release
```

A `.wasm` file will be outputted in the target directory. The `.wasm` file is the built contract.
```
target/wasm32-unknown-unknown/release/nft_project.wasm
```

## Deploy to FutureNet
If you have Docker installed, you can run a local node with the [Stellar Quickstart](https://github.com/stellar/quickstart) Docker image that joins the [Futurenet](https://soroban.stellar.org/docs/reference/futurenet) network, and then use that local node to deploy.
To run a local node for the Futurenet network with the Stellar Quickstart Docker image, run the following command.
```
docker run --rm -it \
  -p 8000:8000 \
  --name stellar \
  stellar/quickstart:soroban-dev@sha256:a057ec6f06c6702c005693f8265ed1261e901b153a754e97cf18b0962257e872 \
  --futurenet \
  --enable-soroban-rpc
```
Once the image is started you can check its status by querying the Horizon API:
```
curl http://localhost:8000
```
It takes sometime to join a remote network. Monitor the output of that endpoint until you see the core_latest_ledger become a number above zero.

Generate a key by going to the [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=futurenet). Make note of both the G... and S... keys. The G... key is the public key and will also be the account ID. The S... key is the secret key and is that you use to control the account.

Create an account on the Futurenet network by making a request to the Friendbot. Specify as the `addr` the `G...` key of your account.
```
curl "https://friendbot-futurenet.stellar.org/?addr=G..."
```
Once you have an account on the network, we'll use the code we wrote in Write a Contract and the resulting `.wasm` file we built in Build as our contract to deploy. Run the following commands to deploy the contract to the network. Use the `S...` key as the secret key.
```
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/nft_project.wasm \
    --source S... \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```
 We can also connect to `remote` futurenet node by using `--rpc-url`
 ```
--rpc-url https://rpc-futurenet.stellar.org:443
```
 
 A contract ID will be outputted.
```
cd4dae2c409c433b1e1d83994a20214d3e5f60bdd3a817978d8aa7c797864313
```
Using the contract ID that was outputted, use the `soroban-cli` to invoke the `hello` function with a single argument `friend`.
```
soroban contract invoke \
    --id cd4dae2c409c433b1e1d83994a20214d3e5f60bdd3a817978d8aa7c797864313 \
    --source S... \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    -- hello \
    --to friend
```
The following output should appear.
```
["Hello", "friend"]
```