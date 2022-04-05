const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api')

const { METADATA_KEY_LENGTH, METADATA_VALUE_LITERAL_LENGTH, PROCESS_IDENTIFIER_LENGTH } = require('./env')

const rolesEnum = ['Owner', 'Customer', 'AdditiveManufacturer', 'Laboratory', 'Buyer', 'Supplier', 'Reviewer']

const types = {
  Address: 'MultiAddress',
  LookupSource: 'MultiAddress',
  PeerId: 'Vec<u8>',
  Key: 'Vec<u8>',
  TokenId: 'u128',
  RoleKey: 'Role',
  TokenMetadataKey: `[u8; ${METADATA_KEY_LENGTH}]`,
  TokenMetadataValue: 'MetadataValue',
  Token: {
    id: 'TokenId',
    original_id: 'TokenId',
    roles: 'BTreeMap<RoleKey, AccountId>',
    creator: 'AccountId',
    created_at: 'BlockNumber',
    destroyed_at: 'Option<BlockNumber>',
    metadata: 'BTreeMap<TokenMetadataKey, TokenMetadataValue>',
    parents: 'Vec<TokenId>',
    children: 'Option<Vec<TokenId>>',
  },
  ProcessIO: {
    roles: 'BTreeMap<RoleKey, AccountId>',
    metadata: 'BTreeMap<TokenMetadataKey, TokenMetadataValue>',
    parent_index: 'Option<u32>',
  },
  MetadataValue: {
    _enum: {
      File: 'Hash',
      Literal: `[u8; ${METADATA_VALUE_LITERAL_LENGTH}]`,
      TokenId: 'TokenId',
      None: null,
    },
  },
  Role: {
    _enum: rolesEnum,
  },
  ProcessIdentifier: `[u8; ${PROCESS_IDENTIFIER_LENGTH}]`,
  ProcessVersion: 'u32',
  ProcessId: {
    id: 'ProcessIdentifier',
    version: 'ProcessVersion',
  },
  Process: {
    status: 'ProcessStatus',
    restrictions: 'Vec<Restriction>',
  },
  ProcessStatus: {
    _enum: ['Disabled', 'Enabled'],
  },
  Restriction: {
    _enum: {
      None: '()',
      SenderOwnsAllInputs: '()',
      FixedNumberOfInputs: 'FixedNumberOfInputsRestriction',
      FixedNumberOfOutputs: 'FixedNumberOfOutputsRestriction',
      FixedMetadataValue: 'FixedMetadataValueRestriction',
    },
  },
  FixedNumberOfInputsRestriction: {
    num_inputs: 'u32',
  },
  FixedNumberOfOutputsRestriction: {
    num_outputs: 'u32',
  },
  FixedMetadataValueRestriction: {
    input_index: 'u32',
    metadata_key: 'TokenMetadataKey',
    metadata_value: 'TokenMetadataValue',
  },
  IsNew: 'bool',
  Restrictions: 'Vec<Restriction>',
}

const getApi = ({ host, port, logger }) => {
  const apiOptions = {
    provider: new WsProvider(`ws://${host}:${port}`),
    types,
  }

  const api = new ApiPromise(apiOptions)

  api.on('disconnected', () => {
    logger.warn(`Disconnected from substrate node at ${host}:${port}`)
  })

  api.on('connected', () => {
    logger.info(`Connected to substrate node at ${host}:${port}`)
  })

  api.on('error', (err) => {
    logger.error(`Error from substrate node connection. Error was ${err.message || JSON.stringify(err)}`)
  })

  return api
}

module.exports = {
  rolesEnum,
  getApi,
  Keyring,
}
