# Upgrading Substrate Dependencies

This guide will hopefull help in the future when it comes to upgrading Substrate. It was updated from `0.9.25` to `0.9.30` and the issues it caused can be seen [here](https://github.com/digicatapult/dscp-node/pull/91/files)

# First Thing

Make sure you have pulled the main branch. Tthen checkout a integration branch, this branch is where all other project branches will be merged, **NOT INTO MAIN**. As Matt said "that will break up the reviews and make the changes more understandable...and it provides a point of integration so we’re not doing weird `git reset` commands or the like".

# Rustup

You'll need rustup up so run:

    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```

The you'll need the scripts:

    ```bash
    ./scripts/init.sh
    ```

First search for the version you want to change (search/replace), then replace it with the version you require.

After making the changes build the repo

```bash
cargo build --release
```

Depending on the lenght of time between versions there may be a **lot** of changes.

If there are errors you will need to investigate, some may be obvious but it might be worth looking at the latest version of the branch or use any documentation if available.

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

Perhaps the most [difficult part is the changes to the code](https://github.com/digicatapult/dscp-node/pull/91/files#diff-6d40c1b90e071cdb5271cce23374b2ecae20ab264980fda18a4d4d4c290efca1L66), if you look at the original to the new version there are substancial changes (depending on the update).
