import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it, beforeEach } from "bun:test";
import { Escrow } from "../scripts/common/schema/types/escrow";
import { Keypair } from "@solana/web3.js";
import { ChainHelpers, EscrowHelpers } from "../scripts/common/account";
import { getProvider, getRpc, li } from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";

describe("escrow-anchor", async () => {
  // Configure the client to use the local cluster
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("LOCALNET"),
    COMMITMENT
  );
  anchor.setProvider(provider);

  const chain = new ChainHelpers(provider);
  const escrowProgram = anchor.workspace.Escrow as anchor.Program<Escrow>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  let payer: Keypair;

  const escrow = new EscrowHelpers(provider, escrowProgram);

  beforeEach(async () => {
    const chain = new ChainHelpers(provider);

    // Generate new keypairs for each test
    payer = Keypair.generate();

    // Airdrop SOL to payer for transaction fees
    await chain.requestAirdrop(payer.publicKey, 2);
  });

  describe("instructions", () => {
    it("make, refund", async () => {
      // prepare mint
      const makerMintKeypair = Keypair.generate();
      const takerMintKeypair = Keypair.generate();
      await chain.createMint(makerMintKeypair, 6, TX_PARAMS);
      await chain.createMint(takerMintKeypair, 6, TX_PARAMS);

      const maker = ownerKeypair.publicKey;
      const taker = Keypair.generate().publicKey;
      const makerMint = makerMintKeypair.publicKey;
      const takerMint = takerMintKeypair.publicKey;

      await chain.mintTokens(42, makerMint, maker, TX_PARAMS);

      // interact with escrow
      await escrow.tryMake(
        0,
        {
          trader: maker,
          amount: 500_000,
          mint: makerMint,
        },
        {
          trader: taker,
          amount: 250_000,
          mint: takerMint,
        },
        TX_PARAMS
      );

      const {
        maker: { amount: makerAmount },
        taker: { amount: takerAmount },
      } = await escrow.getEscrowState(maker, 0);

      expect(makerAmount.toNumber()).toEqual(500_000);
      expect(takerAmount.toNumber()).toEqual(250_000);

      await escrow.tryRefund(0, TX_PARAMS);

      const makerBalance = await chain.getTokenBalance(makerMint, maker);
      expect(makerBalance).toEqual(42);
    });

    it("make, take", async () => {
      // prepare mint
      const makerMintKeypair = Keypair.generate();
      const takerMintKeypair = Keypair.generate();
      await chain.createMint(makerMintKeypair, 6, TX_PARAMS);
      await chain.createMint(takerMintKeypair, 6, TX_PARAMS);

      const maker = ownerKeypair.publicKey;
      const takerKeypair = Keypair.generate();
      const taker = takerKeypair.publicKey;
      const makerMint = makerMintKeypair.publicKey;
      const takerMint = takerMintKeypair.publicKey;

      await chain.mintTokens(42, makerMint, maker, TX_PARAMS);
      await chain.mintTokens(42, takerMint, taker, TX_PARAMS);

      // interact with escrow
      await escrow.tryMake(
        0,
        {
          trader: maker,
          amount: 500_000,
          mint: makerMint,
        },
        {
          trader: taker,
          amount: 250_000,
          mint: takerMint,
        },
        TX_PARAMS
      );

      const {
        maker: { amount: makerAmount },
        taker: { amount: takerAmount },
      } = await escrow.getEscrowState(maker, 0);

      expect(makerAmount.toNumber()).toEqual(500_000);
      expect(takerAmount.toNumber()).toEqual(250_000);

      await chain.requestAirdrop(taker, 2);
      await escrow.tryTake(0, maker, takerKeypair, TX_PARAMS);

      const makerBalaneInTakerMint = await chain.getTokenBalance(
        takerMint,
        maker
      );
      const takerBalaneInMakerMint = await chain.getTokenBalance(
        makerMint,
        taker
      );

      expect(makerBalaneInTakerMint).toEqual(0.25);
      expect(takerBalaneInMakerMint).toEqual(0.5);
    });
  });
});
