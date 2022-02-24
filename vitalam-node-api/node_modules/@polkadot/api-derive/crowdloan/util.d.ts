import type { Vec } from '@polkadot/types';
import type { EventRecord, ParaId } from '@polkadot/types/interfaces';
interface Changes {
    added: string[];
    blockHash: string;
    removed: string[];
}
export declare function extractContributed(paraId: string | number | ParaId, events: Vec<EventRecord>): Changes;
export {};
