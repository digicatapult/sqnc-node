import type { Codec, Registry } from '../../types';
import type { Check } from './types';
/** @internal */
export declare function decodeLatestMeta<Modules extends Codec>(registry: Registry, type: string, version: number, { compare, data }: Check): void;
/** @internal */
export declare function toLatest<Modules extends Codec>(registry: Registry, version: number, { data }: Check, withThrow?: boolean): void;
/** @internal */
export declare function defaultValues(registry: Registry, { data, fails }: Check, withThrow?: boolean, withFallbackCheck?: boolean): void;
export declare function testMeta(version: number, matchers: Record<string, Check>, withFallback?: boolean): void;
