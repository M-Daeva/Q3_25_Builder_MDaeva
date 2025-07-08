import * as anchor from "@coral-xyz/anchor";
import { describe, expect, it, beforeEach } from "bun:test";
import { Vault } from "../scripts/common/schema/types/vault";
import { Keypair } from "@solana/web3.js";
import { ChainHelpers, VaultHelpers } from "../scripts/common/account";
import { getProvider, getRpc, li } from "../scripts/common/utils";
import { getWallet, readKeypair, rootPath } from "../scripts/backend/utils";
import { COMMITMENT, PATH } from "../scripts/common/config";

describe("vault-anchor", async () => {
  // Configure the client to use the local cluster
  const ownerKeypair = await readKeypair(rootPath(PATH.OWNER_KEYPAIR));
  const provider = getProvider(
    getWallet(ownerKeypair),
    getRpc("LOCALNET"),
    COMMITMENT
  );
  anchor.setProvider(provider);

  const vaultProgram = anchor.workspace.Vault as anchor.Program<Vault>;
  const TX_PARAMS = {
    cpu: { k: 1, b: 150 },
  };

  let payer: Keypair;

  const initVault = async () => {
    await VaultHelpers.initialize(provider);
    return new VaultHelpers(provider, vaultProgram);
  };

  const vault = await initVault();

  beforeEach(async () => {
    const chain = new ChainHelpers(provider);

    // Generate new keypairs for each test
    payer = Keypair.generate();

    // Airdrop SOL to payer for transaction fees
    await chain.requestAirdrop(payer.publicKey, 2);
  });

  describe("accounts", () => {
    it("getVaultState", async () => {
      const { stateBump, vaultBump } = await vault.getVaultState();

      expect(stateBump).toEqual(255);
      expect(vaultBump).toEqual(254);
    });

    it("getVault", async () => {
      const balance = (await vault.getVault())?.lamports || 0;
      expect(balance).toEqual(890_880);
    });
  });

  describe("instructions", () => {
    it("deposit", async () => {
      const amount = 1_000_000_000;
      const balanceBefore = (await vault.getVault())?.lamports || 0;

      await vault.tryDeposit(amount, TX_PARAMS);

      const balanceAfter = (await vault.getVault())?.lamports || 0;
      expect(balanceAfter - balanceBefore).toEqual(amount);
    });

    it("withdraw", async () => {
      const amount = 500_000_000;
      const balanceBefore = (await vault.getVault())?.lamports || 0;

      await vault.tryWithdraw(amount, TX_PARAMS);

      const balanceAfter = (await vault.getVault())?.lamports || 0;
      expect(balanceBefore - balanceAfter).toEqual(amount);
    });

    it("close", async () => {
      await vault.tryClose(TX_PARAMS);

      const balanceAfter = (await vault.getVault())?.lamports || 0;
      expect(balanceAfter).toEqual(0);
    });
  });
});
