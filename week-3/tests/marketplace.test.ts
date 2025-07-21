import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Marketplace } from "../scripts/common/schema/types/marketplace";
import { Nft } from "../scripts/common/schema/types/nft";
import { Keypair, PublicKey } from "@solana/web3.js";
import {
  ChainHelpers,
  NftHelpers,
  MarketplaceHelpers,
} from "../scripts/common/account";
import {
  getProvider,
  getRpc,
  li,
  publicKeyFromString,
  wait,
} from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";

describe("marketplace", async () => {
  // configure the client to use the local cluster
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("LOCALNET"),
    COMMITMENT
  );
  anchor.setProvider(provider);

  const chain = new ChainHelpers(provider);
  const nftProgram = anchor.workspace.Nft as anchor.Program<Nft>;
  const marketplaceProgram = anchor.workspace
    .Marketplace as anchor.Program<Marketplace>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const nft = new NftHelpers(provider, nftProgram);
  const marketplace = new MarketplaceHelpers(provider, marketplaceProgram);

  // generate new keypairs
  const admin = Keypair.generate();
  const seller = Keypair.generate();
  const buyer = Keypair.generate();

  // airdrop SOL for transaction fees
  await Promise.all(
    [admin, seller, buyer].map((x) => chain.requestAirdrop(x.publicKey, 2))
  );

  // mint 2 tokens of each collection
  for (const [id, metadata] of [
    [0, "HellCats"],
    [1, "GalacticPunks"],
    [2, "LunaBulls"],
  ] as [number, string][]) {
    await nft.tryCreateCollection(id, metadata, TX_PARAMS);
    await nft.tryMintToken(id, "", seller.publicKey, TX_PARAMS);
    await nft.tryMintToken(id, "", seller.publicKey, TX_PARAMS);
  }

  const [hellCats, galacticPunks, lunaBulls] = await Promise.all(
    [0, 1, 2].map((x) => nft.getCollection(ownerKeypair.publicKey, x))
  );

  // const token = await nft.getToken(
  //   collection.address,
  //   collection.nextTokenId - 1
  // );

  await marketplace.tryInit(
    500,
    [hellCats, galacticPunks].map((x) => x.address),
    ["sol"],
    "Flip Guru",
    TX_PARAMS
  );

  describe("instructions", () => {
    it("init", async () => {
      const config = await marketplace.getMarketplace();
      expect(config.feeBps).toEqual(500);
    });

    // it("stake, claim", async () => {
    //   const config = await staking.getConfig();

    //   let stakerAAta = await chain.getTokenBalance(
    //     token.mint,
    //     stakerA.publicKey
    //   );
    //   expect(stakerAAta).toEqual(1);

    //   let [configPda] = PublicKey.findProgramAddressSync(
    //     [Buffer.from("config")],
    //     publicKeyFromString(stakingProgram.idl.address)
    //   );
    //   let appAta = await chain.getTokenBalance(token.mint, configPda);
    //   expect(appAta).toEqual(0);

    //   await staking.tryStake(
    //     0,
    //     token.mint,
    //     nftProgram.idl.address,
    //     stakerA,
    //     TX_PARAMS
    //   );

    //   stakerAAta = await chain.getTokenBalance(token.mint, stakerA.publicKey);
    //   expect(stakerAAta).toEqual(0);

    //   [configPda] = PublicKey.findProgramAddressSync(
    //     [Buffer.from("config")],
    //     publicKeyFromString(stakingProgram.idl.address)
    //   );
    //   appAta = await chain.getTokenBalance(token.mint, configPda);
    //   expect(appAta).toEqual(1);

    //   await wait(1_000);
    //   const stakerABalanceBefore = await chain.getTokenBalance(
    //     config.rewardsMint,
    //     stakerA.publicKey
    //   );

    //   await staking.tryClaim(stakerA, TX_PARAMS);
    //   const stakerABalanceAfter = await chain.getTokenBalance(
    //     config.rewardsMint,
    //     stakerA.publicKey
    //   );

    //   expect(stakerABalanceAfter - stakerABalanceBefore).toBeGreaterThan(0);
    // });

    // it("unstake", async () => {
    //   await staking.tryUnstake(0, token.mint, stakerA, TX_PARAMS);

    //   const stakerAAta = await chain.getTokenBalance(
    //     token.mint,
    //     stakerA.publicKey
    //   );
    //   expect(stakerAAta).toEqual(1);

    //   const [configPda] = PublicKey.findProgramAddressSync(
    //     [Buffer.from("config")],
    //     publicKeyFromString(stakingProgram.idl.address)
    //   );
    //   const appAta = await chain.getTokenBalance(token.mint, configPda);
    //   expect(appAta).toEqual(0);
    // });
  });
});
