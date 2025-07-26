import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { ed25519 } from "@noble/curves/ed25519";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TxParams } from "../interfaces";
import {
  getHandleTx,
  getOrCreateAtaInstructions,
  li,
  logAndReturn,
  publicKeyFromString,
} from "../../common/utils";

import { Dice } from "../schema/types/dice";

export class DiceHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Dice>;

  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams,
    isDisplayed: boolean
  ) => Promise<anchor.web3.TransactionSignature>;

  constructor(provider: anchor.AnchorProvider, program: anchor.Program<Dice>) {
    this.provider = provider;
    this.program = program;
    this.handleTx = getHandleTx(provider);
  }

  async tryInit(
    amount: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .init(new anchor.BN(amount))
      .accounts({
        house: this.provider.wallet.publicKey,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryPlaceBet(
    userKeypair: Keypair,
    id: number,
    roll: number,
    amount: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .placeBet(new anchor.BN(id), roll, new anchor.BN(amount))
      .accounts({
        house: this.provider.wallet.publicKey,
        player: userKeypair.publicKey,
      })
      .instruction();

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx([ix], modifiedParams, isDisplayed);
  }

  async tryResolveBet(
    id: number,
    signature: number[],
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const vaultPDA = this.getVaultPda();
    const betPDA = this.getBetPDA(vaultPDA, id);
    const betAccount = await this.program.account.bet.fetch(betPDA);

    const message = this.createBetMessage({
      player: betAccount.player,
      amount: betAccount.amount,
      slot: betAccount.slot,
      id: betAccount.id,
      roll: betAccount.roll,
      bump: betAccount.bump,
    });

    const ed25519Ix = anchor.web3.Ed25519Program.createInstructionWithPublicKey(
      {
        publicKey: this.provider.wallet.publicKey.toBytes(),
        message,
        signature: new Uint8Array(signature),
      }
    );

    const ix = await this.program.methods
      .resolveBet(new anchor.BN(id), Array.from(new Uint8Array(signature)))
      .accounts({
        house: this.provider.wallet.publicKey,
      })
      .instruction();

    return await this.handleTx([ed25519Ix, ix], params, isDisplayed);
  }

  async tryRefundBet(
    userKeypair: Keypair,
    id: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .refundBet(new anchor.BN(id))
      .accounts({
        house: this.provider.wallet.publicKey,
        player: userKeypair.publicKey,
      })
      .instruction();

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx([ix], modifiedParams, isDisplayed);
  }

  getVaultPda(isDisplayed: boolean = false) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), this.provider.wallet.publicKey.toBuffer()],
      this.program.programId
    );

    return logAndReturn(pda, isDisplayed);
  }

  getBetPDA(
    vaultPubkey: PublicKey,
    id: number | anchor.BN,
    isDisplayed: boolean = false
  ) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("bet"), vaultPubkey.toBuffer(), this.idToBuffer(id)],
      this.program.programId
    );

    return logAndReturn(pda, isDisplayed);
  }

  async getBet(id: number, isDisplayed: boolean = false) {
    const pda = this.getBetPDA(this.getVaultPda(), id);
    const res = await this.program.account.bet.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async generateSignatureForBet(
    houseKeypair: Keypair,
    betId: number
  ): Promise<number[]> {
    try {
      const vaultPDA = this.getVaultPda();
      const betPDA = this.getBetPDA(vaultPDA, betId);
      const betAccount = await this.program.account.bet.fetch(betPDA);

      const message = this.createBetMessage({
        player: betAccount.player,
        amount: betAccount.amount,
        slot: betAccount.slot,
        id: betAccount.id,
        roll: betAccount.roll,
        bump: betAccount.bump,
      });

      return this.createEd25519Signature(houseKeypair, message);
    } catch (error) {
      console.error("Error generating signature:", error);
      throw error;
    }
  }

  private createBetMessage(bet: {
    player: PublicKey;
    amount: anchor.BN;
    slot: anchor.BN;
    id: anchor.BN;
    roll: number;
    bump: number;
  }): Uint8Array {
    const message = new Uint8Array(32 + 8 + 8 + 16 + 1 + 1);
    let offset = 0;

    // Player pubkey (32 bytes)
    message.set(bet.player.toBytes(), offset);
    offset += 32;

    // Amount as little-endian u64 (8 bytes)
    const amountBuffer = Buffer.alloc(8);
    amountBuffer.writeBigUInt64LE(BigInt(bet.amount.toString()), 0);
    message.set(amountBuffer, offset);
    offset += 8;

    // Slot as little-endian u64 (8 bytes)
    const slotBuffer = Buffer.alloc(8);
    slotBuffer.writeBigUInt64LE(BigInt(bet.slot.toString()), 0);
    message.set(slotBuffer, offset);
    offset += 8;

    // ID as little-endian u128 (16 bytes)
    const idBuffer = this.idToBuffer(bet.id);
    message.set(idBuffer, offset);
    offset += 16;

    // Roll (1 byte)
    message[offset] = bet.roll;
    offset += 1;

    // Bump (1 byte)
    message[offset] = bet.bump;

    return message;
  }

  private createEd25519Signature(
    houseKeypair: Keypair,
    message: Uint8Array
  ): number[] {
    // Sign the message with the house's private key
    const signature = ed25519.sign(
      message,
      houseKeypair.secretKey.slice(0, 32)
    );

    // Convert to number array for the program
    return Array.from(signature);
  }

  private idToBuffer(id: number | anchor.BN): Buffer {
    const idBuffer = Buffer.alloc(16);
    const idValue = typeof id === "number" ? BigInt(id) : BigInt(id.toString());

    // Write as little-endian u128 (16 bytes)
    // Split into two 64-bit parts
    const lower = idValue & BigInt("0xFFFFFFFFFFFFFFFF");
    const upper = idValue >> BigInt(64);

    // Write lower 64 bits at offset 0
    idBuffer.writeBigUInt64LE(lower, 0);
    // Write upper 64 bits at offset 8
    idBuffer.writeBigUInt64LE(upper, 8);

    return idBuffer;
  }
}

export class ChainHelpers {
  private provider: anchor.AnchorProvider;
  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams,
    isDisplayed: boolean
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

  async createMint(
    mintKeypair: anchor.web3.Keypair,
    decimals: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    // https://solanacookbook.com/references/token.html#how-to-create-a-new-token
    const rent = await spl.getMinimumBalanceForRentExemptMint(
      this.provider.connection
    );

    const instructions: anchor.web3.TransactionInstruction[] = [
      // create mint account
      SystemProgram.createAccount({
        fromPubkey: this.provider.wallet.publicKey,
        newAccountPubkey: mintKeypair.publicKey,
        space: spl.MINT_SIZE,
        lamports: rent,
        programId: spl.TOKEN_PROGRAM_ID,
      }),
      // init mint account
      spl.createInitializeMintInstruction(
        mintKeypair.publicKey,
        decimals,
        this.provider.wallet.publicKey, // mint authority
        this.provider.wallet.publicKey // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
      ),
    ];

    // pass the mint keypair as a signer
    const updatedParams = {
      ...params,
      signers: [...(params.signers || []), mintKeypair],
    };

    return await this.handleTx(instructions, updatedParams, isDisplayed);
  }

  async getOrCreateAta(
    mintPubkey: PublicKey,
    ownerPubkey: PublicKey,
    allowOwnerOffCurve = false,
    params: TxParams = {},
    isDisplayed: boolean = false
  ) {
    const { ata, ixs } = await getOrCreateAtaInstructions(
      this.provider.connection,
      this.provider.wallet.publicKey,
      mintPubkey,
      ownerPubkey,
      allowOwnerOffCurve
    );

    if (ixs.length) {
      const sig = await this.handleTx(ixs, params, isDisplayed);
      li({ createAta: sig });
    }

    return logAndReturn(ata, isDisplayed);
  }

  async mintTokens(
    amount: number,
    mint: PublicKey | string,
    recipient: PublicKey | string,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const pkMint = publicKeyFromString(mint);
    const pkRecipient = publicKeyFromString(recipient);

    const { ata: ataRecipient, ixs } = await getOrCreateAtaInstructions(
      this.provider.connection,
      this.provider.wallet.publicKey,
      pkMint,
      pkRecipient,
      true
    );

    const { decimals } = await spl.getMint(this.provider.connection, pkMint);

    const instructions: anchor.web3.TransactionInstruction[] = [
      ...ixs,
      spl.createMintToCheckedInstruction(
        pkMint,
        ataRecipient,
        this.provider.wallet.publicKey,
        amount * 10 ** decimals,
        decimals
      ),
    ];

    return await this.handleTx(instructions, params, isDisplayed);
  }

  async transferTokens(
    amount: number,
    mint: PublicKey | string,
    to: PublicKey | string,
    params: TxParams = {},
    isDisplayed: boolean = false
  ) {
    const pkFrom = this.provider.wallet.publicKey;
    const pkTo = publicKeyFromString(to);
    const pkMint = publicKeyFromString(mint);

    const [infoFrom, infoTo] = await Promise.all(
      [pkFrom, pkTo].map((owner) =>
        getOrCreateAtaInstructions(
          this.provider.connection,
          this.provider.wallet.publicKey,
          pkMint,
          owner,
          true
        )
      )
    );

    const { decimals } = await spl.getMint(this.provider.connection, pkMint);

    const instructions: anchor.web3.TransactionInstruction[] = [
      ...infoFrom.ixs,
      ...infoTo.ixs,
      spl.createTransferCheckedInstruction(
        infoFrom.ata,
        pkMint,
        infoTo.ata,
        this.provider.wallet.publicKey,
        amount * 10 ** decimals,
        decimals
      ),
    ];

    return await this.handleTx(instructions, params, isDisplayed);
  }

  async getBalance(
    publicKey: PublicKey | string,
    isDisplayed: boolean = false
  ): Promise<number> {
    const balance = await this.provider.connection.getBalance(
      publicKeyFromString(publicKey)
    );

    return logAndReturn(balance / anchor.web3.LAMPORTS_PER_SOL, isDisplayed);
  }

  async getTokenBalance(
    mint: PublicKey | string,
    owner: PublicKey | string,
    isDisplayed: boolean = false
  ): Promise<number> {
    const pkMint = publicKeyFromString(mint);
    const pkOwner = publicKeyFromString(owner);

    const ata = await spl.getAssociatedTokenAddress(
      pkMint,
      pkOwner,
      true,
      spl.TOKEN_PROGRAM_ID,
      spl.ASSOCIATED_TOKEN_PROGRAM_ID
    );

    let uiAmount: number | null = 0;

    try {
      ({
        value: { uiAmount },
      } = await this.provider.connection.getTokenAccountBalance(ata));
    } catch (_) {}

    return logAndReturn(uiAmount || 0, isDisplayed);
  }

  async getTx(signature: string, isDisplayed: boolean = false) {
    const tx = await this.provider.connection.getParsedTransaction(signature);

    return logAndReturn(tx, isDisplayed);
  }
}
