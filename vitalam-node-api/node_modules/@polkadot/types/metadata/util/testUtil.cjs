"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.decodeLatestMeta = decodeLatestMeta;
exports.toLatest = toLatest;
exports.defaultValues = defaultValues;
exports.testMeta = testMeta;

var _fs = _interopRequireDefault(require("fs"));

var _path = _interopRequireDefault(require("path"));

var _util = require("@polkadot/util");

var _index = require("../../create/index.cjs");

var _StorageKey = require("../../primitive/StorageKey.cjs");

var _Metadata = require("../Metadata.cjs");

var _getUniqTypes = require("./getUniqTypes.cjs");

// Copyright 2017-2021 @polkadot/types authors & contributors
// SPDX-License-Identifier: Apache-2.0

/** @internal */
function decodeLatestMeta(registry, type, version, {
  compare,
  data
}) {
  it('decodes metadata properly', () => {
    const metadata = new _Metadata.Metadata(registry, data);
    registry.setMetadata(metadata);
    expect(metadata.version).toBe(version);
    expect(metadata[`asV${version}`].modules.length).not.toBe(0);

    try {
      expect(metadata.toJSON()).toEqual(compare);
    } catch (error) {
      if (process.env.GITHUB_REPOSITORY) {
        console.error((0, _util.stringify)(metadata.toJSON()));
        throw error;
      }

      _fs.default.writeFileSync(_path.default.join(process.cwd(), `packages/types-support/src/metadata/v${version}/${type}-json.json`), (0, _util.stringify)(metadata.toJSON(), 2), {
        flag: 'w'
      });
    }
  });
}
/** @internal */


function toLatest(registry, version, {
  data
}, withThrow = true) {
  it(`converts v${version} to latest`, () => {
    const metadata = new _Metadata.Metadata(registry, data);
    registry.setMetadata(metadata);
    const metadataInit = metadata[`asV${version}`];
    const metadataLatest = metadata.asLatest;
    expect((0, _getUniqTypes.getUniqTypes)(registry, metadataInit, withThrow)).toEqual((0, _getUniqTypes.getUniqTypes)(registry, metadataLatest, withThrow));
  });
}
/** @internal */


function defaultValues(registry, {
  data,
  fails = []
}, withThrow = true, withFallbackCheck = false) {
  describe('storage with default values', () => {
    const metadata = new _Metadata.Metadata(registry, data);
    metadata.asLatest.modules.filter(({
      storage
    }) => storage.isSome).forEach(m => {
      m.storage.unwrap().items.forEach(({
        fallback,
        modifier,
        name,
        type
      }) => {
        const inner = (0, _StorageKey.unwrapStorageType)(registry, type, modifier.isOptional);
        const location = `${m.name.toString()}.${name.toString()}: ${inner}`;
        it(location, () => {
          expect(() => {
            try {
              const type = registry.createType(inner, (0, _util.hexToU8a)(fallback.toHex()));

              if (withFallbackCheck) {
                const [hexType, hexOrig] = [(0, _util.u8aToHex)(type.toU8a()), (0, _util.u8aToHex)(fallback.toU8a(true))];
                (0, _util.assert)(hexType === hexOrig, () => `Fallback does not match (${(hexOrig.length - 2) / 2 - (hexType.length - 2) / 2} bytes missing): ${hexType} !== ${hexOrig}`);
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

function testMeta(version, matchers, withFallback = true) {
  describe(`MetadataV${version}`, () => {
    describe.each(Object.keys(matchers))('%s', type => {
      const matcher = matchers[type];
      const registry = new _index.TypeRegistry();
      decodeLatestMeta(registry, type, version, matcher);
      toLatest(registry, version, matcher);
      defaultValues(registry, matcher, true, withFallback);
    });
  });
}