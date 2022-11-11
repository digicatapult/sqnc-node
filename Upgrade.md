# Description

This guide will hopefully provide help to upgrade Substrate in the future.

This is a guide based on updating from `0.9.25` to `0.9.30` and the issues it caused can be seen [here](https://github.com/digicatapult/dscp-node/pull/91/files)

## Prerequisities

Make sure **main** is up to date and create an **integration** branch which all other branches will be PR'd into. This reduces git issues like having to use `git reset`.

You will need to run `rustup` which is [documented here](https://github.com/digicatapult/dscp-node/blob/main/README.md).

## Upgrade Substrate

This [diener tool](https://crates.io/crates/diener) can be used to update the toml versions easily by upgrading each `cargo.toml` given a specific branch/path.

## Build the Node

To build the Node:

```bash
cargo build --release
```

## Pallets

After the version updates have been made, depending on the length of time between version releases, there may be a **lot** of changes.

If a file has errors run `cargo check` and it will only check that one file.

If there are errors you will need to investigate. It would also be worth looking at the [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template) to see what what changes they have made when updgrading and potentially compare code changes between different tagged releases of substrate.

For example, during the `0.9.30` upgrade `Event` and `Origin` became `RuntimeEvent` and
`RuntimeOrigin`. This information was
obtained from checking the [Polkadot `0.9.30` branch](https://github.com/paritytech/substrate/tree/polkadot-v0.9.30). Checking the branch will help with renaming and syntax changes.

[Version changes can be the most difficult parts of the code](https://github.com/digicatapult/dscp-node/pull/91/files#diff-6d40c1b90e071cdb5271cce23374b2ecae20ab264980fda18a4d4d4c290efca1), if you look at the original compared against the new version there could substancial, or minor, changes (depending on the update).

After the change each pallet needs to be inspected, fixed if needed, along with fixes to tests, ensuring the runtime-benchmarks feature build.

```bash
cargo build --release --features runtime-benchmarks
```

Once a pallet has been brought up to date it needs to be tested, something like `cargo test -p pallet-transaction-payment-free`, of course change the pallet name to the pallet you are on. If it passes, push it into it's own PR, then into the **integration**. If there are errors Rust is very good at highlighting issues and suggesting looking error codes `rustc --explain E0152 `.

## Runtime

In the Runtime file, lib.rs, the `spec_version` must be changed to a set of numbers formatted as three digits rather than `5.6.8`, so instead of `5.6.8` it would be `568`, for example `spec_version: 444`.

If the tests pass create a PR from the **integration** branch.

## Node

The last stop in the node section is to update the dscp-node.

There are two steps to undertake, the dscp-node version must be changed and then and then completion of the tests. If this passes then check runtime-benchmarks builds.

```bash
cargo build --release --features runtime-benchmarks
```

Once all of the nodes/pallets have been checked, `cargo fmt` passes and their tests pass a PR can be raised against the **integration** branch into main.
