import * as anchor from "@coral-xyz/anchor";
import { NetworkConfig, ProgramAddress } from "../interfaces";

export type ProgramName = "REGISTRY" | "DEX_ADAPTER";
export const networks = ["LOCALNET", "DEVNET"] as const;

export const NETWORK_CONFIG: NetworkConfig = {
  LOCALNET: "http://localhost:8899",
  DEVNET: "https://api.devnet.solana.com",
};

export const COMMITMENT: anchor.web3.Commitment = "confirmed";

export const PATH = {
  TO_CONFIG: "./scripts/common/config/index.ts",
  OWNER_KEYPAIR: "../../.test-wallets/solana/dev-keypair.json",
};

export const UTILS = {
  MS_PER_SECOND: 1_000,
  ENCODING: "utf8",
};
