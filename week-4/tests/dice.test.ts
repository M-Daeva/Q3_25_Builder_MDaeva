import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Dice } from "../scripts/common/schema/types/dice";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { ChainHelpers, DiceHelpers } from "../scripts/common/account";
import { getProvider, getRpc, li } from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";

describe("dice", async () => {
  // configure the client to use the local cluster
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("LOCALNET"),
    COMMITMENT
  );
  anchor.setProvider(provider);

  const chain = new ChainHelpers(provider);
  const diceProgram = anchor.workspace.Dice as anchor.Program<Dice>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  const dice = new DiceHelpers(provider, diceProgram);

  // generate new keypairs
  const admin = Keypair.generate();
  const user = Keypair.generate();

  // airdrop SOL for transaction fees
  await Promise.all(
    [admin, user].map((x) => chain.requestAirdrop(x.publicKey, 2))
  );

  describe("instructions", () => {
    it("init", async () => {
      await dice.tryInit(1 * LAMPORTS_PER_SOL, TX_PARAMS);

      const vaultPda = dice.getVaultPda();
      const vaultSol = await chain.getBalance(vaultPda);

      expect(vaultSol).toEqual(1);
    });

    it("place and resolve bet - player won", async () => {
      const ID = 0;
      const BET = 0.5 * LAMPORTS_PER_SOL;
      const ROLL = 95;

      await dice.tryPlaceBet(user, ID, ROLL, BET, TX_PARAMS);

      const bet = await dice.getBet(ID);
      expect(bet.amount.toNumber()).toEqual(BET);

      const signature = await dice.generateSignatureForBet(ownerKeypair, ID);
      expect(signature.length).toEqual(64);

      const playerSolBefore = await chain.getBalance(user.publicKey);
      await dice.tryResolveBet(ID, signature, TX_PARAMS);

      const playerSolAfter = await chain.getBalance(user.publicKey);
      expect(playerSolAfter - playerSolBefore).toEqual(0.5280016640000003);
    });

    it("place and resolve bet - player lost", async () => {
      const ID = 1;
      const BET = 0.5 * LAMPORTS_PER_SOL;
      const ROLL = 15;

      await dice.tryPlaceBet(user, ID, ROLL, BET, TX_PARAMS);

      const playerSolBefore = await chain.getBalance(user.publicKey);
      const signature = await dice.generateSignatureForBet(ownerKeypair, ID);
      await dice.tryResolveBet(ID, signature, TX_PARAMS);

      const playerSolAfter = await chain.getBalance(user.publicKey);
      expect(Math.round(10 * (playerSolAfter - playerSolBefore)) / 10).toEqual(
        0
      );
    });

    it("place and refund bet", async () => {
      const ID = 2;
      const BET = 0.5 * LAMPORTS_PER_SOL;
      const ROLL = 15;

      const playerSolBefore = await chain.getBalance(user.publicKey);
      await dice.tryPlaceBet(user, ID, ROLL, BET, TX_PARAMS);
      await dice.tryRefundBet(user, ID, TX_PARAMS);

      const playerSolAfter = await chain.getBalance(user.publicKey);
      expect(playerSolAfter - playerSolBefore).toEqual(0);
    });
  });
});
