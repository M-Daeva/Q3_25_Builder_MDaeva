import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TxParams } from "../interfaces";
import {
  getHandleTx,
  getOrCreateAtaInstructions,
  getTokenProgramFactory,
  li,
  logAndReturn,
  publicKeyFromString,
} from "../../common/utils";

import { Nft } from "../schema/types/nft";
import { Staking } from "../schema/types/staking";
import { Marketplace } from "../schema/types/marketplace";

export type Asset = "sol" | PublicKey;
export type IdlAsset = { sol: {} } | { mint: { 0: PublicKey } };

// function getAssets(assets: Asset[]): IdlAsset[] {
//   return assets.map((x) => (x === "sol" ? { sol: {} } : { mint: { 0: x } }));
// }

export class NftHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Nft>;

  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams,
    isDisplayed: boolean
  ) => Promise<anchor.web3.TransactionSignature>;

  constructor(provider: anchor.AnchorProvider, program: anchor.Program<Nft>) {
    this.provider = provider;
    this.program = program;
    this.handleTx = getHandleTx(provider);
  }

  async tryCreateCollection(
    id: number,
    metadata: string,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .createCollection(id, metadata)
      .accounts({
        admin: this.provider.wallet.publicKey,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryMintToken(
    id: number,
    metadata: string,
    recipient: PublicKey | string,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .mintToken(id, metadata)
      .accounts({
        admin: this.provider.wallet.publicKey,
        recipient: publicKeyFromString(recipient),
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async getCollection(
    admin: PublicKey | string,
    id: number,
    isDisplayed: boolean = false
  ) {
    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("collection"),
        publicKeyFromString(admin).toBuffer(),
        Buffer.from([id]),
      ],
      this.program.programId
    );

    const res = await this.program.account.collection.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async getToken(
    collection: PublicKey | string,
    id: number,
    isDisplayed: boolean = false
  ) {
    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("token"),
        publicKeyFromString(collection).toBuffer(),
        new anchor.BN(id).toArrayLike(Buffer, "le", 2), // TODO: create a helper
      ],
      this.program.programId
    );

    const res = await this.program.account.token.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }
}

export class StakingHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Staking>;

  private handleTx: (
    instructions: anchor.web3.TransactionInstruction[],
    params: TxParams,
    isDisplayed: boolean
  ) => Promise<anchor.web3.TransactionSignature>;

  constructor(
    provider: anchor.AnchorProvider,
    program: anchor.Program<Staking>
  ) {
    this.provider = provider;
    this.program = program;
    this.handleTx = getHandleTx(provider);
  }

  async tryInit(
    rewardsRate: number,
    maxStake: number,
    collection: PublicKey | string,
    nftMint: PublicKey | string,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .init(
        rewardsRate,
        new anchor.BN(maxStake),
        publicKeyFromString(collection)
      )
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        nftMint,
        admin: this.provider.wallet.publicKey,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryStake(
    tokenId: number,
    nftMint: PublicKey | string,
    nftProgram: anchor.Address,
    userKeypair: Keypair,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .stake(tokenId)
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        nftMint,
        user: userKeypair.publicKey,
        nftProgram,
      })
      .instruction();

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx([ix], modifiedParams, isDisplayed);
  }

  async tryUnstake(
    tokenId: number,
    nftMint: PublicKey | string,
    userKeypair: Keypair,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .unstake(tokenId)
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        nftMint,
        user: userKeypair.publicKey,
      })
      .instruction();

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx([ix], modifiedParams, isDisplayed);
  }

  async tryClaim(
    userKeypair: Keypair,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .claim()
      .accounts({
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        user: userKeypair.publicKey,
      })
      .instruction();

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx([ix], modifiedParams, isDisplayed);
  }

  async getConfig(isDisplayed: boolean = false) {
    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      this.program.programId
    );

    const poolConfig = await this.program.account.config.fetch(configPda);

    return logAndReturn(poolConfig, isDisplayed);
  }

  async getUserVault(user: PublicKey | string, isDisplayed: boolean = false) {
    const [userVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user_vault"), publicKeyFromString(user).toBuffer()],
      this.program.programId
    );

    const userVault = await this.program.account.vault.fetch(userVaultPda);

    return logAndReturn(userVault, isDisplayed);
  }
}

export class MarketplaceHelpers {
  private provider: anchor.AnchorProvider;
  private program: anchor.Program<Marketplace>;

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
    program: anchor.Program<Marketplace>
  ) {
    this.provider = provider;
    this.program = program;
    this.handleTx = getHandleTx(provider);
    this.getTokenProgram = getTokenProgramFactory(provider);
  }

  async tryInit(
    feeBps: number,
    collectionWhitelist: PublicKey[],
    assetWhitelist: PublicKey[],
    name: string,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .init(feeBps, collectionWhitelist, assetWhitelist, name)
      .accounts({})
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryCreateTrade(
    userKeypair: Keypair,
    nftProgram: anchor.Address,
    nftMint: PublicKey,
    tokenMint: PublicKey,
    isSellNftTrade: boolean,
    collection: PublicKey,
    tokenId: number,
    priceAmount: number,
    priceAsset: PublicKey,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const { admin } = await this.getMarketplace();

    let instructions: anchor.web3.TransactionInstruction[] = [];

    if (isSellNftTrade) {
      if (priceAsset.equals(PublicKey.default)) {
        const tokenProgram = await this.getTokenProgram(nftMint);

        const ix = await this.program.methods
          .createSellForSolTrade(collection, tokenId, {
            amount: new anchor.BN(priceAmount),
            asset: priceAsset,
          })
          .accounts({
            admin,
            seller: userKeypair.publicKey,
            tokenProgram,
            nftProgram,
            nftMint,
          })
          .instruction();

        instructions.push(ix);
      } else {
        const tokenProgram = await this.getTokenProgram(priceAsset);

        const ix = await this.program.methods
          .createSellForTokenTrade(collection, tokenId, {
            amount: new anchor.BN(priceAmount),
            asset: priceAsset,
          })
          .accounts({
            admin,
            seller: userKeypair.publicKey,
            tokenProgram,
            nftProgram,
            nftMint,
            tokenMint,
          })
          .instruction();

        instructions.push(ix);
      }
    } else {
      if (priceAsset.equals(PublicKey.default)) {
        const tokenProgram = await this.getTokenProgram(nftMint);

        const ix = await this.program.methods
          .createBuyWithSolTrade(collection, tokenId, {
            amount: new anchor.BN(priceAmount),
            asset: priceAsset,
          })
          .accounts({
            admin,
            buyer: userKeypair.publicKey,
            tokenProgram,
            nftProgram,
            nftMint,
          })
          .instruction();

        instructions.push(ix);
      } else {
        const tokenProgram = await this.getTokenProgram(priceAsset);

        const ix = await this.program.methods
          .createBuyWithTokenTrade(collection, tokenId, {
            amount: new anchor.BN(priceAmount),
            asset: priceAsset,
          })
          .accounts({
            admin,
            buyer: userKeypair.publicKey,
            tokenProgram,
            nftProgram,
            nftMint,
            tokenMint,
          })
          .instruction();

        instructions.push(ix);
      }
    }

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx(instructions, modifiedParams, isDisplayed);
  }

  async tryAcceptTrade(
    userKeypair: Keypair,
    nftProgram: anchor.Address,
    nftMint: PublicKey,
    tokenMint: PublicKey,
    creator: PublicKey,
    collection: PublicKey,
    tokenId: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const { admin } = await this.getMarketplace();
    const trade = await this.getTrade(creator, collection, tokenId);

    let instructions: anchor.web3.TransactionInstruction[] = [];

    if (trade.isSellNftTrade) {
      if (trade.price.asset.equals(PublicKey.default)) {
        const tokenProgram = await this.getTokenProgram(nftMint);

        const ix = await this.program.methods
          .acceptSellForSolTrade(collection, tokenId)
          .accounts({
            admin,
            buyer: userKeypair.publicKey,
            seller: creator,
            tokenProgram,
            nftMint,
          })
          .instruction();

        instructions.push(ix);
      } else {
        const tokenProgram = await this.getTokenProgram(trade.price.asset);

        const ix = await this.program.methods
          .acceptSellForTokenTrade(collection, tokenId)
          .accounts({
            admin,
            buyer: userKeypair.publicKey,
            seller: creator,
            tokenProgram,
            nftMint,
            tokenMint,
          })
          .instruction();

        instructions.push(ix);
      }
    } else {
      if (trade.price.asset.equals(PublicKey.default)) {
        const tokenProgram = await this.getTokenProgram(nftMint);

        const ix = await this.program.methods
          .acceptBuyWithSolTrade(collection, tokenId)
          .accounts({
            admin,
            buyer: creator,
            seller: userKeypair.publicKey,
            tokenProgram,
            nftProgram,
            nftMint,
          })
          .instruction();

        instructions.push(ix);
      } else {
        const tokenProgram = await this.getTokenProgram(trade.price.asset);

        const ix = await this.program.methods
          .acceptBuyWithTokenTrade(collection, tokenId)
          .accounts({
            admin,
            buyer: creator,
            seller: userKeypair.publicKey,
            tokenProgram,
            nftProgram,
            nftMint,
            tokenMint,
          })
          .instruction();

        instructions.push(ix);
      }
    }

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx(instructions, modifiedParams, isDisplayed);
  }

  async tryRemoveTrade(
    userKeypair: Keypair,
    nftProgram: anchor.Address,
    nftMint: PublicKey,
    tokenMint: PublicKey,
    creator: PublicKey,
    collection: PublicKey,
    tokenId: number,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const { admin } = await this.getMarketplace();
    const trade = await this.getTrade(creator, collection, tokenId);

    let instructions: anchor.web3.TransactionInstruction[] = [];

    if (trade.isSellNftTrade) {
      const mint = trade.price.asset.equals(PublicKey.default)
        ? nftMint
        : trade.price.asset;
      const tokenProgram = await this.getTokenProgram(mint);

      const ix = await this.program.methods
        .removeSellTrade(collection, tokenId)
        .accounts({
          admin,
          seller: userKeypair.publicKey,
          tokenProgram,
          nftProgram,
          nftMint,
        })
        .instruction();

      instructions.push(ix);
    } else {
      if (trade.price.asset.equals(PublicKey.default)) {
        const ix = await this.program.methods
          .removeBuyWithSolTrade(collection, tokenId)
          .accounts({
            admin,
            buyer: userKeypair.publicKey,
          })
          .instruction();

        instructions.push(ix);
      } else {
        const tokenProgram = await this.getTokenProgram(trade.price.asset);

        const ix = await this.program.methods
          .removeBuyWithTokenTrade(collection, tokenId)
          .accounts({
            admin,
            buyer: userKeypair.publicKey,
            tokenProgram,
            tokenMint,
          })
          .instruction();

        instructions.push(ix);
      }
    }

    const modifiedParams = {
      ...params,
      signers: [...(params.signers || []), userKeypair],
    };

    return await this.handleTx(instructions, modifiedParams, isDisplayed);
  }

  async tryWithdrawFee(
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const { admin } = await this.getMarketplace();
    const { value: balances } = await this.getBalances();

    let promiseList: Promise<anchor.web3.TransactionInstruction>[] = [];

    for (const { asset } of balances) {
      if (asset.equals(PublicKey.default)) {
        const ix = this.program.methods
          .withdrawSolFee()
          .accounts({
            admin,
            sender: this.provider.wallet.publicKey,
          })
          .instruction();

        promiseList.push(ix);
      } else {
        const tokenProgram = await this.getTokenProgram(asset);

        const ix = this.program.methods
          .withdrawTokenFee()
          .accounts({
            admin,
            sender: this.provider.wallet.publicKey,
            tokenProgram,
            tokenMint: asset,
          })
          .instruction();

        promiseList.push(ix);
      }
    }

    const ixs = await Promise.all(promiseList);
    return await this.handleTx(ixs, params, isDisplayed);
  }

  async getMarketplace(isDisplayed: boolean = false) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), this.provider.wallet.publicKey.toBuffer()],
      this.program.programId
    );

    const res = await this.program.account.marketplace.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async getBalances(isDisplayed: boolean = false) {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("balances"), this.provider.wallet.publicKey.toBuffer()],
      this.program.programId
    );

    const res = await this.program.account.balances.fetch(pda);

    return logAndReturn(res, isDisplayed);
  }

  async getTrade(
    user: PublicKey,
    collection: PublicKey,
    tokenId: number,
    isDisplayed: boolean = false
  ) {
    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("trade"),
        user.toBuffer(),
        collection.toBuffer(),
        new anchor.BN(tokenId).toArrayLike(Buffer, "le", 2),
      ],
      this.program.programId
    );

    const res = await this.program.account.trade.fetch(pda);

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
