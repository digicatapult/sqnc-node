import type { Vec } from '../codec/Vec';
import type { PortableType } from '../interfaces/metadata';
import type { SiLookupTypeId, SiType } from '../interfaces/scaleInfo';
import type { Registry, TypeDef } from '../types';
import { Struct } from '../codec/Struct';
export declare class GenericPortableRegistry extends Struct {
    #private;
    constructor(registry: Registry, value?: Uint8Array);
    /**
     * @description The types of the registry
     */
    get types(): Vec<PortableType>;
    /**
     * @description Finds a specific type in the registry
     */
    getSiType(lookupId: SiLookupTypeId | string | number): SiType;
    /**
     * @description Lookup the type definition for the index
     */
    getTypeDef(lookupId: SiLookupTypeId | string | number): TypeDef;
}
