import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { TxParams } from "../interfaces";
import {
  getHandleTx,
  getOrCreateAtaInstructions,
  getTokenProgramFactory,
  li,
  logAndReturn,
  publicKeyFromString,
} from "../../common/utils";
import {
  ActivateAccountArgs,
  CreateAccountArgs,
  InitArgs,
  ReopenAccountArgs,
  RequestAccountRotationArgs,
  UpdateConfigArgs,
  WithdrawRevenueArgs,
  WriteDataArgs,
} from "../interfaces/registry";
import {
  convertCreateAccountArgs,
  convertInitArgs,
  convertReopenAccountArgs,
  convertRequestAccountRotationArgs,
  convertUpdateConfigArgs,
  convertWithdrawRevenueArgs,
  convertWriteDataArgs,
} from "../interfaces/registry.anchor";

import { Registry } from "../schema/types/registry";

export class RegistryHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Registry>;
  private sender: PublicKey;

  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams,
    isDisplayed: boolean
  ) => Promise<anchor.web3.TransactionSignature>;

  private getTokenProgram: (
    mint: anchor.web3.PublicKey
  ) => Promise<anchor.web3.PublicKey>;

  constructor(
    provider: anchor.AnchorProvider,
    program: anchor.Program<Registry>
  ) {
    this.provider = provider;
    this.program = program;
    this.sender = provider.wallet.publicKey;
    this.handleTx = getHandleTx(provider);
    this.getTokenProgram = getTokenProgramFactory(provider);
  }

  async tryInit(
    args: InitArgs,
    revenueMint: PublicKey,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .init(...convertInitArgs(args))
      .accounts({
        tokenProgram: await this.getTokenProgram(revenueMint),
        revenueMint,
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryUpdateConfig(
    args: UpdateConfigArgs,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .updateConfig(...convertUpdateConfigArgs(args))
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryConfirmAdminRotation(
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .confirmAdminRotation()
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryWithdrawRevenue(
    args: WithdrawRevenueArgs,
    revenueMint: PublicKey,
    recipient?: PublicKey,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .withdrawRevenue(...convertWithdrawRevenueArgs(args))
      .accounts({
        tokenProgram: await this.getTokenProgram(revenueMint),
        revenueMint,
        sender: this.sender,
        recipient: recipient || this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  // TODO: add simulation
  async tryCreateAccount(
    args: CreateAccountArgs,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .createAccount(...convertCreateAccountArgs(args))
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryCloseAccount(
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .closeAccount()
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryReopenAccount(
    args: ReopenAccountArgs,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .reopenAccount(...convertReopenAccountArgs(args))
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryActivateAccount(
    args: ActivateAccountArgs,
    revenueMint: PublicKey,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .activateAccount(args.user || this.sender)
      .accounts({
        tokenProgram: await this.getTokenProgram(revenueMint),
        sender: this.sender,
        revenueMint,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  // TODO: add encryption
  async tryWriteData(
    args: WriteDataArgs,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .writeData(...convertWriteDataArgs(args))
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryRequestAccountRotation(
    args: RequestAccountRotationArgs,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .requestAccountRotation(...convertRequestAccountRotationArgs(args))
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryConfirmAccountRotation(
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .confirmAccountRotation()
      .accounts({
        sender: this.sender,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async queryConfig(isDisplayed: boolean = false) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      this.program.programId
    );
    const res = await this.program.account.config.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async queryUserCounter(isDisplayed: boolean = false) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user_counter")],
      this.program.programId
    );
    const res = await this.program.account.userCounter.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async queryAdminRotationState(isDisplayed: boolean = false) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("admin_rotation_state")],
      this.program.programId
    );
    const res = await this.program.account.rotationState.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async queryUserId(user: PublicKey, isDisplayed: boolean = false) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user_id"), user.toBuffer()],
      this.program.programId
    );
    const res = await this.program.account.userId.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async queryUserAccount(user: PublicKey, isDisplayed: boolean = false) {
    const { id } = await this.queryUserId(user);

    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("user_account"),
        new anchor.BN(id).toArrayLike(Buffer, "le", 1),
      ],
      this.program.programId
    );
    const res = await this.program.account.userAccount.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async queryUserRotationState(user: PublicKey, isDisplayed: boolean = false) {
    const { id } = await this.queryUserId(user);

    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("user_rotation_state"),
        new anchor.BN(id).toArrayLike(Buffer, "le", 1),
      ],
      this.program.programId
    );
    const res = await this.program.account.rotationState.fetch(pda);

    return logAndReturn(res, isDisplayed);
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
