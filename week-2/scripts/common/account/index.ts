import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { TxParams } from "../interfaces";
import {
  getHandleTx,
  getProgram,
  l,
  li,
  logAndReturn,
  publicKeyFromString,
} from "../../common/utils";

import { Vault } from "../schema/types/vault";
import VaultIdl from "../schema/idl/vault.json";

export class VaultHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Vault>;

  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams,
    isDisplayed: boolean
  ) => Promise<anchor.web3.TransactionSignature>;

  // use PublicKey for addresses
  private getPda(...args: (PublicKey | string)[]): PublicKey {
    const seeds: Array<Buffer | Uint8Array> = args.map((x) =>
      typeof x === "string" ? Buffer.from(x) : x.toBuffer()
    );

    const [pda] = PublicKey.findProgramAddressSync(
      seeds,
      this.program.programId
    );

    return pda;
  }

  constructor(provider: anchor.AnchorProvider, program: anchor.Program<Vault>) {
    this.provider = provider;
    this.program = program;
    this.handleTx = getHandleTx(provider);
  }

  static async initialize(provider: anchor.AnchorProvider): Promise<void> {
    await getProgram<Vault>(provider, VaultIdl as any)
      .methods.initialize()
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();
  }

  async tryDeposit(
    amount: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .deposit(new anchor.BN(amount))
      .accounts({ user: this.provider.wallet.publicKey })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryWithdraw(
    amount: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .withdraw(new anchor.BN(amount))
      .accounts({ user: this.provider.wallet.publicKey })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryClose(
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .close()
      .accounts({ user: this.provider.wallet.publicKey })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async getVaultState(isDisplayed: boolean = false) {
    const vaultState = await this.program.account.vaultState.fetch(
      this.getPda("state", this.provider.wallet.publicKey)
    );

    return logAndReturn(vaultState, isDisplayed);
  }

  async getVault(isDisplayed: boolean = false) {
    const vaultState = await this.provider.connection.getAccountInfo(
      this.getPda("vault", this.getPda("state", this.provider.wallet.publicKey))
    );

    return logAndReturn(vaultState, isDisplayed);
  }
}

export class ChainHelpers {
  private provider: anchor.AnchorProvider;
  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams
  ) => Promise<anchor.web3.TransactionSignature>;

  constructor(provider: anchor.AnchorProvider) {
    this.provider = provider;
    this.handleTx = getHandleTx(provider);
  }

  async requestAirdrop(
    publicKey: anchor.web3.PublicKey | string,
    amount: number
  ): Promise<anchor.web3.TransactionSignature> {
    const signature = await this.provider.connection.requestAirdrop(
      publicKeyFromString(publicKey),
      amount * anchor.web3.LAMPORTS_PER_SOL
    );

    const { blockhash, lastValidBlockHeight } =
      await this.provider.connection.getLatestBlockhash();

    await this.provider.connection.confirmTransaction({
      blockhash,
      lastValidBlockHeight,
      signature,
    });

    return signature;
  }

  async transferSOL(
    to: anchor.web3.PublicKey | string,
    amount: number,
    params: TxParams = {}
  ): Promise<anchor.web3.TransactionSignature> {
    const instructions: anchor.web3.TransactionInstruction[] = [
      anchor.web3.SystemProgram.transfer({
        fromPubkey: this.provider.wallet.publicKey,
        toPubkey: publicKeyFromString(to),
        lamports: amount * anchor.web3.LAMPORTS_PER_SOL,
      }),
    ];

    return await this.handleTx(instructions, params);
  }

  async getBalance(
    publicKey: anchor.web3.PublicKey | string,
    isDisplayed: boolean = false
  ): Promise<number> {
    const balance = await this.provider.connection.getBalance(
      publicKeyFromString(publicKey)
    );

    return logAndReturn(balance / anchor.web3.LAMPORTS_PER_SOL, isDisplayed);
  }
}
