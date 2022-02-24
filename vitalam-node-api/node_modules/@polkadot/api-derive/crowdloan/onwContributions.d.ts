import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { ParaId } from '@polkadot/types/interfaces';
import type { DeriveOwnContributions } from '../types';
export declare function ownContributions(instanceId: string, api: ApiInterfaceRx): (paraId: string | number | ParaId, keys: string[]) => Observable<DeriveOwnContributions>;
