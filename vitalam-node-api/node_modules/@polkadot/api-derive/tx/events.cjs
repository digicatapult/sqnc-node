"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.events = events;

var _rxjs = require("rxjs");

var _index = require("../util/index.cjs");

// Copyright 2017-2021 @polkadot/api-derive authors & contributors
// SPDX-License-Identifier: Apache-2.0
function events(instanceId, api) {
  return (0, _index.memo)(instanceId, at => (0, _rxjs.combineLatest)([api.query.system.events.at(at), api.rpc.chain.getBlock(at)]).pipe((0, _rxjs.map)(([events, block]) => ({
    block,
    events
  }))));
}