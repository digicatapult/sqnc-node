# DSCP node

This repository contains the source code for the blockchain nodes used in the DSCP project. The structure and code is heavily based on [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template). To use this repository, it's important to understand the [key concepts](https://substrate.dev/docs/en/) of Substrate, such as `FRAME`, `runtime`, `extrinsics` and `transaction weight`.

For more on governance visit [dscp-documentation](https://github.com/digicatapult/dscp-documentation/blob/main/docs/governance.md).

## Getting started

You will need a Rust environment, including targets for WASM builds. The former can be easily achieved using `rustup`:

```bash
curl https://sh.rustup.rs -sSf | sh
```

The latter by running a bundled script:

```bash
./scripts/init.sh
```

### Building the Node

To build the node with optimisations, you can run from the project root directory:

```bash
cargo build --release
```

### Running a dev chain node

The build will generate a `target` directory.
Running the release build:

```bash
./target/release/dscp-node --dev
```

Note that if you want to reset the state of your chain (for example because you've changed a storage format) you can call:

```bash
./target/release/dscp-node purge-chain --dev
```

This will delete your dev chain so that it can be started from scratch again.

### Node Authorization

The node uses the [node-authorization](https://docs.rs/pallet-node-authorization/3.0.0/pallet_node_authorization/index.html) pallet to manage a configurable set of nodes for a permissioned network. The pre-configured well-known network for `local` chain contains `Alice`, `Bob`, `Charlie` and `Eve`. A node will not peer with the rest of the network unless the owner (account) starts the node with a `node-key` that corresponds to their `PeerId` and `AccountId` saved in `wellKnownNodes` storage. The set of `PeerId`s is initially configured in [`GenesisConfig`](node/src/chain_spec.rs). For example, to run and peer `Alice` and `Bob`, call the following two commands:

```bash
./target/release/dscp-node \
--chain=local \
--base-path /tmp/validator1 \
--alice \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--port 30333 \
--ws-port 9944
```

```bash
./target/release/dscp-node \
--chain=local \
--base-path /tmp/validator2 \
--bob \
--node-key=0000000000000000000000000000000000000000000000000000000000000002 \
--port 30334 \
--ws-port 9945
```

For `dev` chain, the network only contains a node for `Alice` so other nodes will not peer unless added to the well-known network, either by editing `chain_spec.rs` or using [dispatchable calls](https://docs.rs/pallet-node-authorization/3.0.0/pallet_node_authorization/enum.Call.html) at runtime. Also see [example](https://digicatapult.atlassian.net/wiki/spaces/EN/pages/1738014910/Example).

### Calculating weights

To calculate the weights for a pallet you first must ensure the node is built with the benchmarking feature enabled:

```bash
cargo build --release --features runtime-benchmarks
```

Then you can run the benchmark tool with for example

```bash
./target/release/dscp-node benchmark pallet \
    --pallet 'pallet_simple_nft' \
    --extrinsic '*' \
    --repeat 1000 \
    --output ./weights/
```

The generated weights implementation should then be integrated into the `pallet_simple_nft` module.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then follow the instructions at the top of [`docker-compose.yaml`](docker-compose.yaml)

## API

Interaction with the node can be done using the `@digicatapult/dscp-node` package published on npmjs.

### SimpleNFT pallet

The `SimpleNFT` pallet exposes an extrinsic for minting/burning tokens and a storage format that allows their retrieval.

Note: The json object with types, described above, has been upgraded from `"Address": "AccountId", "LookupSource": "AccountId"` to `"Address": "MultiAddress", "LookupSource": "MultiAddress"` and it also needs to be used in conjunction with the new version of _PolkaDot JS_, **v4.7.2** or higher.

Two storage endpoints are then exposed under `SimpleNFT` for the id of the last token issued (`LastToken`) and a mapping of tokens by id (`TokensById`):

```rust
LastToken get(fn last_token): T::TokenId;
TokensById get(fn tokens_by_id): map T::TokenId => Token<T::AccountId, T::RoleKey, T::TokenId, T::BlockNumber, T::TokenMetadataKey, T::TokenMetadataValue>;
```

Tokens can be minted/burnt by calling the following extrinsic under `SimpleNFT`:

```rust
pub fn run_process(
            origin: OriginFor<T>,
            process: Option<ProcessId<T>>
            inputs: Vec<T::TokenId>,
            outputs: Vec<
              Output<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>
            >,
        ) -> dispatch::DispatchResult { ... }
```

All of this functionality can be easily accessed using [https://polkadot.js.org/apps](https://polkadot.js.org/apps) against a running `dev` node. You will need to add a network endpoint of `ws://localhost:9944` under `Settings` and apply the above type configurations in the `Settings/Developer` tab.

Pallet tests can be run with:

```bash
cargo test -p pallet-simple-nft
```

### ProcessValidation pallet

Pallet for defining process restrictions. Intended for use with `pallet-simple-nft`. Processes can be defined using the extrinsic `create_process`:

```rust
pub fn create_process(
  origin: OriginFor<T>,
  id: T::ProcessIdentifier,
  program: BoundedVec<
      BooleanExpressionSymbol<
          T::RoleKey,
          T::TokenMetadataKey,
          T::TokenMetadataValue,
          T::TokenMetadataValueDiscriminator
      >,
      T::MaxProcessProgramLength
  >) -> DispatchResultWithPostInfo;
```

And disabled using `disable_process`:

```rust
pub fn disable_process(
  origin: OriginFor<T>,
  id: T::ProcessIdentifier,
  version: T::ProcessVersion
) -> DispatchResultWithPostInfo;
```

#### Process program

The process `program` argument is formed of a `BoundedVec` of `BooleanExpressionSymbol`s with a configured maximum length in our runtime of 200 symbols. Each `BooleanExpressionSymbol` can be either a process `Restriction` (see below) which evaluates to a boolean value at runtime or a `BooleanOperation` which can perform an binary operation on a pair of boolean arguments. The program itself is then a binary expression tree written in `postfix` notation so the tree:

```
     OR
   /    \
 AND    OR
 / \    / \
A   B  C   D
```

Could be represented as:

```
[
  Restriction(A),
  Restriction(B),
  Op(AND),
  Restriction(C),
  Restriction(D),
  Op(OR),
  Op(AND)
]
```

#### Binary Operations

A complete truth table set of binary operators is available when writing a process program. The table below describes each operation:

| Operation      | description              |
| :------------- | :----------------------- |
| `Null`         | false                    |
| `Identity`     | true                     |
| `TransferL`    | A                        |
| `TransferR`    | B                        |
| `NotL`         | !A                       |
| `NotR`         | !B                       |
| `And`          | A and B                  |
| `Nand`         | !(A and B)               |
| `Or`           | A or B                   |
| `Nor`          | !(A or B)                |
| `Xor`          | (A and !B) or (!A and B) |
| `Xnor`         | A equals B               |
| `ImplicationL` | if(A) then B else true   |
| `ImplicationR` | if(B) then A else true   |
| `InhibitionL`  | A and !B                 |
| `InhibitionR`  | B and !A                 |

#### Restrictions

The pallet defines various type of process restrictions that can be applied to a process. These include:

| Restriction                     |                                                                                  description                                                                                   |
| :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| `None`                          |                                                                Default `Restriction` value that always succeeds                                                                |
| `Fail`                          |                                                                     `Restriction` value that always fails                                                                      |
| `Combined`                      |                            Requires two specified restrictions combined via a specified operator [`AND`, `OR`, `XOR`, `NAND`, `NOR`] returns `true`                            |
| `SenderOwnsAllInputs`           |                                             Requires that the process `sender` is assigned the `default` role on all input tokens                                              |
| `SenderHasInputRole`            |                                    Requires that the process `sender` is assigned to a specified role on a specified (by index) input token                                    |
| `SenderHasOutputRole`           |                                   Requires that the process `sender` is assigned to a specified role on a specified (by index) output token                                    |
| `OutputHasRole`                 |                                                          Requires that a specified (by index) output token has a role                                                          |
| `MatchInputOutputRole`          |       Requires that the account of a specified role on a specified (by index) output token matches the account of a specified role on a specified (by index) input token       |
| `MatchInputOutputMetadataValue` | Requires that the metadata value of a specified key on a specified (by index) output token matches the metadata value of a specified key on a specified (by index) input token |
| `FixedNumberOfInputs`           |                                                         Requires that the number of inputs must be a specified integer                                                         |
| `FixedNumberOfOutputs`          |                                                        Requires that the number of outputs must be a specified integer                                                         |
| `FixedInputMetadataValue`       |                              Requires that a metadata item of a specified key must have a specified value, on a specified (by index) input token                               |
| `FixedOutputMetadataValue`      |                              Requires that a metadata item of a specified key must have a specified value, on a specified (by index) output token                              |
| `FixedOutputMetadataValueType`  |                         Requires that a metadata item of a specified key must have a value of a specified type, on a specified (by index) output token                         |

### IPFSKey pallet

The `IPFSKey` pallet facilitates the generation and scheduled rotation of a fixed length symmetric encryption key that is distributed to all chain participants. In this instance the key is to be used as an IPFS swarm key.

Two storage values are exposed by this pallet:

```rust
#[pallet::storage]
#[pallet::getter(fn key)]
pub(super) type Key<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

#[pallet::storage]
#[pallet::getter(fn key_schedule)]
pub(super) type KeyScheduleId<T: Config> = StorageValue<_, Option<Vec<u8>>, ValueQuery>;
```

The first exposes the maintained swarm key, whilst the latter the handle used with the `pallet-scheduling` frame pallet for setting a rotation schedule. This schedule is configured for a 7 day rotation.

Two extrinsics are exposed by this pallet, one for updating a shared symmetric key and one for forcing a rotation of the key based on a configured randomness source. In the `runtime` in this repository `update_key` can be called by either `sudo` or a simple majority of the Membership. `rotate_key` can be called either by `sudo` or a pair of accounts in the `Membership` set.

```rust
pub(super) fn update_key(origin: OriginFor<T>, new_key: Vec<u8>) -> DispatchResultWithPostInfo { ... }
pub(super) fn rotate_key(origin: OriginFor<T>) -> DispatchResultWithPostInfo { ... }
```

Pallet tests can be run with:

```bash
cargo test -p pallet-symmetric-key
```

### Doas pallet

The `Doas` pallet allows for a configurable `Origin` to execute dispatchable functions that require a `Root` call. This is seen as a more flexible of the [`sudo`](https://docs.rs/pallet-sudo/latest/pallet_sudo) pallet provided by ParityTech. This pallet may be used in conjunction with the [`collective`](https://docs.rs/pallet-sudo/latest/pallet_collective) pallet to enable `sudo` like functionality where a majority of the collective must agree to perform the action.

This pallet exposes three extrinsics:

```rust
pub(super) fn doas_root(origin: OriginFor<T>, call: Box<<T as Config>::Call>) -> DispatchResultWithPostInfo { ... }
pub(super) fn doas_root_unchecked_weight(origin: OriginFor<T>, call: Box<<T as Config>::Call>, _weight: Weight) -> DispatchResultWithPostInfo { ... }
pub(super) fn doas(origin: OriginFor<T>, who: <T::Lookup as StaticLookup>::Source, call: Box<<T as Config>::Call>) -> DispatchResultWithPostInfo { ... }
```

## Repo Structure

A Substrate project consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to
  [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
  network. Substrate makes it possible to supply custom consensus engines and also ships with
  several consensus mechanisms that have been built on top of
  [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A
  [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a
  source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
  are useful for development and testing, and critical when architecting the launch of a
  production chain. Take note of the `development_config` and `testnet_genesis` functions, which
  are used to define the genesis state for the local development chain configuration. These
  functions identify some
  [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys)
  and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
  the libraries that this file imports and the names of the functions it invokes. In particular,
  there are references to consensus-related topics, such as the
  [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule),
  the [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring
  mechanism and the
  [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality
  gadget.

### Runtime

In Substrate, the terms
"[runtime](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#runtime)" and
"[state transition function](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#stf-state-transition-function)"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
the [FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) framework to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://substrate.dev/docs/en/knowledgebase/runtime/macros) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://www.substrate.io/substrate-users/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) and note
the following:

- This file configures several pallets to include in the runtime. Each pallet configuration is
  defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- [`pallet_simple_nft`](#SimpleNFT-pallet) is custom to this project.
- The pallets are composed into a single runtime by way of the
  [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
  macro, which is part of the core
  [FRAME Support](https://substrate.dev/docs/en/knowledgebase/runtime/frame#support-library)
  library.

### Pallets

A FRAME pallet is compromised of a number of blockchain primitives:

- Storage: FRAME defines a rich set of powerful
  [storage abstractions](https://substrate.dev/docs/en/knowledgebase/runtime/storage) that makes
  it easy to use Substrate's efficient key-value database to manage the evolving state of a
  blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
  from outside of the runtime in order to update its state.
- Events: Substrate uses [events](https://substrate.dev/docs/en/knowledgebase/runtime/events) to
  notify users of important changes in the runtime.
- Errors: When a dispatchable fails, it returns an error.
- Config: The `Config` configuration interface is used to define the types and parameters upon
  which a FRAME pallet depends.
