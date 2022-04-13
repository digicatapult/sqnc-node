const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api')

const api = ({ options }) => {
  const {
    apiHost = 'localhost',
    apiPort = 9944,
    metadataKeyLength = 32,
    metadataValueLiteralLength = 32,
    processorIdentifierLength = 32,
  } = options

  const provider = new WsProvider(`ws://${apiHost}:${apiPort}`)

  const types = {
    Address: 'MultiAddress',
    LookupSource: 'MultiAddress',
    PeerId: 'Vec<u8>',
    Key: 'Vec<u8>',
    TokenId: 'u128',
    RoleKey: 'Role',
    TokenMetadataKey: `[u8; ${metadataKeyLength}]`,
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
        Literal: `[u8; ${metadataValueLiteralLength}]`,
        TokenId: 'TokenId',
        None: null,
      },
    },
    Role: {
      _enum: ['Owner', 'Customer', 'AdditiveManufacturer', 'Laboratory', 'Buyer', 'Supplier', 'Reviewer'],
    },
    ProcessIdentifier: `[u8; ${processorIdentifierLength}]`,
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
        SenderHasInputRole: 'SenderHasInputRoleRestriction',
        FixedNumberOfInputs: 'FixedNumberOfInputsRestriction',
        FixedNumberOfOutputs: 'FixedNumberOfOutputsRestriction',
        FixedInputMetadataValue: 'FixedMetadataValueRestriction',
        FixedOutputMetadataValue: 'FixedMetadataValueRestriction',
      },
    },
    SenderHasInputRoleRestriction: {
      index: 'u32',
      role_key: 'RoleKey',
    },
    FixedNumberOfInputsRestriction: {
      num_inputs: 'u32',
    },
    FixedNumberOfOutputsRestriction: {
      num_outputs: 'u32',
    },
    FixedMetadataValueRestriction: {
      index: 'u32',
      metadata_key: 'TokenMetadataKey',
      metadata_value: 'TokenMetadataValue',
    },
    IsNew: 'bool',
    Restrictions: 'Vec<Restriction>',
  }

  const apiOptions = {
    provider,
    types,
  }

  const api = new ApiPromise(apiOptions)
  api.isReadyOrError.catch(() => {})

  const keyring = new Keyring({ type: 'sr25519' })

  return {
    api,
    types,
    keyring,
  }
}

module.exports = api
