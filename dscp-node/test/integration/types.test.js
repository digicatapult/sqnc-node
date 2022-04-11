const { describe, test } = require('mocha')
const { expect } = require('chai')

const { METADATA_KEY_LENGTH, METADATA_VALUE_LITERAL_LENGTH } = require('../../src/env')
const { runProcess, utf8ToUint8Array, getLastTokenId, getToken } = require('../helper/routes')

const USER_ALICE_TOKEN = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'

describe('types', function () {
  describe('health check', function () {
    test('mint token', async function () {
      const lastTokenId = await getLastTokenId()
      const roles = new Map(Object.entries({ 0: USER_ALICE_TOKEN }))
      const key = utf8ToUint8Array('test', METADATA_KEY_LENGTH)
      const value = utf8ToUint8Array('test', METADATA_VALUE_LITERAL_LENGTH)
      const metadata = new Map(Object.entries({ [key]: { Literal: value } }))
      const newToken = await runProcess(undefined, [], [[roles, metadata, undefined]])
      expect(newToken[0]).to.equal(lastTokenId + 1)
    })

    test('get token', async function () {
      const lastTokenId = await getLastTokenId()
      const token = await getToken(lastTokenId)
      expect(token.id).to.equal(lastTokenId)
    })
  })
})
