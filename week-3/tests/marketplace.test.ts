import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Marketplace } from "../scripts/common/schema/types/marketplace";
import { Nft } from "../scripts/common/schema/types/nft";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
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
  const mintXKeypair = Keypair.generate();
  const mintYKeypair = Keypair.generate();
  await chain.createMint(mintXKeypair, 6, TX_PARAMS);
  await chain.createMint(mintYKeypair, 6, TX_PARAMS);

  const admin = Keypair.generate();
  const seller = Keypair.generate();
  const buyer = Keypair.generate();

  // airdrop SOL for transaction fees
  await Promise.all(
    [admin, seller, buyer].map((x) => chain.requestAirdrop(x.publicKey, 2))
  );

  // mint tokens for users
  let promiseList = [];
  for (const mintKeypair of [mintXKeypair, mintYKeypair]) {
    for (const user of [admin, seller, buyer]) {
      promiseList.push(
        chain.mintTokens(100, mintKeypair.publicKey, user.publicKey, TX_PARAMS)
      );
    }
  }
  await Promise.all(promiseList);

  // mint 2 nft tokens of each collection
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

  await marketplace.tryInit(
    500,
    [hellCats, galacticPunks].map((x) => x.address),
    [PublicKey.default, mintXKeypair.publicKey],
    "Flip Guru",
    TX_PARAMS
  );

  describe("instructions", () => {
    it("init", async () => {
      const config = await marketplace.getMarketplace();
      expect(config.feeBps).toEqual(500);
    });

    it("create and accept sell for token trade", async () => {
      const hellCatsTokenA = await nft.getToken(hellCats.address, 0);
      await marketplace.tryCreateTrade(
        seller,
        nftProgram.idl.address,
        hellCatsTokenA.mint,
        mintXKeypair.publicKey,
        true,
        hellCats.address,
        hellCatsTokenA.id,
        42_000,
        mintXKeypair.publicKey,
        TX_PARAMS
      );

      const trade = await marketplace.getTrade(
        seller.publicKey,
        hellCats.address,
        hellCatsTokenA.id
      );
      expect(trade.collection).toEqual(hellCats.address);
      expect(trade.tokenId).toEqual(hellCatsTokenA.id);

      const buyerTokensBefore = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        buyer.publicKey
      );

      await marketplace.tryAcceptTrade(
        buyer,
        nftProgram.idl.address,
        hellCatsTokenA.mint,
        mintXKeypair.publicKey,
        seller.publicKey,
        hellCats.address,
        hellCatsTokenA.id,
        TX_PARAMS
      );

      const buyerTokensAfter = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        buyer.publicKey
      );

      expect(
        Math.round(1_000 * (buyerTokensBefore - buyerTokensAfter))
      ).toEqual(42);
    });

    it("withdraw fee", async () => {
      let balances = await marketplace.getBalances();
      expect(balances.value[1].amount.toNumber()).toEqual(2_100);

      const adminMintXBefore = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        ownerKeypair.publicKey
      );

      await marketplace.tryWithdrawFee(TX_PARAMS);

      balances = await marketplace.getBalances();
      expect(balances.value[1].amount.toNumber()).toEqual(0);

      const adminMintXAfter = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        ownerKeypair.publicKey
      );

      expect(
        Math.round(1_000_000 * (adminMintXAfter - adminMintXBefore))
      ).toEqual(2_100);
    });

    it("create and accept buy with sol trade, withdraw fee", async () => {
      let buyerSolBefore = await chain.getBalance(buyer.publicKey);
      let sellerSolBefore = await chain.getBalance(seller.publicKey);

      const hellCatsTokenB = await nft.getToken(hellCats.address, 1);
      await marketplace.tryCreateTrade(
        buyer,
        nftProgram.idl.address,
        hellCatsTokenB.mint,
        PublicKey.default,
        false,
        hellCats.address,
        hellCatsTokenB.id,
        1.5 * LAMPORTS_PER_SOL,
        PublicKey.default,
        TX_PARAMS
      );

      let buyerSolAfter = await chain.getBalance(buyer.publicKey);
      let sellerSolAfter = await chain.getBalance(seller.publicKey);
      expect(buyerSolBefore - buyerSolAfter).toEqual(1.50373752);
      expect(sellerSolAfter - sellerSolBefore).toEqual(0);

      buyerSolBefore = buyerSolAfter;
      sellerSolBefore = sellerSolAfter;

      const trade = await marketplace.getTrade(
        buyer.publicKey,
        hellCats.address,
        hellCatsTokenB.id
      );
      expect(trade.collection).toEqual(hellCats.address);
      expect(trade.tokenId).toEqual(hellCatsTokenB.id);

      await marketplace.tryAcceptTrade(
        seller,
        nftProgram.idl.address,
        hellCatsTokenB.mint,
        PublicKey.default,
        buyer.publicKey,
        hellCats.address,
        hellCatsTokenB.id,
        TX_PARAMS
      );

      buyerSolAfter = await chain.getBalance(buyer.publicKey);
      sellerSolAfter = await chain.getBalance(seller.publicKey);

      expect(buyerSolAfter - buyerSolBefore).toEqual(0.0016982400000000175);
      expect(sellerSolAfter - sellerSolBefore).toEqual(1.4249999999999998);

      let adminSolBefore = await chain.getBalance(ownerKeypair.publicKey);
      await marketplace.tryWithdrawFee(TX_PARAMS);

      let adminSolAfter = await chain.getBalance(ownerKeypair.publicKey);
      expect(adminSolAfter - adminSolBefore).toEqual(0.07499504089355469);
    });

    it("create and remove buy for token trade", async () => {
      const buyerTokensBefore = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        buyer.publicKey
      );

      const galacticPunksTokenA = await nft.getToken(galacticPunks.address, 0);
      await marketplace.tryCreateTrade(
        buyer,
        nftProgram.idl.address,
        galacticPunksTokenA.mint,
        mintXKeypair.publicKey,
        false,
        galacticPunks.address,
        galacticPunksTokenA.id,
        42_000,
        mintXKeypair.publicKey,
        TX_PARAMS
      );

      const trade = await marketplace.getTrade(
        buyer.publicKey,
        galacticPunks.address,
        galacticPunksTokenA.id
      );
      expect(trade.collection).toEqual(galacticPunks.address);
      expect(trade.tokenId).toEqual(galacticPunksTokenA.id);

      await marketplace.tryRemoveTrade(
        buyer,
        nftProgram.idl.address,
        galacticPunksTokenA.mint,
        mintXKeypair.publicKey,
        buyer.publicKey,
        galacticPunks.address,
        galacticPunksTokenA.id,
        TX_PARAMS
      );

      const buyerTokensAfter = await chain.getTokenBalance(
        mintXKeypair.publicKey,
        buyer.publicKey
      );

      expect(
        Math.round(1_000 * (buyerTokensBefore - buyerTokensAfter))
      ).toEqual(0);
    });

    it("create and remove sell for sol trade", async () => {
      const sellerSolBefore = await chain.getBalance(seller.publicKey);

      const galacticPunksTokenA = await nft.getToken(galacticPunks.address, 0);
      await marketplace.tryCreateTrade(
        seller,
        nftProgram.idl.address,
        galacticPunksTokenA.mint,
        PublicKey.default,
        true,
        galacticPunks.address,
        galacticPunksTokenA.id,
        1.5 * LAMPORTS_PER_SOL,
        PublicKey.default,
        TX_PARAMS
      );

      const trade = await marketplace.getTrade(
        seller.publicKey,
        galacticPunks.address,
        galacticPunksTokenA.id
      );
      expect(trade.collection).toEqual(galacticPunks.address);
      expect(trade.tokenId).toEqual(galacticPunksTokenA.id);

      await marketplace.tryRemoveTrade(
        seller,
        nftProgram.idl.address,
        galacticPunksTokenA.mint,
        PublicKey.default,
        seller.publicKey,
        galacticPunks.address,
        galacticPunksTokenA.id,
        TX_PARAMS
      );

      const sellerSolAfter = await chain.getBalance(seller.publicKey);

      expect(sellerSolBefore - sellerSolAfter).toEqual(0.0020392799999999767);
    });
  });
});
