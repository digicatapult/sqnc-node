import type { MetadataV13, MetadataV14 } from '../../interfaces/metadata';
import type { Registry } from '../../types';
/**
 * Convert the Metadata to v14
 * @internal
 **/
export declare function toV14(registry: Registry, v13: MetadataV13, metaVersion: number): MetadataV14;
