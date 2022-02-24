// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0
import fs from 'fs';
import path from 'path';
import { assert, hexToU8a, stringify, u8aToHex } from '@polkadot/util';
import { TypeRegistry } from "../../create/index.js";
import { unwrapStorageType } from "../../primitive/StorageKey.js";
import { Metadata } from "../Metadata.js";
import { getUniqTypes } from "./getUniqTypes.js";
/** @internal */

export function decodeLatestMeta(registry, type, version, {
  compare,
  data
}) {
  it('decodes metadata properly', () => {
    const metadata = new Metadata(registry, data);
    registry.setMetadata(metadata);
    expect(metadata.version).toBe(version);
    expect(metadata[`asV${version}`].modules.length).not.toBe(0);

    try {
      expect(metadata.toJSON()).toEqual(compare);
    } catch (error) {
      if (process.env.GITHUB_REPOSITORY) {
        console.error(stringify(metadata.toJSON()));
        throw error;
      }

      fs.writeFileSync(path.join(process.cwd(), `packages/types-support/src/metadata/v${version}/${type}-json.json`), stringify(metadata.toJSON(), 2), {
        flag: 'w'
      });
    }
  });
}
/** @internal */

export function toLatest(registry, version, {
  data
}, withThrow = true) {
  it(`converts v${version} to latest`, () => {
    const metadata = new Metadata(registry, data);
    registry.setMetadata(metadata);
    const metadataInit = metadata[`asV${version}`];
    const metadataLatest = metadata.asLatest;
    expect(getUniqTypes(registry, metadataInit, withThrow)).toEqual(getUniqTypes(registry, metadataLatest, withThrow));
  });
}
/** @internal */

export function defaultValues(registry, {
  data,
  fails = []
}, withThrow = true, withFallbackCheck = false) {
  describe('storage with default values', () => {
    const metadata = new Metadata(registry, data);
    metadata.asLatest.modules.filter(({
      storage
    }) => storage.isSome).forEach(m => {
      m.storage.unwrap().items.forEach(({
        fallback,
        modifier,
        name,
        type
      }) => {
        const inner = unwrapStorageType(registry, type, modifier.isOptional);
        const location = `${m.name.toString()}.${name.toString()}: ${inner}`;
        it(location, () => {
          expect(() => {
            try {
              const type = registry.createType(inner, hexToU8a(fallback.toHex()));

              if (withFallbackCheck) {
                const [hexType, hexOrig] = [u8aToHex(type.toU8a()), u8aToHex(fallback.toU8a(true))];
                assert(hexType === hexOrig, () => `Fallback does not match (${(hexOrig.length - 2) / 2 - (hexType.length - 2) / 2} bytes missing): ${hexType} !== ${hexOrig}`);
              }
            } catch (error) {
              const message = `${location}:: ${error.message}`;

              if (withThrow && !fails.some(f => location.includes(f))) {
                throw new Error(message);
              } else {
                console.warn(message);
              }
            }
          }).not.toThrow();
        });
      });
    });
  });
}
export function testMeta(version, matchers, withFallback = true) {
  describe(`MetadataV${version}`, () => {
    describe.each(Object.keys(matchers))('%s', type => {
      const matcher = matchers[type];
      const registry = new TypeRegistry();
      decodeLatestMeta(registry, type, version, matcher);
      toLatest(registry, version, matcher);
      defaultValues(registry, matcher, true, withFallback);
    });
  });
}