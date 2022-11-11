# Description

This guide will hopefully help upgrade Substrate in the future.

This is a guide based on updating from `0.9.25` to `0.9.30` and the issues it caused can be seen [here](https://github.com/digicatapult/dscp-node/pull/91/files)

# First Thing

Create an **integration** branch which all other branches will be PR'd into. This reduces git issues like having to use `git reset`.

Upgrade the references to the [paritytech/substrate](https://github.com/paritytech/substrate) repo in each of the workspace cargo.toml files. Doing this it will help ensure that the `cargo build` (as seen below) works and the rest of the process can take place.

You will need to run `rustup` which is [documented here](https://github.com/digicatapult/dscp-node/blob/main/README.md).

# Upgrade Substrate

This [diener tool](https://crates.io/crates/diener) can be used to update the file versions fairly painlesslly.

After the version updates have been made, depending on the length of time between version releases, there may be a **lot** of changes.

If there are errors you will need to investigate, some may be obvious but it will also be useful to look at the latest version of the branch or use any documentation if available. It may also be worth looking at the [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template) to see what what changes they have made when updgrading.

For example, during the `0.9.30` upgrade `Event` and `Origin` became `RuntimeEvent` and
`RuntimeOrigin`. This information was
obtained from checking the [Polkadot `0.9.30` branch](https://github.com/paritytech/substrate/tree/polkadot-v0.9.30). Checking the branch will help with renaming and syntax changes.

# Build the Node

To build the Node:

```bash
cargo build --release
```

# Pallets

[Version changes can be the most difficult parts of the code](https://github.com/digicatapult/dscp-node/pull/91/files#diff-6d40c1b90e071cdb5271cce23374b2ecae20ab264980fda18a4d4d4c290efca1), if you look at the original compared against the new version there could substancial changes (depending on the update).

After the change each pallet needs to be inspected, fixed if needed, along with fixes to tests and ensuring the runtime-benchmarks feature build.

Once a pallet has been brought up to date it needs to be tested, something like `cargo test -p pallet-transaction-payment-free`, of course change the pallet name to what is necessary. If it passes push it and potentially into it's own PR into the **integration**. If there are errors Rust is very good at highlighting issues and suggesting looking error codes `rustc --explain E0152 `.

## Runtime

The final step is to update the dscp-node. There are several steps to do now, the dscp-node version must be changed and then and then complete the tests and check runtime-benchmarks builds. And in the Runtime file lib.rs file, roughly line 100, the version must be changed there as a set of numbers, so instead of `5.6.8` it would be `568`, for example `spec_version: 444`

If the tests pass create a PR from the **integration** branch.

Once all of the nodes/pallets have been checked and pass their tests a PR can be raised against the **integration** branch into main.
