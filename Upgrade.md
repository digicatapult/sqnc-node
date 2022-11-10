# Upgrading Substrate Dependencies

This guide will hopefully help upgrade Substrate in the future.

This is a guide based on updating from `0.9.25` to `0.9.30` and the issues it caused can be seen [here](https://github.com/digicatapult/dscp-node/pull/91/files)

# First Thing

Create an **integration** branch which all other branches will be PR'd into. This reduces git issues like having to use `git reset`.

Upgrade the references in the repo. By doing this it will ensure that the `cargo build` (as seen below) works and the rest of the process can take place.

You will need to run `rustup` which is [documented here](https://github.com/digicatapult/dscp-node/blob/main/README.md).

# Build the Node

To build the Node:

```bash
cargo build --release
```

# Upgrade Substrate

This [tool](https://crates.io/crates/diener) can be used to update the version fairlessly painlesslly.

After the version updates have been made, depending on the length of time between version releases, there may be a **lot** of changes.

If there are errors you will need to investigate, some may be obvious but it will also be useful to look at the latest version of the branch or use any documentation if available.

For example, during the `0.9.30` upgrade `Event` and `Origin` became `RuntimeEvent` and
`RuntimeOrigin`. This information was
obtained from checking the [Polkadot `0.9.31` branch](https://github.com/paritytech/substrate/tree/polkadot-v0.9.31). Checking the branch will help with renaming and syntax changes.

# Code Change Example

This code requires a change to the parameter:

```rust
    let doas_root_unchecked_weight_call = DoasCall::doas_root_unchecked_weight { call, weight: 1_000 }
```

Becomes:

```rust
let doas_root_unchecked_weight_call = DoasCall::doas_root_unchecked_weight {
            call,
            weight: Weight::from_ref_time(1_000)
        };
```

[Version changes can be the most difficult parts of the code](https://github.com/digicatapult/dscp-node/pull/91/files#diff-6d40c1b90e071cdb5271cce23374b2ecae20ab264980fda18a4d4d4c290efca1), if you look at the original compared against the new version there could substancial changes (depending on the update).

After the change each pallet needs to be inspected, fixed if needed, along with fixes to tests and ensuring the runtime-benchmarks feature build.

Once a pallet is working it should be put into it's own PR into the **integration**.

## DSCP-Node

The final pallet to look at is the dscp-node. The first thing to do is update the dscp-node and then follow the previous tests from the other pallets and check runtime-benchmarks builds.

If the tests pass create a PR from the **integration** branch.

Once all of the pallets have been checked and pass their tests a PR can be raised against the **integration** branch into main.
