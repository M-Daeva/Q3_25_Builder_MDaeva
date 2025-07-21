import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Staking } from "../scripts/common/schema/types/staking";
import { Nft } from "../scripts/common/schema/types/nft";
import { Keypair, PublicKey } from "@solana/web3.js";
import {
  ChainHelpers,
  StakingHelpers,
  NftHelpers,
} from "../scripts/common/account";
import {
  getProvider,
  getRpc,
  publicKeyFromString,
  wait,
} from "../scripts/common/utils";
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
  const nftProgram = anchor.workspace.Nft as anchor.Program<Nft>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const staking = new StakingHelpers(provider, stakingProgram);
  const nft = new NftHelpers(provider, nftProgram);

  // generate new keypairs
  const admin = Keypair.generate();
  const stakerA = Keypair.generate();
  const stakerB = Keypair.generate();

  // airdrop SOL for transaction fees
  await Promise.all(
    [admin, stakerA, stakerB].map((x) => chain.requestAirdrop(x.publicKey, 2))
  );

  const COLLECTION_ID = 0;
  const COLLECTION_METADATA = "HellCats";

  await nft.tryCreateCollection(COLLECTION_ID, COLLECTION_METADATA, TX_PARAMS);
  await nft.tryMintToken(COLLECTION_ID, "", stakerA.publicKey, TX_PARAMS);

  const collection = await nft.getCollection(
    ownerKeypair.publicKey,
    COLLECTION_ID
  );
  const token = await nft.getToken(
    collection.address,
    collection.nextTokenId - 1
  );

  describe("instructions", () => {
    it("mint nft", async () => {
      const COLLECTION_ID = 0;
      const COLLECTION_METADATA = "HellCats";

      const collection = await nft.getCollection(
        ownerKeypair.publicKey,
        COLLECTION_ID
      );
      const token = await nft.getToken(
        collection.address,
        collection.nextTokenId - 1
      );

      expect(collection.metadata).toEqual(COLLECTION_METADATA);
      expect(token.id).toEqual(0);
    });

    it("init", async () => {
      await staking.tryInit(10, 5, collection.address, token.mint, TX_PARAMS);

      const config = await staking.getConfig();
      expect(config.rewardsRate).toEqual(10);
      expect(config.maxStake.toNumber()).toEqual(5);
    });

    it("stake, claim", async () => {
      const config = await staking.getConfig();

      let stakerAAta = await chain.getTokenBalance(
        token.mint,
        stakerA.publicKey
      );
      expect(stakerAAta).toEqual(1);

      let [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("config")],
        publicKeyFromString(stakingProgram.idl.address)
      );
      let appAta = await chain.getTokenBalance(token.mint, configPda);
      expect(appAta).toEqual(0);

      await staking.tryStake(
        0,
        token.mint,
        nftProgram.idl.address,
        stakerA,
        TX_PARAMS
      );

      stakerAAta = await chain.getTokenBalance(token.mint, stakerA.publicKey);
      expect(stakerAAta).toEqual(0);

      [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("config")],
        publicKeyFromString(stakingProgram.idl.address)
      );
      appAta = await chain.getTokenBalance(token.mint, configPda);
      expect(appAta).toEqual(1);

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

      expect(stakerABalanceAfter - stakerABalanceBefore).toBeGreaterThan(0);
    });

    it("unstake", async () => {
      await staking.tryUnstake(0, token.mint, stakerA, TX_PARAMS);

      const stakerAAta = await chain.getTokenBalance(
        token.mint,
        stakerA.publicKey
      );
      expect(stakerAAta).toEqual(1);

      const [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("config")],
        publicKeyFromString(stakingProgram.idl.address)
      );
      const appAta = await chain.getTokenBalance(token.mint, configPda);
      expect(appAta).toEqual(0);
    });
  });
});
