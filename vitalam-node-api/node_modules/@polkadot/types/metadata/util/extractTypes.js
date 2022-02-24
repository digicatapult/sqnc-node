// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0
// we are attempting to avoid circular refs, hence the path import
import { getTypeDef } from "../../create/getTypeDef.js";
import { TypeDefInfo } from "../../create/types.js";

/** @internal */
export function extractTypes(types) {
  return types.map(type => {
    const decoded = getTypeDef(type);

    switch (decoded.info) {
      case TypeDefInfo.Plain:
        return decoded.type;

      case TypeDefInfo.BTreeSet:
      case TypeDefInfo.Compact:
      case TypeDefInfo.Option:
      case TypeDefInfo.Vec:
      case TypeDefInfo.VecFixed:
        return extractTypes([decoded.sub.type]);

      case TypeDefInfo.BTreeMap:
      case TypeDefInfo.HashMap:
      case TypeDefInfo.Result:
      case TypeDefInfo.Tuple:
        return extractTypes(decoded.sub.map(({
          type
        }) => type));

      default:
        throw new Error(`Unhandled: Unable to create and validate type from ${type}`);
    }
  });
}