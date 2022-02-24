"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.paramsNotation = paramsNotation;
exports.encodeTypeDef = encodeTypeDef;
exports.withTypeString = withTypeString;

var _defineProperty2 = _interopRequireDefault(require("@babel/runtime/helpers/defineProperty"));

var _util = require("@polkadot/util");

var _types = require("./types.cjs");

function ownKeys(object, enumerableOnly) { var keys = Object.keys(object); if (Object.getOwnPropertySymbols) { var symbols = Object.getOwnPropertySymbols(object); if (enumerableOnly) { symbols = symbols.filter(function (sym) { return Object.getOwnPropertyDescriptor(object, sym).enumerable; }); } keys.push.apply(keys, symbols); } return keys; }

function _objectSpread(target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i] != null ? arguments[i] : {}; if (i % 2) { ownKeys(Object(source), true).forEach(function (key) { (0, _defineProperty2.default)(target, key, source[key]); }); } else if (Object.getOwnPropertyDescriptors) { Object.defineProperties(target, Object.getOwnPropertyDescriptors(source)); } else { ownKeys(Object(source)).forEach(function (key) { Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key)); }); } } return target; }

const stringIdentity = value => value.toString();

const INFO_WRAP = ['BTreeMap', 'BTreeSet', 'Compact', 'HashMap', 'Option', 'Result', 'Vec'];

function paramsNotation(outer, inner, transform = stringIdentity) {
  return `${outer}${inner ? `<${(Array.isArray(inner) ? inner : [inner]).map(transform).join(', ')}>` : ''}`;
}

function encodeWithParams(registry, typeDef, outer) {
  const {
    info,
    sub
  } = typeDef;

  switch (info) {
    case _types.TypeDefInfo.BTreeMap:
    case _types.TypeDefInfo.BTreeSet:
    case _types.TypeDefInfo.Compact:
    case _types.TypeDefInfo.HashMap:
    case _types.TypeDefInfo.Linkage:
    case _types.TypeDefInfo.Option:
    case _types.TypeDefInfo.Result:
    case _types.TypeDefInfo.Vec:
      return paramsNotation(outer, sub, param => encodeTypeDef(registry, param));
  }

  throw new Error(`Unable to encode ${(0, _util.stringify)(typeDef)} with params`);
} // eslint-disable-next-line @typescript-eslint/no-unused-vars


function encodeDoNotConstruct(registry, {
  displayName
}) {
  return `DoNotConstruct<${displayName || 'Unknown'}>`;
}

function encodeSubTypes(registry, sub, asEnum, extra) {
  const names = sub.map(({
    name
  }) => name);
  (0, _util.assert)(names.every(n => !!n), () => `Subtypes does not have consistent names, ${names.join(', ')}`);
  const inner = sub.reduce((result, type) => _objectSpread(_objectSpread({}, result), {}, {
    [type.name]: encodeTypeDef(registry, type)
  }), _objectSpread({}, extra));
  return (0, _util.stringify)(asEnum ? {
    _enum: inner
  } : inner);
}

function encodeEnum(registry, typeDef) {
  (0, _util.assert)(typeDef.sub && Array.isArray(typeDef.sub), 'Unable to encode Enum type');
  const sub = typeDef.sub; // c-like enums have all Null entries
  // TODO We need to take the disciminant into account and auto-add empty entries

  return sub.every(({
    type
  }) => type === 'Null') ? (0, _util.stringify)({
    _enum: sub.map(({
      name
    }, index) => `${name || `Empty${index}`}`)
  }) : encodeSubTypes(registry, sub, true);
}

function encodeStruct(registry, typeDef) {
  (0, _util.assert)(typeDef.sub && Array.isArray(typeDef.sub), 'Unable to encode Struct type');
  return encodeSubTypes(registry, typeDef.sub, false, _objectSpread({}, typeDef.alias ? {
    _alias: [...typeDef.alias.entries()].reduce((all, [k, v]) => _objectSpread(_objectSpread({}, all), {}, {
      [k]: v
    }), {})
  } : {}));
}

function encodeTuple(registry, typeDef) {
  (0, _util.assert)(typeDef.sub && Array.isArray(typeDef.sub), 'Unable to encode Tuple type');
  return `(${typeDef.sub.map(type => encodeTypeDef(registry, type)).join(',')})`;
}

function encodeUInt(registry, {
  length
}, type) {
  (0, _util.assert)((0, _util.isNumber)(length), 'Unable to encode VecFixed type');
  return `${type}<${length}>`;
} // eslint-disable-next-line @typescript-eslint/no-unused-vars


function encodeVecFixed(registry, {
  length,
  sub
}) {
  (0, _util.assert)((0, _util.isNumber)(length) && !(0, _util.isUndefined)(sub) && !Array.isArray(sub), 'Unable to encode VecFixed type');
  return `[${sub.type};${length}]`;
} // We setup a record here to ensure we have comprehensive coverage (any item not covered will result
// in a compile-time error with the missing index)


const encoders = {
  [_types.TypeDefInfo.BTreeMap]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'BTreeMap'),
  [_types.TypeDefInfo.BTreeSet]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'BTreeSet'),
  [_types.TypeDefInfo.Compact]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'Compact'),
  [_types.TypeDefInfo.DoNotConstruct]: (registry, typeDef) => encodeDoNotConstruct(registry, typeDef),
  [_types.TypeDefInfo.Enum]: (registry, typeDef) => encodeEnum(registry, typeDef),
  [_types.TypeDefInfo.HashMap]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'HashMap'),
  [_types.TypeDefInfo.Int]: (registry, typeDef) => encodeUInt(registry, typeDef, 'Int'),
  [_types.TypeDefInfo.Linkage]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'Linkage'),
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  [_types.TypeDefInfo.Null]: (registry, typeDef) => 'Null',
  [_types.TypeDefInfo.Option]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'Option'),
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  [_types.TypeDefInfo.Plain]: (registry, typeDef) => typeDef.displayName || typeDef.type,
  [_types.TypeDefInfo.Range]: (registry, typeDef) => encodeWithParams(registry, typeDef, typeDef.type.includes('RangeInclusive') ? 'RangeInclusive' : 'Range'),
  [_types.TypeDefInfo.Result]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'Result'),
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  [_types.TypeDefInfo.Set]: (registry, typeDef) => typeDef.type,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  [_types.TypeDefInfo.Si]: (registry, typeDef) => typeDef.lookupName || typeDef.type,
  [_types.TypeDefInfo.Struct]: (registry, typeDef) => encodeStruct(registry, typeDef),
  [_types.TypeDefInfo.Tuple]: (registry, typeDef) => encodeTuple(registry, typeDef),
  [_types.TypeDefInfo.UInt]: (registry, typeDef) => encodeUInt(registry, typeDef, 'UInt'),
  [_types.TypeDefInfo.Vec]: (registry, typeDef) => encodeWithParams(registry, typeDef, 'Vec'),
  [_types.TypeDefInfo.VecFixed]: (registry, typeDef) => encodeVecFixed(registry, typeDef)
};

function encodeType(registry, typeDef, withLookup = true) {
  const encoder = encoders[typeDef.info];
  (0, _util.assert)(encoder, () => `Cannot encode type ${(0, _util.stringify)(typeDef)}`);
  return withLookup && typeDef.lookupName ? typeDef.lookupName : encoder(registry, typeDef);
}

function encodeTypeDef(registry, typeDef) {
  (0, _util.assert)(!(0, _util.isUndefined)(typeDef.info), () => `Invalid type definition with no instance info, typeDef=${(0, _util.stringify)(typeDef)}`); // In the case of contracts we do have the unfortunate situation where the displayName would
  // refer to "Option" when it is an option. For these, string it out, only using when actually
  // not a top-level element to be used

  if (typeDef.displayName && !INFO_WRAP.some(i => typeDef.displayName === i)) {
    return typeDef.displayName;
  }

  return encodeType(registry, typeDef);
}

function withTypeString(registry, typeDef) {
  return _objectSpread(_objectSpread({}, typeDef), {}, {
    type: encodeType(registry, typeDef, false)
  });
}