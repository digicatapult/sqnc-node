"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = void 0;
// Copyright 2017-2021 @polkadot/types-known authors & contributors
// SPDX-License-Identifier: Apache-2.0

/* eslint-disable sort-keys */
// these are override types for Statemine, Statemint, Westmint
const versioned = [{
  minmax: [0, undefined],
  types: {
    TAssetBalance: 'u128',
    ProxyType: {
      _enum: ['Any', 'NonTransfer', 'CancelProxy', 'Assets', 'AssetOwner', 'AssetManager', 'Staking']
    },
    AssetInstance: 'AssetInstanceV0',
    MultiAsset: 'MultiAssetV0',
    MultiLocation: 'MultiLocationV0',
    Response: 'ResponseV0',
    Xcm: 'XcmV0',
    XcmOrder: 'XcmOrderV0'
  }
}];
var _default = versioned;
exports.default = _default;