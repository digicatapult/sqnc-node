// Copyright 2017-2019 @polkadot/api authors & contributors
// SPDX-License-Identifier: Apache-2.0
import { assert, isUndefined } from '@polkadot/util';

function sig(_, {
  method,
  section
}, args) {
  return `${section}.${method}(${args.join(', ')})`;
}

function doDoubleMap(registry, creator, args) {
  const {
    key1,
    key2
  } = creator.meta.type.asDoubleMap;
  assert(args.length === 2, () => `${sig(registry, creator, [key1, key2])} is a double map, requiring 2 arguments, ${args.length} found`); // pass as tuple

  return [creator, args];
}

function doMap(registry, creator, args) {
  const {
    key
  } = creator.meta.type.asMap;
  assert(args.length === 1, () => `${sig(registry, creator, [key])} is a map, requiring 1 argument, ${args.length} found`); // expand

  return args.length ? [creator, args[0]] : [creator];
}

function doNMap(registry, creator, args) {
  const {
    keyVec
  } = creator.meta.type.asNMap;
  assert(args.length === keyVec.length, () => `${sig(registry, creator, keyVec)} is a multi map, requiring ${keyVec.length} arguments, ${args.length} found`); // pass as tuple

  return [creator, args];
} // sets up the arguments in the form of [creator, args] ready to be used in a storage
// call. Additionally, it verifies that the correct number of arguments have been passed


export function extractStorageArgs(registry, creator, _args) {
  const args = _args.filter(arg => !isUndefined(arg));

  if (creator.meta.type.isDoubleMap) {
    return doDoubleMap(registry, creator, args);
  } else if (creator.meta.type.isMap) {
    return doMap(registry, creator, args);
  } else if (creator.meta.type.isNMap) {
    return doNMap(registry, creator, args);
  }

  assert(args.length === 0, () => `${sig(registry, creator, [])} does not take any arguments, ${args.length} found`); // no args

  return [creator];
}