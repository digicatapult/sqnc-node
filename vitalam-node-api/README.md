# VITALam node api

## Description

A `Node.js` API to support communication to the [Substrate-based](https://www.substrate.io/) [`vitalam-node`](https://github.com/digicatapult/vitalam-node) (via [`polkadot-js/api`](https://www.npmjs.com/package/@polkadot/api)).

## Configuration

The following environment variables are used by `vitalam-api` and can be configured. Entries marked as `required` are needed when running `vitalam-api` in production mode.

| variable                      | required |                       default                       | description                                                                                                  |
| :---------------------------- | :------: | :-------------------------------------------------: | :----------------------------------------------------------------------------------------------------------- |
| LEGACY_METADATA_KEY           |    N     |                         ''                          | Key given to token metadata posted without a key (such as when posted using the legacy `metadataFile` field) |
| METADATA_KEY_LENGTH           |    N     |                        `32`                         | Fixed length of metadata keys                                                                                |
| METADATA_VALUE_LITERAL_LENGTH |    N     |                        `32`                         | Fixed length of metadata LITERAL values                                                                      |
| MAX_METADATA_COUNT            |    N     |                        `16`                         | Maximum number of metadata items allowed per token                                                           |
