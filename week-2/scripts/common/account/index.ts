import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TraderInfo, TxParams } from "../interfaces";
import {
  getHandleTx,
  getOrCreateAtaInstructions,
  getProgram,
  l,
  li,
  logAndReturn,
  publicKeyFromString,
} from "../../common/utils";

import { Vault } from "../schema/types/vault";
import { Escrow } from "../schema/types/escrow";
import { Amm } from "../schema/types/amm";
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

export class EscrowHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Escrow>;

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

  constructor(
    provider: anchor.AnchorProvider,
    program: anchor.Program<Escrow>
  ) {
    this.provider = provider;
    this.program = program;
    this.handleTx = getHandleTx(provider);
  }

  async tryMake(
    id: number,
    maker: TraderInfo,
    taker: TraderInfo,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const getTrader = (trader: TraderInfo) => {
      return {
        trader: publicKeyFromString(trader.trader),
        amount: new anchor.BN(trader.amount),
        mint: publicKeyFromString(trader.mint),
      };
    };

    const makerPubkey = publicKeyFromString(maker.trader);
    const [escrowStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), makerPubkey.toBuffer(), Buffer.from([id])],
      this.program.programId
    );

    const ix = await this.program.methods
      .make(id, getTrader(maker), getTrader(taker))
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        maker: makerPubkey,
        escrowState: escrowStatePda,
        makerMint: publicKeyFromString(maker.mint),
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryRefund(
    id: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const makerPubkey = this.provider.wallet.publicKey;
    const [escrowStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), makerPubkey.toBuffer(), Buffer.from([id])],
      this.program.programId
    );

    const {
      maker: { mint },
    } = await this.program.account.escrow.fetch(escrowStatePda);

    const ix = await this.program.methods
      .refund(id)
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        maker: makerPubkey,
        escrowState: escrowStatePda,
        makerMint: mint,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryTake(
    id: number,
    maker: PublicKey | string,
    takerKeypair: Keypair,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const makerPubkey = publicKeyFromString(maker);
    const [escrowStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), makerPubkey.toBuffer(), Buffer.from([id])],
      this.program.programId
    );

    const {
      maker: { mint: makerMint },
      taker: { mint: takerMint, trader: takerPubkey },
    } = await this.program.account.escrow.fetch(escrowStatePda);

    const ix = await this.program.methods
      .take(id, makerPubkey)
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        maker: makerPubkey,
        taker: takerPubkey,
        escrowState: escrowStatePda,
        makerMint,
        takerMint,
      })
      .instruction();

    // add taker as signer
    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), takerKeypair],
    };

    return await this.handleTx([ix], modifiedParams, isDisplayed);
  }

  async getEscrowState(
    maker: PublicKey | string,
    id: number,
    isDisplayed: boolean = false
  ) {
    const [escrowStatePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        publicKeyFromString(maker).toBuffer(),
        Buffer.from([id]),
      ],
      this.program.programId
    );

    const vaultState = await this.program.account.escrow.fetch(escrowStatePda);

    return logAndReturn(vaultState, isDisplayed);
  }

  async getVault(
    maker: PublicKey | string,
    id: number,
    isDisplayed: boolean = false
  ) {
    const chain = new ChainHelpers(this.provider);
    const escrowState = await this.getEscrowState(maker, id);
    const vaultAta = await chain.getOrCreateAta(
      escrowState.maker.mint,
      publicKeyFromString(maker)
    );

    return logAndReturn(vaultAta, isDisplayed);
  }
}

export class AmmHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Amm>;

  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams,
    isDisplayed: boolean
  ) => Promise<anchor.web3.TransactionSignature>;

  constructor(provider: anchor.AnchorProvider, program: anchor.Program<Amm>) {
    this.provider = provider;
    this.program = program;
    this.handleTx = getHandleTx(provider);
  }

  async tryCreatePool(
    id: number,
    mintX: PublicKey | string,
    mintY: PublicKey | string,
    feeBps: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .createPool(
        new anchor.BN(id),
        publicKeyFromString(mintX),
        publicKeyFromString(mintY),
        feeBps
      )
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        mintX,
        mintY,
        poolCreator: this.provider.wallet.publicKey,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryProvideLiquidity(
    id: number,
    mintXAmount: number,
    mintYAmount: number,
    liquidityProviderKeypair: Keypair,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const { mintX, mintY } = await this.getPoolConfig(id);

    const ix = await this.program.methods
      .provideLiquidity(
        new anchor.BN(id),
        new anchor.BN(mintXAmount),
        new anchor.BN(mintYAmount)
      )
      .accounts({
        mintX,
        mintY,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        liquidityProvider: liquidityProviderKeypair.publicKey,
      })
      .instruction();

    // add liquidityProviderKeypair as signer
    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), liquidityProviderKeypair],
    };

    return await this.handleTx([ix], modifiedParams, isDisplayed);
  }

  async getPoolConfig(id: number, isDisplayed: boolean = false) {
    const [poolConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config"), new anchor.BN(id).toArrayLike(Buffer, "le", 8)],
      this.program.programId
    );

    const poolConfig = await this.program.account.poolConfig.fetch(
      poolConfigPda
    );

    return logAndReturn(poolConfig, isDisplayed);
  }

  async getPoolBalance(id: number, isDisplayed: boolean = false) {
    const [poolBalancePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("balance"), new anchor.BN(id).toArrayLike(Buffer, "le", 8)],
      this.program.programId
    );

    const poolBalance = await this.program.account.poolBalance.fetch(
      poolBalancePda
    );

    return logAndReturn(poolBalance, isDisplayed);
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
