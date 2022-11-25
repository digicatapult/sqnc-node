# @digicatapult/veritable-node

A package for building a thin `polkadot.js` API that includes the additional types of `veritable-node`, to be used when interacting with the node.

Building an API with `@digicatapult/dscp-node`:

```js
const { buildApi } = require('@digicatapult/dscp-node')

const { api, types, keyring } = buildApi({
  options: {
    apiHost: 'localhost',
    apiPort: 9944,
  },
})
```

The following `options` can be configured:
| variable                   | required |                        default                        | description                                                           |
| :------------------------- | :------: | :---------------------------------------------------: | :-------------------------------------------------------------------- |
| apiHost                    |    N     |                      `localhost`                      | The hostname of the `veritable-node` the API should connect to             |
| apiPort                    |    N     |                        `9944`                         | The port of the `veritable-node` the API should connect to                 |
