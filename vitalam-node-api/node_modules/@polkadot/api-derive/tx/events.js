// Copyright 2017-2021 @polkadot/api-derive authors & contributors
// SPDX-License-Identifier: Apache-2.0
import { combineLatest, map } from 'rxjs';
import { memo } from "../util/index.js";
export function events(instanceId, api) {
  return memo(instanceId, at => combineLatest([api.query.system.events.at(at), api.rpc.chain.getBlock(at)]).pipe(map(([events, block]) => ({
    block,
    events
  }))));
}