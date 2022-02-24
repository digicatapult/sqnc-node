import type { StorageEntryMetadataLatest } from '../../../interfaces/metadata';
import type { StorageEntry } from '../../../primitive/types';
import type { Registry } from '../../../types';
export interface CreateItemOptions {
    key?: string;
    skipHashing?: boolean;
}
export interface CreateItemFn {
    meta: StorageEntryMetadataLatest;
    method: string;
    prefix: string;
    section: string;
}
/** @internal */
export declare function createFunction(registry: Registry, itemFn: CreateItemFn, options: CreateItemOptions): StorageEntry;
