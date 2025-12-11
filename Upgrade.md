# Upgrading polkadot-sdk dependencies

A guide on how to upgrade to the latest [release](https://github.com/paritytech/polkadot-sdk/releases) of `polkadot-sdk`.

Based on updating from `release-polkadot-v1.5.0` to `release-polkadot-v1.9.0`. See the [PR](https://github.com/digicatapult/sqnc-node/pull/169/files).

## Upgrade polkadot-sdk version

`Polkadot-sdk` (`substrate`) dependencies are upgraded using the tool [`psvm`](https://github.com/paritytech/psvm). Once installed you can update the `polkadot-sdk` with, for example:

```bash
# upgrade polkadot-sdk to version stable2409-2
psvm -v "stable2409-2"
```

Available versions are documented on [https://github.com/paritytech/release-registry/](https://github.com/paritytech/release-registry/)

## Debugging techniques

Now that the `polkadot-sdk` dependencies point to a newer release, the next steps involve attempting to compile various sections of `sqnc-node`. There will likely be errors and the advice from the compiler can be difficult to understand. Some techniques for solving compiler errors:

- Thoroughly read ALL of the error. The compiler produces a lot of output for errors and often the relevant tip is buried deep amongst lots of other information. For example, the important part of the following error was `perhaps two different versions of crate 'jsonrpsee_core' are being used?`. The solution was to bump to the latest version of `jsonrpsee` in `Cargo.toml`.

```rust
error[E0271]: expected `impl Fn(DenyUnsafe, Arc<dyn SpawnNamed>) -> Result<RpcModule<()>, Error>` to be a opaque type that returns `Result<RpcModule<_>, Error>`, but it returns `Result<RpcModule<()>, Error>`
   --> node/src/test_service.rs:214:22
    |
214 |         rpc_builder: Box::new(rpc_builder),
    |                      ^^^^^^^^^^^^^^^^^^^^^ expected `Result<RpcModule<_>, Error>`, found `Result<RpcModule<()>, Error>`
    |
    = note: `RpcModule<()>` and `jsonrpsee_core::server::rpc_module::RpcModule<_>` have similar names, but are actually distinct types
note: `RpcModule<()>` is defined in crate `jsonrpsee_core`
   --> /Users/JGray/.cargo/registry/src/index.crates.io-6f17d22bba15001f/jsonrpsee-core-0.16.3/src/server/rpc_module.rs:522:1
    |
522 | pub struct RpcModule<Context> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `jsonrpsee_core::server::rpc_module::RpcModule<_>` is defined in crate `jsonrpsee_core`
   --> /Users/JGray/.cargo/registry/src/index.crates.io-6f17d22bba15001f/jsonrpsee-core-0.22.3/src/server/rpc_module.rs:502:1
    |
502 | pub struct RpcModule<Context> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `jsonrpsee_core` are being used?
    = note: required for the cast from `Box<impl Fn(DenyUnsafe, Arc<(dyn SpawnNamed + 'static)>) -> Result<RpcModule<()>, sc_service::Error>>` to `Box<(dyn Fn(DenyUnsafe, Arc<(dyn SpawnNamed + 'static)>) -> Result<jsonrpsee_core::server::rpc_module::RpcModule<_>, sc_service::Error> + 'static)>`
```

- Rerun `cargo build` after attempting to fix each error. One small change can fix many errors (or generate lots of new ones if wrong).
- Check the [templates](https://github.com/paritytech/polkadot-sdk/tree/release-polkadot-v1.9.0/templates) of the release branch to see what differences there are between the paritytech managed template and `sqnc-node` (which is based on the template). Look back through the commit log - there can often be pull requests that show the necessary code changes to work with the newest release.
- Check [release changelogs](https://github.com/paritytech/polkadot-sdk/releases).

### Pallets

First attempt to build each of the pallets.

```bash
cargo build --release \
    -p pallet-doas \
    -p pallet-organisation-data \
    -p pallet-process-validation \
    -p pallet-symmetric-key \
    -p sqnc-pallet-traits \
    -p pallet-transaction-payment-free \
    -p pallet-utxo-nft \
    -p pallet-validator-set
```

Pallets can also be built one at a time e.g. `cargo build --release -p pallet-doas`.

Once a pallet successfully builds, it needs to be tested `cargo test --release -p pallet-doas`.

If tests pass, bump at least a minor version in the pallet `Cargo.toml` e.g. `pallets/doas/Cargo.toml`.

### Runtime

Test that the upgraded dependencies work for the runtime, including the newly upgraded pallets `cargo build --release -p sqnc-runtime`. Fix any compilation errors.

Bump the version in top-level `Cargo.toml`. Also increment the runtime `spec_version` in `runtime/src/lib.rs` by one.

Run the tests for the runtime:

`cargo test --release -p sqnc-runtime`

### Node

The last item to upgrade is the `node` itself. Bump the node's version in `node/Cargo.toml` and try to build with the runtime-benchmarks feature enabled:

```bash
cargo build --release --features runtime-benchmarks
```

Fix any compilation issues and run `cargo test`. Finally remember to `cargo fmt`.
