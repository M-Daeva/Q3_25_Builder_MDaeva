import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Staking } from "../scripts/common/schema/types/staking";
import { Keypair } from "@solana/web3.js";
import { ChainHelpers, StakingHelpers } from "../scripts/common/account";
import { getProvider, getRpc, li, wait } from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";

describe("staking-anchor", async () => {
  // configure the client to use the local cluster
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("LOCALNET"),
    COMMITMENT
  );
  anchor.setProvider(provider);

  const chain = new ChainHelpers(provider);
  const stakingProgram = anchor.workspace.Staking as anchor.Program<Staking>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const mintNftKeypair = Keypair.generate();
  await chain.createMint(mintNftKeypair, 6, TX_PARAMS);

  const staking = new StakingHelpers(provider, stakingProgram);

  // generate new keypairs for each test
  const admin = Keypair.generate();
  const stakerA = Keypair.generate();
  const stakerB = Keypair.generate();

  // airdrop SOL for transaction fees
  await Promise.all(
    [admin, stakerA, stakerB].map((x) => chain.requestAirdrop(x.publicKey, 2))
  );

  // mint tokens for users
  await Promise.all(
    [stakerA, stakerB].map((user) =>
      chain.mintTokens(100, mintNftKeypair.publicKey, user.publicKey, TX_PARAMS)
    )
  );

  describe("instructions", () => {
    it("init", async () => {
      await staking.tryInit(10, 5, mintNftKeypair.publicKey, TX_PARAMS);

      const config = await staking.getConfig();
      expect(config.rewardsRate).toEqual(10);
      expect(config.maxStake.toNumber()).toEqual(5);
    });

    it("stake, claim", async () => {
      const config = await staking.getConfig();
      await staking.tryStake(
        [1, 2, 3],
        mintNftKeypair.publicKey,
        stakerA,
        TX_PARAMS
      );

      // const vault = await staking.getUserVault(stakerA.publicKey);
      await wait(1_000);
      const stakerABalanceBefore = await chain.getTokenBalance(
        config.rewardsMint,
        stakerA.publicKey
      );

      await staking.tryClaim(stakerA, TX_PARAMS);
      const stakerABalanceAfter = await chain.getTokenBalance(
        config.rewardsMint,
        stakerA.publicKey
      );

      expect(stakerABalanceAfter - stakerABalanceBefore).toEqual(0.00003);
    });
  });
});
