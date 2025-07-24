import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Nft } from "../scripts/common/schema/types/nft";
import { Keypair, PublicKey } from "@solana/web3.js";
import { ChainHelpers, NftHelpers } from "../scripts/common/account";
import {
  getProvider,
  getRpc,
  publicKeyFromString,
  wait,
} from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";

describe("staking", async () => {
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
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

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
  });
});
