"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.substrate = void 0;

var _createFunction = require("./createFunction.cjs");

// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0
// Small helper function to factorize code on this page.

/** @internal */
function createRuntimeFunction(method, key, {
  docs,
  type
}) {
  return registry => (0, _createFunction.createFunction)(registry, {
    meta: {
      docs: registry.createType('Vec<Text>', [docs]),
      modifier: registry.createType('StorageEntryModifierLatest', 1),
      // required
      name: registry.createType('Text', method),
      toJSON: () => key,
      type: registry.createType('StorageEntryTypeLatest', type, 0)
    },
    method,
    prefix: 'Substrate',
    section: 'substrate'
  }, {
    key,
    skipHashing: true
  });
}

const substrate = {
  changesTrieConfig: createRuntimeFunction('changesTrieConfig', ':changes_trie', {
    docs: ' Changes trie configuration is stored under this key.',
    type: 'u32'
  }),
  childStorageKeyPrefix: createRuntimeFunction('childStorageKeyPrefix', ':child_storage:', {
    docs: ' Prefix of child storage keys.',
    type: 'u32'
  }),
  code: createRuntimeFunction('code', ':code', {
    docs: ' Wasm code of the runtime.',
    type: 'Bytes'
  }),
  extrinsicIndex: createRuntimeFunction('extrinsicIndex', ':extrinsic_index', {
    docs: ' Current extrinsic index (u32) is stored under this key.',
    type: 'u32'
  }),
  heapPages: createRuntimeFunction('heapPages', ':heappages', {
    docs: ' Number of wasm linear memory pages required for execution of the runtime.',
    type: 'u64'
  })
};
exports.substrate = substrate;