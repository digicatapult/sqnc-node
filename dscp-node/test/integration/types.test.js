const { describe, test, before } = require('mocha')
const { expect } = require('chai')

const { api } = require('../helper/api')

describe('types', function () {
  describe('health check', function () {
    before(async function () {
      await api.isReady
    })

    test('types check', async function () {
      const lastTokenId = await api.query.simpleNftModule.lastToken()
      const id = lastTokenId ? parseInt(lastTokenId, 10) : -1
      expect(id).to.equal(0)
    })
  })
})
