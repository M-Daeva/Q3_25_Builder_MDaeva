import * as anchor from "@coral-xyz/anchor";
import { COMMITMENT, PATH, REVENUE_MINT } from "../common/config";
import { getProgram, getProvider, getRpc, li } from "../common/utils";
import { getWallet, parseNetwork, readKeypair, rootPath } from "./utils";
import { PublicKey } from "@solana/web3.js";
import {
  ChainHelpers,
  RegistryHelpers,
  DexAdapterHelpers,
} from "../common/account";

import { Registry } from "../common/schema/types/registry";
import RegistryIdl from "../common/schema/idl/registry.json";

import { DexAdapter } from "../common/schema/types/dex_adapter";
import DexAdapterIdl from "../common/schema/idl/dex_adapter.json";

async function main() {
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("DEVNET"),
    COMMITMENT
  );

  const chain = new ChainHelpers(provider);
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const registryProgram = getProgram<Registry>(provider, RegistryIdl as any);
  const dexAdapterProgram = getProgram<DexAdapter>(
    provider,
    DexAdapterIdl as any
  );

  const registryAddress = registryProgram.programId;
  const dexAdapterAddress = dexAdapterProgram.programId;
  const dexAddress = new PublicKey(
    "devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH"
  );

  li({
    registry: registryAddress.toString(),
    dexAdapter: dexAdapterAddress.toString(),
    dex: dexAddress.toString(),
  });

  const registry = new RegistryHelpers(provider, registryProgram);
  const dexAdapter = new DexAdapterHelpers(
    provider,
    dexAdapterProgram,
    registryAddress,
    dexAddress
  );

  // await dexAdapter.tryInit(
  //   {
  //     registry: registryAddress,
  //     dex: dexAddress,
  //   },
  //   TX_PARAMS
  // );

  await dexAdapter.queryConfig(true);

  // await registry.tryInit(
  //   { accountRegistrationFee: { amount: 100_000, asset: REVENUE_MINT.DEVNET } },
  //   REVENUE_MINT.DEVNET,
  //   TX_PARAMS
  // );

  // await registry.queryConfig(true);
}

main();
