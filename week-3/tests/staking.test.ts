import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Staking } from "../scripts/common/schema/types/staking";
import { Nft } from "../scripts/common/schema/types/nft";
import { Keypair } from "@solana/web3.js";
import {
  ChainHelpers,
  StakingHelpers,
  NftHelpers,
} from "../scripts/common/account";
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
  const nftProgram = anchor.workspace.Nft as anchor.Program<Nft>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const collectionKeypair = Keypair.generate();
  const mintNftKeypair = Keypair.generate();
  const mintNftWrongKeypair = Keypair.generate();
  await chain.createMint(mintNftKeypair, 6, TX_PARAMS);
  await chain.createMint(mintNftWrongKeypair, 6, TX_PARAMS);

  const staking = new StakingHelpers(provider, stakingProgram);
  const nft = new NftHelpers(provider, nftProgram);

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
    it("mint nft", async () => {
      const COLLECTION_ID = 0;
      const COLLECTION_METADATA = "HellCats";

      await nft.tryCreateCollection(
        COLLECTION_ID,
        COLLECTION_METADATA,
        TX_PARAMS
      );
      await nft.tryMintToken(COLLECTION_ID, "", stakerA.publicKey, TX_PARAMS);

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
        collectionKeypair.publicKey,
        stakerA,
        TX_PARAMS
      );

      // // const vault = await staking.getUserVault(stakerA.publicKey);
      // await wait(1_000);
      // const stakerABalanceBefore = await chain.getTokenBalance(
      //   config.rewardsMint,
      //   stakerA.publicKey
      // );

      // await staking.tryClaim(stakerA, TX_PARAMS);
      // const stakerABalanceAfter = await chain.getTokenBalance(
      //   config.rewardsMint,
      //   stakerA.publicKey
      // );

      // // TODO: sometimes it gives 0.00006
      // expect(stakerABalanceAfter - stakerABalanceBefore).toEqual(0.00003);
    });

    // it("try to stake wrong nft", async () => {
    //   try {
    //     await staking.tryStake(
    //       [1, 2, 3],
    //       mintNftWrongKeypair.publicKey,
    //       collectionKeypair.publicKey,
    //       stakerA,
    //       TX_PARAMS
    //     );

    //     expect(5).toEqual(7);
    //   } catch (error) {}
    // });
  });
});
