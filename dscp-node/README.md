# @digicatapult/dscp-node

A package that exports the additional types of `dscp-node` to be used when interacting with the node via `polkadot.js` For an example, see [`dscp-api`](https://github.com/digicatapult/dscp-api/blob/main/app/util/substrateApi.js).

The following environment variables are used by `@digicatapult/dscp-node` and can be configured.

| variable                      | required | default | description                                        |
| :---------------------------- | :------: | :-----: | :------------------------------------------------- |
| METADATA_KEY_LENGTH           |    N     |  `32`   | Fixed length of metadata keys                      |
| METADATA_VALUE_LITERAL_LENGTH |    N     |  `32`   | Fixed length of metadata LITERAL values            |
| MAX_METADATA_COUNT            |    N     |  `16`   | Maximum number of metadata items allowed per token |
