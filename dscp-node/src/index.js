const { METADATA_KEY_LENGTH, METADATA_VALUE_LITERAL_LENGTH, PROCESS_IDENTIFIER_LENGTH } = require('./env')

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
    _enum: ['Owner', 'Customer', 'AdditiveManufacturer', 'Laboratory', 'Buyer', 'Supplier', 'Reviewer'],
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

module.exports = types
