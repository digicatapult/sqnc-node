# Upgrading Substrate dependencies

This guide will hopefully provide help to upgrade Substrate in the future.

This is a guide based on updating from `0.9.25` to `0.9.30` and the issues it caused can be seen [here](https://github.com/digicatapult/sqnc-node/pull/91/files)

## Prerequisities

Make sure **main** is up to date and create an **integration** branch which all other branches will be PR'd into. This reduces git issues like having to use `git reset`.

You will need to run `rustup` which is [documented here](https://github.com/digicatapult/sqnc-node/blob/main/README.md).

## Upgrade Substrate

This [diener tool](https://crates.io/crates/diener) can be used to update the toml versions easily by upgrading each `cargo.toml` given a specific branch/path

`cargo install diener`
and in the root of the `veritable-node` directory run `diener update --substrate --branch polkadot-v<version>`

### Test Building the Node

We are test building the Node having only upgraded the dependencies, this includes no code changes to any of the pallets, runtime or node. We include features runtime benchmarks to fully tests all deps.

To build the Node:

```bash
cargo build --release --features runtime-benchmarks
```

### Pallets

Once the version updates have been made we should try to build a pallet, in this example we shall try to build the `pallet-doas` in isolation.

`cargo build --release -p pallet-doas`

If there are errors you will need to investigate. It would also be worth looking at the [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template) to see what what changes they have made when updgrading and potentially compare code changes between different tagged releases of substrate.

For example, during the `0.9.30` upgrade `Event` and `Origin` became `RuntimeEvent` and
`RuntimeOrigin`. This information was
obtained from checking the [Polkadot `0.9.30` branch](https://github.com/paritytech/substrate/tree/polkadot-v0.9.30). Checking the branch will help with renaming and syntax changes.

[Version changes can be the most difficult parts of the code](https://github.com/digicatapult/sqnc-node/pull/91/files#diff-6d40c1b90e071cdb5271cce23374b2ecae20ab264980fda18a4d4d4c290efca1), if you look at the original compared against the new version there could substancial, or minor, changes (depending on the update).

After the change each pallet needs to be inspected, fixed if needed, along with fixes to tests. If there are any compilation errors Rust is very good at highlighting issues and suggesting looking error codes e.g. `rustc --explain E0152 `.

```bash
cargo build --release pallet-doas
```

Once a pallet has been brought up to date it needs to be tested, something like
`cargo test --release -p pallet-doas`

If it passes, bump its version in the pallets `pallets/doas/Cargo.toml` push it into it's own PR, then into the **integration** branch

### Runtime

We now need to test that the upgraded dependencies work for the runtime including our newly upgraded pallets. So firstly replace all the pallet versions in the runtime's `runtime/Cargo.toml` and then we need to test a runtime build, we do this by
`cargo build --release -p sqnc-node-runtime`

We will need to increment the runtimes `spec_version` to a higher value, note spec_version does not recognize semver, so we treat it as a whole number. e.g. In place of `5.6.8` the `spec_version` would be `568`.

We should also bump our runtime's version in `runtime/Cargo.toml`
We should now run the tests for the runtime, which we can do using:

`cargo test --release -p sqnc-node-runtime`

If the tests pass create a PR from the **integration** branch.

### Node

The last item to upgrade is the `veritable-node` itself.

There are two steps to undertake, the veritable-node's and veritable-node-runtime's versions should be bumped in `node/Cargo.toml` and then we should try to build with the runtime-benchmarks feature enabled.

```bash
cargo build --release --features runtime-benchmarks
```

Once all of the nodes/pallets have been checked, `cargo fmt` passes and their tests pass a PR can be raised against the **integration** branch into main.
