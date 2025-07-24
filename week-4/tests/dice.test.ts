import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it } from "bun:test";
import { Dice } from "../scripts/common/schema/types/dice";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { ChainHelpers, DiceHelpers } from "../scripts/common/account";
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
  });
});
