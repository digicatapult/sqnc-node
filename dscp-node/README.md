The following environment variables are used by the `@digicatapult/dscp-node` package and can be configured.

| variable                      | required | default | description                                        |
| :---------------------------- | :------: | :-----: | :------------------------------------------------- |
| METADATA_KEY_LENGTH           |    N     |  `32`   | Fixed length of metadata keys                      |
| METADATA_VALUE_LITERAL_LENGTH |    N     |  `32`   | Fixed length of metadata LITERAL values            |
| MAX_METADATA_COUNT            |    N     |  `16`   | Maximum number of metadata items allowed per token |

The following environment variables are used for testing:
| variable  | required |   default   | description |
| :-------: | :------: | :---------: | :---------: |
| LOG_LEVEL |    N     |   `info`    | Log level   |
| API_HOST  |    N     | `localhost` | Node host   |
| API_PORT  |    N     |   `9944`    | Node port   |

## Testing

Follow instructions in the README at root of this repo to build a node. Start a dev node with:

```
./target/release/dscp-node --dev
```

Then run tests with

```
cd dscp-node/
npm run test
```
