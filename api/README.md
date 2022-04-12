# @digicatapult/dscp-node

A package for building a thin `polkadot.js` API that includes the additional types of `dscp-node`, to be used when interacting with the node.

Building an API with `@digicatapult/dscp-node`:

```js
const { buildApi } = require('@digicatapult/dscp-node')

const { api, types, keyring } = buildApi({
  options: {
    apiHost: 'localhost',
    apiPort: 9944,
    metadataKeyLength: 32,
    metadataValueLiteralLength: 32,
    processorIdentifierLength: 32,
    logger = { warn: () => {}, info: () => {}, error: () => {} },
  },
})
```

The following `options` can be configured:
| variable                   | required |                        default                        | description                                                           |
| :------------------------- | :------: | :---------------------------------------------------: | :-------------------------------------------------------------------- |
| apiHost                    |    N     |                      `localhost`                      | The hostname of the `dscp-node` the API should connect to             |
| apiPort                    |    N     |                        `9944`                         | The port of the `dscp-node` the API should connect to                 |
| metadataKeyLength          |    N     |                         `32`                          | Fixed length of `dscp-node` metadata key type                         |
| metadataValueLiteralLength |    N     |                         `32`                          | Fixed length of `dscp-node` metadata `LITERAL` value type             |
| processorIdentifierLength  |    N     |                         `32`                          | Fixed length of `dscp-node` process identifier type                   |
| logger                     |    N     | `{ warn: () => {}, info: () => {}, error: () => {} }` | An optional logger (such as `pino`) for logging API connection status |
