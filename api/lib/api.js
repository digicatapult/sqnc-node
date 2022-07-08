const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api')

const api = ({ options }) => {
  const { apiHost = 'localhost', apiPort = 9944 } = options

  const provider = new WsProvider(`ws://${apiHost}:${apiPort}`)

  const apiOptions = {
    provider,
  }

  const api = new ApiPromise(apiOptions)
  api.isReadyOrError.catch(() => {})

  const keyring = new Keyring({ type: 'sr25519' })

  return {
    api,
    keyring,
  }
}

module.exports = api
