import type { Codec, Constructor, Registry } from '../../types';
export declare function typeToConstructor<T = Codec>(registry: Registry, type: string | Constructor<T>): Constructor<T>;
