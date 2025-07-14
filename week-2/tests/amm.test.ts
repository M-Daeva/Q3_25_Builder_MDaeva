import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
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

  const amm = new AmmHelpers(provider, ammProgram);

  // generate new keypairs for each test
  const poolCreator = Keypair.generate();
  const liquidityProviderA = Keypair.generate();
  const liquidityProviderB = Keypair.generate();
  const trader = Keypair.generate();

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
        chain.mintTokens(100, mintKeypair.publicKey, user.publicKey, TX_PARAMS)
      );
    }
  }
  await Promise.all(promiseList);

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

    it("swap", async () => {
      const id: number = 0;

      const trader_mint_x_balance_before = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        trader.publicKey
      );
      const trader_mint_y_balance_before = await chain.getTokenBalance(
        mintYKeypair.publicKey,
        trader.publicKey
      );

      // swap
      await amm.trySwap(id, 1_000, mintXKeypair.publicKey, trader, TX_PARAMS);

      const trader_mint_x_balance_after = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        trader.publicKey
      );
      const trader_mint_y_balance_after = await chain.getTokenBalance(
        mintYKeypair.publicKey,
        trader.publicKey
      );

      expect(
        Math.round(
          1e6 * (trader_mint_x_balance_before - trader_mint_x_balance_after)
        )
      ).toEqual(1_000);
      // mint_y_amount ≈ (1 - 0.01) * 1_000 * (8_000_000 / 2_000_000) = 3_960
      expect(
        Math.round(
          1e6 * (trader_mint_y_balance_after - trader_mint_y_balance_before)
        )
      ).toEqual(3_960);

      // swap
      await amm.trySwap(id, 3_960, mintYKeypair.publicKey, trader, TX_PARAMS);
    });

    it("withdraw liquidity", async () => {
      const id: number = 0;

      const liquidity_provider_mint_x_balance_before =
        await chain.getTokenBalance(
          mintXKeypair.publicKey,
          liquidityProviderA.publicKey
        );
      const liquidity_provider_mint_y_balance_before =
        await chain.getTokenBalance(
          mintYKeypair.publicKey,
          liquidityProviderA.publicKey
        );

      // withdraw liquidity
      await amm.tryWithdrawLiquidity(
        id,
        4_000_000,
        liquidityProviderA,
        TX_PARAMS
      );

      const liquidity_provider_mint_x_balance_after =
        await chain.getTokenBalance(
          mintXKeypair.publicKey,
          liquidityProviderA.publicKey
        );
      const liquidity_provider_mint_y_balance_after =
        await chain.getTokenBalance(
          mintYKeypair.publicKey,
          liquidityProviderA.publicKey
        );

      expect(
        Math.round(
          1e6 *
            (liquidity_provider_mint_x_balance_after -
              liquidity_provider_mint_x_balance_before)
        )
      ).toEqual(2_000_018);
      expect(
        Math.round(
          1e6 *
            (liquidity_provider_mint_y_balance_after -
              liquidity_provider_mint_y_balance_before)
        )
      ).toEqual(8_000_000);
    });
  });
});
