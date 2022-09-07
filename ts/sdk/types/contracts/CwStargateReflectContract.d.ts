/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/
import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
export interface InstantiateMsg {
    [k: string]: unknown;
}
export declare type QueryMsg = {
    query_stargate: {
        path: string;
        query_request: string;
        [k: string]: unknown;
    };
};
export interface QueryStargateResponse {
    value: string;
    [k: string]: unknown;
}
export interface CwStargateReflectReadOnlyInterface {
    contractAddress: string;
    queryStargate: ({ path, queryRequest }: {
        path: string;
        queryRequest: string;
    }) => Promise<QueryStargateResponse>;
}
export declare class CwStargateReflectQueryClient implements CwStargateReflectReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    queryStargate: ({ path, queryRequest }: {
        path: string;
        queryRequest: string;
    }) => Promise<QueryStargateResponse>;
}
//# sourceMappingURL=CwStargateReflectContract.d.ts.map