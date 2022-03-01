# VITALam node api

## Description

This is exists within the `./vitalam-node` directory as an NPM module to provide the following api related exports:
- `api` for connecting to the substrate-node via `@polkadot/api` npm dependency
- `rolesEnum` requirement for the substrate-node initialisation
- `Keyring` from the `@polkadot/api` 

A `Node.js` API to support communication to the [Substrate-based](https://www.substrate.io/) [`vitalam-node`](https://github.com/digicatapult/vitalam-node) (via [`polkadot-js/api`](https://www.npmjs.com/package/@polkadot/api)).

## Configuration

The following environment variables are used by `vitalam-node-api` and can be configured. Entries marked as `required` are needed when running `vitalam-node-api` in production mode.

| variable                      | required |                       default                       | description                                                                                                  |
| :---------------------------- | :------: | :-------------------------------------------------: | :----------------------------------------------------------------------------------------------------------- |
| METADATA_KEY_LENGTH           |    N     |                        `32`                         | Fixed length of metadata keys                                                                                |
| METADATA_VALUE_LITERAL_LENGTH |    N     |                        `32`                         | Fixed length of metadata LITERAL values                                                                      |
| MAX_METADATA_COUNT            |    N     |                        `16`                         | Maximum number of metadata items allowed per token                                                           |
