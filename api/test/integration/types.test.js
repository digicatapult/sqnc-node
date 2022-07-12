const { describe, test } = require('mocha')
const { expect } = require('chai')

const { setupApi, runProcess, utf8ToUint8Array, getLastTokenId, getToken } = require('../helper/routes')

const USER_ALICE_TOKEN = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'
const METADATA_KEY_LENGTH = 32
const METADATA_VALUE_LITERAL_LENGTH = 32

describe('types', function () {
  const context = {}

  before(async function () {
    setupApi(context)
  })

  describe('health check', function () {
    test('mint token', async function () {
      const lastTokenId = await getLastTokenId(context)
      const roles = new Map(Object.entries({ 0: USER_ALICE_TOKEN }))
      const key = utf8ToUint8Array('test', METADATA_KEY_LENGTH)
      const value = utf8ToUint8Array('test', METADATA_VALUE_LITERAL_LENGTH)
      const metadata = new Map([[key, { Literal: value }]])
      const parent_index = null
      const newToken = await runProcess(context, undefined, [], [{ roles, metadata, parent_index }])
      expect(newToken[0]).to.equal(lastTokenId + 1)
    })

    test('get token', async function () {
      const lastTokenId = await getLastTokenId(context)
      const token = await getToken(context, lastTokenId)
      expect(token.id).to.equal(lastTokenId)
    })
  })
})
