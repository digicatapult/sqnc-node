import type { Observable } from 'rxjs';
import type { ApiInterfaceRx } from '@polkadot/api/types';
import type { EraRewardPoints } from '@polkadot/types/interfaces';
/**
 * @description Retrieve the staking overview, including elected and points earned
 */
export declare function currentPoints(instanceId: string, api: ApiInterfaceRx): () => Observable<EraRewardPoints>;
