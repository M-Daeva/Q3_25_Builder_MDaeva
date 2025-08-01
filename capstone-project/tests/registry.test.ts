import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Registry } from "../scripts/common/schema/types/registry";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { ChainHelpers, RegistryHelpers } from "../scripts/common/account";
import {
  getProvider,
  getRpc,
  li,
  publicKeyFromString,
  wait,
} from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";
import { ENV } from "../scripts/backend/envs";

describe("registry", async () => {
  // configure the client to use the local cluster
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("LOCALNET"),
    COMMITMENT
  );
  anchor.setProvider(provider);

  const chain = new ChainHelpers(provider);
  const registryProgram = anchor.workspace.Registry as anchor.Program<Registry>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const registry = new RegistryHelpers(provider, registryProgram);

  // generate new keypairs
  const mintXKeypair = Keypair.generate();
  const mintYKeypair = Keypair.generate();
  await chain.createMint(mintXKeypair, 6, TX_PARAMS);
  await chain.createMint(mintYKeypair, 6, TX_PARAMS);

  const admin = Keypair.generate();
  const user = Keypair.generate();

  // airdrop SOL for transaction fees
  await Promise.all(
    [admin, user].map((x) => chain.requestAirdrop(x.publicKey, 2))
  );

  // mint tokens for users
  let promiseList = [];
  for (const mintKeypair of [mintXKeypair, mintYKeypair]) {
    for (const actor of [admin, user]) {
      promiseList.push(
        chain.mintTokens(100, mintKeypair.publicKey, actor.publicKey, TX_PARAMS)
      );
    }
  }
  await Promise.all(promiseList);

  describe("instructions", () => {
    it("simulate account creation cost", async () => {
      await registry.tryInit({}, mintXKeypair.publicKey, TX_PARAMS);

      const lamportsPerCu = await chain.getCuPrice(
        ENV.QN_URL
        //  new PublicKey(registryProgram.idl.address)
      );
      await registry.simulateCreateAccount(5_000, lamportsPerCu, true);

      // 1 kB - 0.382668208 SOL
      // 5 kB - 0.41363114 SOL
    });
  });
});
