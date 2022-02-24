import _defineProperty from "@babel/runtime/helpers/esm/defineProperty";

function ownKeys(object, enumerableOnly) { var keys = Object.keys(object); if (Object.getOwnPropertySymbols) { var symbols = Object.getOwnPropertySymbols(object); if (enumerableOnly) { symbols = symbols.filter(function (sym) { return Object.getOwnPropertyDescriptor(object, sym).enumerable; }); } keys.push.apply(keys, symbols); } return keys; }

function _objectSpread(target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i] != null ? arguments[i] : {}; if (i % 2) { ownKeys(Object(source), true).forEach(function (key) { _defineProperty(target, key, source[key]); }); } else if (Object.getOwnPropertyDescriptors) { Object.defineProperties(target, Object.getOwnPropertyDescriptors(source)); } else { ownKeys(Object(source)).forEach(function (key) { Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key)); }); } } return target; }

// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0
import { v0 } from "./v0.js"; // order important in structs... :)

/* eslint-disable sort-keys */

export const SiVariant = {
  name: 'Text',
  fields: 'Vec<SiField>',
  index: 'u8',
  docs: 'Vec<Text>'
};
export default {
  rpc: {},
  types: _objectSpread(_objectSpread({}, v0), {}, {
    SiField: {
      name: 'Option<Text>',
      type: 'SiLookupTypeId',
      typeName: 'Option<Text>',
      docs: 'Vec<Text>'
    },
    SiLookupTypeId: 'Compact<u32>',
    SiPath: 'Vec<Text>',
    SiType: {
      path: 'SiPath',
      params: 'Vec<SiTypeParameter>',
      def: 'SiTypeDef',
      docs: 'Vec<Text>'
    },
    SiTypeDef: {
      _enum: {
        Composite: 'SiTypeDefComposite',
        Variant: 'SiTypeDefVariant',
        Sequence: 'SiTypeDefSequence',
        Array: 'SiTypeDefArray',
        Tuple: 'SiTypeDefTuple',
        Primitive: 'SiTypeDefPrimitive',
        Compact: 'SiTypeDefCompact',
        BitSequence: 'SiTypeDefBitSequence',
        // NOTE: This is specific to the implementation for pre-v14 metadata
        // compatibility (always keep this as the last entry in the enum)
        HistoricMetaCompat: 'Type'
      }
    },
    SiTypeDefArray: {
      len: 'u32',
      type: 'SiLookupTypeId'
    },
    SiTypeDefBitSequence: {
      bitStoreType: 'SiLookupTypeId',
      bitOrderType: 'SiLookupTypeId'
    },
    SiTypeDefCompact: {
      type: 'SiLookupTypeId'
    },
    SiTypeDefComposite: {
      fields: 'Vec<SiField>'
    },
    SiTypeDefPrimitive: {
      _enum: ['Bool', 'Char', 'Str', 'U8', 'U16', 'U32', 'U64', 'U128', 'U256', 'I8', 'I16', 'I32', 'I64', 'I128', 'I256']
    },
    SiTypeDefSequence: {
      type: 'SiLookupTypeId'
    },
    SiTypeDefTuple: 'Vec<SiLookupTypeId>',
    SiTypeParameter: {
      name: 'Text',
      type: 'Option<SiLookupTypeId>'
    },
    SiTypeDefVariant: {
      variants: 'Vec<SiVariant>'
    },
    SiVariant
  })
};