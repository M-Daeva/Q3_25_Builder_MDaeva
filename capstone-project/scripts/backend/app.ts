import { ChainHelpers, RegistryHelpers } from "../common/account";
import { COMMITMENT, PATH, REVENUE_MINT } from "../common/config";
import { getProgram, getProvider, getRpc } from "../common/utils";
import { getWallet, parseNetwork, readKeypair, rootPath } from "./utils";
import { PublicKey } from "@solana/web3.js";

import { Registry } from "../common/schema/types/registry";
import RegistryIdl from "../common/schema/idl/registry.json";

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

  const registry = new RegistryHelpers(
    provider,
    getProgram<Registry>(provider, RegistryIdl as any)
  );

  await registry.tryInit(
    { accountRegistrationFee: { amount: 100_000, asset: REVENUE_MINT.DEVNET } },
    REVENUE_MINT.DEVNET,
    TX_PARAMS
  );

  await registry.queryConfig(true);
}

main();
