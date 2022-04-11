const { Keyring } = require('@polkadot/api')

const api = require('./api')

const utf8ToUint8Array = (str, len) => {
  const arr = new Uint8Array(len)
  try {
    arr.set(Buffer.from(str, 'utf8'))
  } catch (err) {
    if (err instanceof RangeError) {
      throw new Error(`${str} is too long. Max length: ${len} bytes`)
    } else throw err
  }
  return arr
}

const getLastTokenId = async () => {
  await api.isReady
  const lastTokenId = await api.query.simpleNftModule.lastToken()
  return lastTokenId ? parseInt(lastTokenId, 10) : -1
}

const getToken = async (tokenId) => {
  await api.isReady
  const token = await api.query.simpleNftModule.tokensById(tokenId)
  return token.toJSON()
}

const runProcess = async (process, inputs, outputs) => {
  await api.isReady
  const keyring = new Keyring({ type: 'sr25519' })
  const alice = keyring.addFromUri('//Alice')

  return new Promise((resolve, reject) => {
    let unsub = null
    api.tx.simpleNftModule
      .runProcess(process, inputs, outputs)
      .signAndSend(alice, (result) => {
        if (result.status.isInBlock) {
          const errors = result.events
            .filter(({ event: { method } }) => method === 'ExtrinsicFailed')
            .map(({ event: { data } }) => data[0])

          if (errors.length > 0) {
            reject('ExtrinsicFailed error in simpleNftModule')
          }

          const tokens = result.events
            .filter(({ event: { method } }) => method === 'Minted')
            .map(({ event: { data } }) => data[0].toNumber())

          unsub()
          resolve(tokens)
        }
      })
      .then((res) => {
        unsub = res
      })
      .catch((err) => {
        throw err
      })
  })
}

module.exports = { getLastTokenId, getToken, runProcess, utf8ToUint8Array }
