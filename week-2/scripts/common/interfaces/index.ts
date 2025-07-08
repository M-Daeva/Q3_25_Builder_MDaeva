import { AddressLookupTableAccount } from "@solana/web3.js";
import { networks, ProgramName } from "../config";

export type Network = (typeof networks)[number];

export type NetworkConfig = {
  [k in Network]: string;
};

export type ProgramAddress = {
  [k in ProgramName]: string;
};

/**
 * y = k * x + b
 */
export interface LinearParams {
  k: number;
  b: number;
}

export interface TxParams {
  lookupTables?: AddressLookupTableAccount[];
  priorityFee?: LinearParams;
  cpu?: LinearParams;
}
