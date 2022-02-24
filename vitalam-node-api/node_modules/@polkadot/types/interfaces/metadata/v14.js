import _defineProperty from "@babel/runtime/helpers/esm/defineProperty";

function ownKeys(object, enumerableOnly) { var keys = Object.keys(object); if (Object.getOwnPropertySymbols) { var symbols = Object.getOwnPropertySymbols(object); if (enumerableOnly) { symbols = symbols.filter(function (sym) { return Object.getOwnPropertyDescriptor(object, sym).enumerable; }); } keys.push.apply(keys, symbols); } return keys; }

function _objectSpread(target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i] != null ? arguments[i] : {}; if (i % 2) { ownKeys(Object(source), true).forEach(function (key) { _defineProperty(target, key, source[key]); }); } else if (Object.getOwnPropertyDescriptors) { Object.defineProperties(target, Object.getOwnPropertyDescriptors(source)); } else { ownKeys(Object(source)).forEach(function (key) { Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key)); }); } } return target; }

// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0
// order important in structs... :)

/* eslint-disable sort-keys */
import { SiVariant } from "../scaleInfo/definitions.js";
export const v14 = {
  // registry
  PortableRegistry: 'GenericPortableRegistry',
  PortableType: {
    id: 'SiLookupTypeId',
    type: 'SiType'
  },
  // compatibility with earlier layouts, i.e. don't break previous users
  ErrorMetadataV14: _objectSpread(_objectSpread({}, SiVariant), {}, {
    args: 'Vec<Type>'
  }),
  EventMetadataV14: _objectSpread(_objectSpread({}, SiVariant), {}, {
    args: 'Vec<Type>'
  }),
  FunctionArgumentMetadataV14: 'FunctionArgumentMetadataV13',
  FunctionMetadataV14: _objectSpread(_objectSpread({}, SiVariant), {}, {
    args: 'Vec<FunctionArgumentMetadataLatest>'
  }),
  // V14
  ExtrinsicMetadataV14: {
    type: 'SiLookupTypeId',
    version: 'u8',
    signedExtensions: 'Vec<SignedExtensionMetadataV14>'
  },
  MetadataV14: {
    lookup: 'PortableRegistry',
    pallets: 'Vec<PalletMetadataV14>',
    extrinsic: 'ExtrinsicMetadataV14'
  },
  PalletCallMetadataV14: {
    type: 'SiLookupTypeId'
  },
  PalletConstantMetadataV14: {
    name: 'Text',
    type: 'SiLookupTypeId',
    value: 'Bytes',
    docs: 'Vec<Text>'
  },
  PalletErrorMetadataV14: {
    type: 'SiLookupTypeId'
  },
  PalletEventMetadataV14: {
    type: 'SiLookupTypeId'
  },
  PalletMetadataV14: {
    name: 'Text',
    storage: 'Option<PalletStorageMetadataV14>',
    calls: 'Option<PalletCallMetadataV14>',
    events: 'Option<PalletEventMetadataV14>',
    constants: 'Vec<PalletConstantMetadataV14>',
    errors: 'Option<PalletErrorMetadataV14>',
    index: 'u8'
  },
  PalletStorageMetadataV14: {
    prefix: 'Text',
    // NOTE: Renamed from entries
    items: 'Vec<StorageEntryMetadataV14>'
  },
  SignedExtensionMetadataV14: {
    identifier: 'Text',
    type: 'SiLookupTypeId',
    additionalSigned: 'SiLookupTypeId'
  },
  StorageEntryMetadataV14: {
    name: 'Text',
    modifier: 'StorageEntryModifierV14',
    type: 'StorageEntryTypeV14',
    fallback: 'Bytes',
    docs: 'Vec<Text>'
  },
  StorageEntryModifierV14: 'StorageEntryModifierV13',
  StorageEntryTypeV14: {
    _enum: {
      Plain: 'SiLookupTypeId',
      Map: {
        hashers: 'Vec<StorageHasherV14>',
        key: 'SiLookupTypeId',
        // NOTE: Renamed from "keys"
        value: 'SiLookupTypeId'
      }
    }
  },
  StorageHasherV14: 'StorageHasherV13'
};