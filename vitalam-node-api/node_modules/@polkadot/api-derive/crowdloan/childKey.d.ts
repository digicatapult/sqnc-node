import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { ParaId } from '@polkadot/types/interfaces';
export declare function childKey(instanceId: string, api: ApiInterfaceRx): (paraId: string | number | ParaId) => Observable<string | null>;
