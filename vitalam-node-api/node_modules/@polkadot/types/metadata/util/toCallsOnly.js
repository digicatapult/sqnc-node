// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0
function trimDocs(docs) {
  const strings = docs.map(doc => doc.toString().trim());
  const firstEmpty = strings.findIndex(doc => !doc.length);
  return firstEmpty === -1 ? strings : strings.slice(0, firstEmpty);
}

function mapCalls(registry, _calls) {
  const calls = _calls.unwrapOr(null);

  return registry.createType('Option<Vec<FunctionMetadataLatest>>', calls ? calls.map(({
    args,
    docs,
    name
  }) => registry.createType('FunctionMetadataLatest', {
    args,
    docs: trimDocs(docs),
    name
  })) : null);
}
/** @internal */


export function toCallsOnly(registry, {
  extrinsic,
  modules
}) {
  return registry.createType('MetadataLatest', {
    extrinsic,
    modules: modules.map(({
      calls,
      index,
      name
    }) => ({
      calls: mapCalls(registry, calls),
      index,
      name
    }))
  }).toJSON();
}