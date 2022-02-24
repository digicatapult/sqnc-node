import type { MetadataLatest } from '../../interfaces/metadata';
import type { AnyJson, Registry } from '../../types';
/** @internal */
export declare function toCallsOnly(registry: Registry, { extrinsic, modules }: MetadataLatest): AnyJson;
