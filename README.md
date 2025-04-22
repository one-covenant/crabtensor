# Crabtensor

Crabtensor is Storb's fork of Rusttensor. Rusttensor can be found [here](https://github.com/womboai/rusttensor).

---

A low level Rust library for creating and interacting with Bittensor subnets, built using [subxt](https://github.com/paritytech/subxt).

## Features

- Type safe runtime APIs such as neuron info, hyperparameters, stake info and delegate info
- Type safe extrinsics such as weight setting, axon serving, and much, _much_ more
- Easy access to storage without direct APIs
- Coldkey and hotkey loading utilities
- Auto-generated chain metadata types and functions to ensure everything is tested at compile-time
- Access to chain constants locally at compile-time with no network roundabout
- Full control and flexibility over transaction creation, signing and submission. Along with full flexibility over block hash handling to allow avoiding extra subtensor calls.

## Prerequisites

- Rust toolchain (latest stable version)
- Cargo package manager
- Access to a Bittensor chain endpoint (such as `wss://entrypoint-finney.opentensor.ai:443`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
crabtensor = { git = "https://github.com/storb-tech/crabtensor", version = "v0.5.1" }
```

## Usage Examples

### Creating a client and connecting to the subtensor

```rust
use crabtensor::subtensor::{self, Subtensor, SubtensorUrl};

async fn create_client() -> Result<Subtensor, ...> {
    // Creating client to interact with subtensor
    subtensor::from_url(SubtensorUrl::Finney).await;
}
```

### Unauthorized queries

#### Block Management

`crabtensor`, based on `subxt` allows all the functionality that `subxt` provides, including the blocks API. You can fetch metadata about any block, and reuse block hashes as needed.

```rust
use crabtensor::subtensor::Subtensor;

async fn blocks(client: &Subtensor) -> Result<(), ...> {
    // The latest block can be acquired with `blocks().at_latest()`
    let latest = client.blocks().at_latest().await?;

    // The block number, reference and hash can then be acquired
    let block_number = latest.header.number();
    let block_hash = latest.header.hash();
    let block_reference = latest.reference();

    // The reference can be used in selecting runtime APIs or storages to query
}
```

#### Runtime APIs

The most common requests to the subtensor aside from the weight setting and axon serving extrinsics are the runtime APIs used for the metagraph and hyperparameters.

The runtime APIs are auto-generated according to the latest chain info via the `build.rs` file, and is accessible via `crabtensor::api`.

#### Storage

Some functionality doesn't have a specific API, such as neuron commitments which are used for arbitrary metadata like in SN39. In such cases, you can access the subtensor storage.

```rust
use crabtensor::api;

async fn storage(client: &Subtensor) -> Result<(), ...> {
    let account_id: AccountId = ...;

    // Get the commitment :
    let commitment_address = api::storage().commitment_of(39, account_id);
    let latest_storage = client.storage().at_latest().await?;
    let commitment = latest_storage.fetch(commitment_address).await?;

    // Type safe access to commitment
}
```

### Authorized extrinsics

#### Wallet Management (WIP)

You can load existing Bittensor wallets created using `btcli` and use them for signing extrinsics such as set_weights or serve_axon. Different kind of wallets can be loaded as follows:

```rust
use crabtensor::wallet::{Signer, home_hotkey_location, load_key_seed, signer_from_seed};

// Create a signer from the private key of a hotkey
fn load_hotkey_signer() -> Result<Signer, ...> {
    let path = home_hotkey_location("coldkey", "hotkey").expect("No home directory");
    let seed = load_key_seed(&path)?;// load seed for creating a signer

    Ok(signer_from_seed(&seed)?)
}

// Or just the account ID from the public key
fn load_hotkey_account_id() -> Result<Signer, ...> {
    let path = home_hotkey_location("coldkey", "hotkey").expect("No home directory");

    Ok(load_key_account_id(&path)?)
}
```

With a signer created in a similar fashion to `load_hotkey_signer`, we can submit extrinsics to the chain.

#### Submitting extrinsics

Some extrinsics have specialized APIs that are nicer to work with, such as `serve_axon` and `set_weights`, which reduce the number of parameters needed and uses more idiomatic types.
Regardless of if the extrinsic has a specialized API or otherwise, submitting them remains the same:

```rust
use crabtensor::subtensor::Subtensor;
use crabtensor::wallet::Signer;
use crabtensor::weights::{set_weights_payload, normalize_weights, NormalizedWeight};

async fn submit_extrinsics(client: &Subtensor, signer: &Signer) -> Result<(), ...> {
    let weights = vec![1.0, 2.0, 3.0];

    let weights = normalize_weights(&weights)
        .enumerate()
        .map(|(index, weight)| NormalizedWeight {
            uid: index as u16,
            weight,
        });

    let payload = set_weights_payload(
        1, // netuid
        weights,
        0, // version_key
    );

    // if your extrinsic doesn't have a wrapping API, it can be created using api::tx(), such as api::tx().subtensor_module().dissolve_network(30) for dissolving SN30

    // Submit the transaction
    let transaction = subtensor.client
        .tx()
        .sign_and_submit_then_watch_default(&payload, keypair) // or sign_and_submit_default to avoid waiting for inclusion
        .await?;

    // watch transaction if needed to wait for finalization

    Ok(())
}
```

## Building

```bash
cargo build --release
```

## Development

The project uses a build script (`build.rs`) to automatically generate Substrate metadata types at compile time. This ensures type safety and up-to-date chain compatibility.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## Note on Security

Always handle wallet keys and sensitive information with care. Never share or commit private keys or seed phrases.
