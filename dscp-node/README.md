# @digicatapult/dscp-node

A package for building a thin `polkadot.js` API that includes the additional types of `dscp-node`, to be used when interacting with the node.

Building an API with `@digicatapult/dscp-node`:

```js
const { buildApi } = require('@digicatapult/dscp-node')

const { api, types, keyring } = await buildApi({
  options: {
    apiHost: 'localhost',
    apiPort: 9944,
    metadataKeyLength: 32,
    metadataValueLiteralLength: 32,
    processorIdentifierLength: 32,
    logLevel: 'warn',
    keyringType: 'sr25519',
  },
})
```

The above example shows the default value for each option.
