import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it, beforeEach } from "bun:test";
import { Amm } from "../scripts/common/schema/types/amm";
import { Keypair } from "@solana/web3.js";
import { ChainHelpers, AmmHelpers } from "../scripts/common/account";
import { getProvider, getRpc, li } from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";

describe("amm-anchor", async () => {
  // configure the client to use the local cluster
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("LOCALNET"),
    COMMITMENT
  );
  anchor.setProvider(provider);

  const chain = new ChainHelpers(provider);
  const ammProgram = anchor.workspace.Amm as anchor.Program<Amm>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const mintXKeypair = Keypair.generate();
  const mintYKeypair = Keypair.generate();
  await chain.createMint(mintXKeypair, 6, TX_PARAMS);
  await chain.createMint(mintYKeypair, 6, TX_PARAMS);

  let poolCreator: Keypair;
  let liquidityProviderA: Keypair;
  let liquidityProviderB: Keypair;
  let trader: Keypair;

  const amm = new AmmHelpers(provider, ammProgram);

  beforeEach(async () => {
    const chain = new ChainHelpers(provider);

    // generate new keypairs for each test
    poolCreator = Keypair.generate();
    liquidityProviderA = Keypair.generate();
    liquidityProviderB = Keypair.generate();
    trader = Keypair.generate();

    // airdrop SOL for transaction fees
    await Promise.all(
      [poolCreator, liquidityProviderA, liquidityProviderB, trader].map((x) =>
        chain.requestAirdrop(x.publicKey, 2)
      )
    );

    // mint tokens for users
    let promiseList = [];
    for (const mintKeypair of [mintXKeypair, mintYKeypair]) {
      for (const user of [liquidityProviderA, liquidityProviderB, trader]) {
        promiseList.push(
          chain.mintTokens(
            100,
            mintKeypair.publicKey,
            user.publicKey,
            TX_PARAMS
          )
        );
      }
    }
    await Promise.all(promiseList);
  });

  describe("instructions", () => {
    it("create pool, provide liquidity", async () => {
      const id: number = 0;

      // create pool
      await amm.tryCreatePool(
        id,
        mintXKeypair.publicKey,
        mintYKeypair.publicKey,
        1 * 100,
        TX_PARAMS
      );

      const { mintLp } = await amm.getPoolConfig(id);
      let poolBalance = await amm.getPoolBalance(id);

      expect(mintLp.toString()).toEqual(
        "CbFuANF3F33Nda8QhWDXgzB4YFaBJECznD5XdiRHKHVQ"
      );
      expect(poolBalance.mintLpAmount.toNumber()).toEqual(0);
      expect(poolBalance.mintXAmount.toNumber()).toEqual(0);
      expect(poolBalance.mintYAmount.toNumber()).toEqual(0);

      // provide liquidity
      await amm.tryProvideLiquidity(
        id,
        2_000_000,
        8_000_000,
        liquidityProviderA,
        TX_PARAMS
      );

      poolBalance = await amm.getPoolBalance(id);
      const providerALpBalance = await chain.getTokenBalance(
        mintLp,
        liquidityProviderA.publicKey
      );

      expect(poolBalance.mintLpAmount.toNumber()).toEqual(4_000_000);
      expect(poolBalance.mintXAmount.toNumber()).toEqual(2_000_000);
      expect(poolBalance.mintYAmount.toNumber()).toEqual(8_000_000);
      expect(providerALpBalance).toEqual(4);
    });
  });
});
