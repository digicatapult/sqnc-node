// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0
import { getModuleTypes, knownOrigins } from '@polkadot/types-known';
import { stringCamelCase } from '@polkadot/util';
const BOXES = [['<', '>'], ['<', ','], [',', '>'], ['(', ')'], ['(', ','], [',', ','], [',', ')']];
/**
 * Creates a compatible type mapping
 * @internal
 **/

function compatType(types, _type) {
  const type = _type.toString();

  const index = types.findIndex(({
    def
  }) => def.HistoricMetaCompat === type);

  if (index !== -1) {
    return index;
  }

  return types.push({
    def: {
      HistoricMetaCompat: type
    }
  }) - 1;
}

function variantType(modName, variantType, types, variants) {
  return types.push({
    def: {
      Variant: {
        variants
      }
    },
    path: [`pallet_${modName.toString()}`, 'pallet', variantType]
  }) - 1;
}
/**
 * @internal
 * generate & register the OriginCaller type
 **/


function registerOriginCaller(registry, modules, metaVersion) {
  registry.register({
    OriginCaller: {
      _enum: modules.map((mod, index) => [mod.name.toString(), metaVersion >= 12 ? mod.index.toNumber() : index]).sort((a, b) => a[1] - b[1]).reduce((result, [name, index]) => {
        for (let i = Object.keys(result).length; i < index; i++) {
          result[`Empty${i}`] = 'Null';
        }

        result[name] = knownOrigins[name] || 'Null';
        return result;
      }, {})
    }
  });
}
/**
 * Find and apply the correct type override
 * @internal
 **/


function setTypeOverride(sectionTypes, types) {
  types.forEach(type => {
    const override = Object.keys(sectionTypes).find(aliased => type.eq(aliased));

    if (override) {
      type.setOverride(sectionTypes[override]);
    } else {
      // FIXME: NOT happy with this approach, but gets over the initial hump cased by (Vec<Announcement>,BalanceOf)
      const orig = type.toString();
      const alias = Object.entries(sectionTypes).reduce((result, [src, dst]) => BOXES.reduce((result, [a, z]) => result.replace(`${a}${src}${z}`, `${a}${dst}${z}`), result), orig);

      if (orig !== alias) {
        type.setOverride(alias);
      }
    }
  });
}
/**
 * Apply module-specific type overrides (always be done as part of toV14)
 * @internal
 **/


function convertCalls(registry, types, modName, calls, sectionTypes) {
  const variants = calls.map(({
    args,
    docs,
    name
  }, index) => {
    setTypeOverride(sectionTypes, args.map(({
      type
    }) => type));
    return registry.createType('SiVariant', {
      docs,
      fields: args.map(({
        name,
        type
      }) => registry.createType('SiField', {
        name,
        type: compatType(types, type)
      })),
      index,
      name
    });
  });
  return registry.createType('PalletCallMetadataV14', {
    type: variantType(modName, 'Call', types, variants)
  });
}
/**
 * Apply module-specific type overrides (always be done as part of toV14)
 * @internal
 */


function convertConstants(registry, types, constants, sectionTypes) {
  return constants.map(({
    docs,
    name,
    type,
    value
  }) => {
    setTypeOverride(sectionTypes, [type]);
    return registry.createType('PalletConstantMetadataV14', {
      docs,
      name,
      type: compatType(types, type),
      value
    });
  });
}
/**
 * Apply module-specific type overrides (always be done as part of toV14)
 * @internal
 */
// eslint-disable-next-line @typescript-eslint/no-unused-vars


function convertErrors(registry, types, modName, errors, _sectionTypes) {
  const variants = errors.map(({
    docs,
    name
  }, index) => registry.createType('SiVariant', {
    docs,
    fields: [],
    index,
    name
  }));
  return registry.createType('PalletErrorMetadataV14', {
    type: variantType(modName, 'Error', types, variants)
  });
}
/**
 * Apply module-specific type overrides (always be done as part of toV14)
 * @internal
 **/


function convertEvents(registry, types, modName, events, sectionTypes) {
  const variants = events.map(({
    args,
    docs,
    name
  }, index) => {
    setTypeOverride(sectionTypes, args);
    return registry.createType('SiVariant', {
      docs,
      fields: args.map(t => registry.createType('SiField', {
        type: compatType(types, t)
      })),
      index,
      name
    });
  });
  return registry.createType('PalletEventMetadataV14', {
    type: variantType(modName, 'Event', types, variants)
  });
}

function createMapEntry(registry, sectionTypes, types, {
  hashers,
  keys,
  value
}) {
  setTypeOverride(sectionTypes, [value, ...keys]);
  return registry.createType('StorageEntryTypeV14', {
    Map: {
      hashers,
      key: types.push({
        def: {
          Tuple: keys.map(t => compatType(types, t))
        }
      }) - 1,
      value: compatType(types, value)
    }
  });
}
/**
 * Apply module-specific storage type overrides (always part of toV14)
 * @internal
 **/


function convertStorage(registry, types, {
  items,
  prefix
}, sectionTypes) {
  return registry.createType('PalletStorageMetadataV14', {
    items: items.map(({
      docs,
      fallback,
      modifier,
      name,
      type
    }) => {
      let entryType;

      if (type.isPlain) {
        const plain = type.asPlain;
        setTypeOverride(sectionTypes, [plain]);
        entryType = registry.createType('StorageEntryTypeV14', {
          Plain: compatType(types, plain)
        });
      } else if (type.isMap) {
        const map = type.asMap;
        entryType = createMapEntry(registry, sectionTypes, types, {
          hashers: [map.hasher],
          keys: [map.key],
          value: map.value
        });
      } else if (type.isDoubleMap) {
        const dm = type.asDoubleMap;
        entryType = createMapEntry(registry, sectionTypes, types, {
          hashers: [dm.hasher, dm.key2Hasher],
          keys: [dm.key1, dm.key2],
          value: dm.value
        });
      } else {
        const nm = type.asNMap;
        entryType = createMapEntry(registry, sectionTypes, types, {
          hashers: nm.hashers,
          keys: nm.keyVec,
          value: nm.value
        });
      }

      return registry.createType('StorageEntryMetadataV14', {
        docs,
        fallback,
        modifier,
        name,
        type: entryType
      });
    }),
    prefix
  });
}
/** @internal */
// eslint-disable-next-line @typescript-eslint/no-unused-vars


function convertExtrinsic(registry, _types, {
  signedExtensions,
  version
}) {
  return registry.createType('ExtrinsicMetadataV14', {
    signedExtensions: signedExtensions.map(identifier => ({
      identifier,
      type: 0 // we don't map the fields at all

    })),
    type: 0,
    // Map to extrinsic like in v14?
    version: version
  });
}
/** @internal */


function createPallet(registry, types, mod, {
  calls,
  constants,
  errors,
  events,
  storage
}) {
  const sectionTypes = getModuleTypes(registry, stringCamelCase(mod.name));
  return registry.createType('PalletMetadataV14', {
    calls: calls && convertCalls(registry, types, mod.name, calls, sectionTypes),
    constants: convertConstants(registry, types, constants, sectionTypes),
    errors: errors && convertErrors(registry, types, mod.name, errors, sectionTypes),
    events: events && convertEvents(registry, types, mod.name, events, sectionTypes),
    index: mod.index,
    name: mod.name,
    storage: storage && convertStorage(registry, types, storage, sectionTypes)
  });
}
/**
 * Convert the Metadata to v14
 * @internal
 **/


export function toV14(registry, v13, metaVersion) {
  // the types that we will pass
  const types = [];
  compatType(types, 'Null'); // position 0 always has Null

  registerOriginCaller(registry, v13.modules, metaVersion);
  const extrinsic = convertExtrinsic(registry, types, v13.extrinsic);
  const pallets = v13.modules.map(mod => createPallet(registry, types, mod, {
    calls: mod.calls.unwrapOr(null),
    constants: mod.constants,
    errors: mod.errors.length ? mod.errors : null,
    events: mod.events.unwrapOr(null),
    storage: mod.storage.unwrapOr(null)
  }));
  return registry.createType('MetadataV14', {
    extrinsic,
    lookup: {
      types: types.map((type, id) => registry.createType('PortableType', {
        id,
        type
      }))
    },
    pallets
  });
}