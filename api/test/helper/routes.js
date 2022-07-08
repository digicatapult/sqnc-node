const { buildApi } = require('../../lib')

const setupApi = (context) => {
  Object.assign(
    context,
    buildApi({
      options: {},
    })
  )
}

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

const getLastTokenId = async (context) => {
  await context.api.isReady
  const lastTokenId = await context.api.query.simpleNFT.lastToken()
  return lastTokenId ? parseInt(lastTokenId, 10) : -1
}

const getToken = async (context, tokenId) => {
  await context.api.isReady
  const token = await context.api.query.simpleNFT.tokensById(tokenId)
  return token.toJSON()
}

const runProcess = async (context, process, inputs, outputs) => {
  await context.api.isReady
  const alice = context.keyring.addFromUri('//Alice')

  return new Promise((resolve, reject) => {
    let unsub = null
    context.api.tx.simpleNFT
      .runProcess(process, inputs, outputs)
      .signAndSend(alice, (result) => {
        if (result.status.isInBlock) {
          const errors = result.events
            .filter(({ event: { method } }) => method === 'ExtrinsicFailed')
            .map(({ event: { data } }) => data[0])

          if (errors.length > 0) {
            reject('ExtrinsicFailed error in simpleNFT')
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

module.exports = { setupApi, getLastTokenId, getToken, runProcess, utf8ToUint8Array }
