const { describe, test, before, after, afterEach } = require('mocha')
const { expect } = require('chai')

const { api } = require('../helper/api')

describe('types', function () {
  describe('health check', function () {
    before(async function () {
      await api.isReady
    })

    test('types check', async function () {
      const lastTokenId = await api.query.simpleNftModule.lastToken()
      console.log(lastTokenId)
      expect(lastTokenId).to.deep.equal(0)
    })
  })
})
