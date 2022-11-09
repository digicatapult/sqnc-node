# Upgrading Substrate Dependencies

This guide will hopefully help upgrade Substrate in the future.

This is a guide based on updating from `0.9.25` to `0.9.30` and the issues it caused can be seen [here](https://github.com/digicatapult/dscp-node/pull/91/files)

# First Thing

You will need to run `rustup` which is [documented here](https://github.com/digicatapult/dscp-node/blob/main/README.md).

# Build the Node

Then build the Node:

```bash
cargo build --release
```

Depending on the lenght of time between version leases there may be a **lot** of changes.

If there are errors you will need to investigate, some may be obvious but it will also be useful to look at the latest version of the branch or use any documentation if available.

For example, during the `0.9.30` upgrade `Event` and `Origin` became `RuntimeEvent` and
`RuntimeOrigin`. This information was
obtained from checking the [Polkadot `0.9.30` branch](https://github.com/paritytech/substrate/tree/polkadot-v0.9.31). Checking the branch will help with renaming and syntax changes.

# Code Changes

There can also be code changes, these will need to be researchded,for example:

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

Perhaps the most [difficult parts are the changes to the code](https://github.com/digicatapult/dscp-node/pull/91/files#diff-6d40c1b90e071cdb5271cce23374b2ecae20ab264980fda18a4d4d4c290efca1L66), if you look at the original to the new version there are substancial changes (depending on the update).
