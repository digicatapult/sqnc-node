import type { MetadataLatest } from '../../../interfaces';
import type { Registry } from '../../../types';
import type { Extrinsics } from '../types';
/** @internal */
export declare function decorateExtrinsics(registry: Registry, { modules }: MetadataLatest, metaVersion: number): Extrinsics;
