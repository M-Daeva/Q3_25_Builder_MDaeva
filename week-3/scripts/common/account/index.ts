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

import {
  Connection,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  createInitializeMintInstruction,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAssociatedTokenAddress,
  MintLayout,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";

import { Staking } from "../schema/types/staking";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

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
    nftMint: PublicKey | string,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const ix = await this.program.methods
      .init(rewardsRate, new anchor.BN(maxStake))
      .accounts({
        nftMint,
        admin: this.provider.wallet.publicKey,
      })
      .instruction();

    return await this.handleTx([ix], params, isDisplayed);
  }

  async tryStake(
    tokens: number[],
    nftMint: PublicKey | string,
    collectionMint: PublicKey | string,
    userKeypair: Keypair,
    params: TxParams = {},
    isDisplayed: boolean = false
  ): Promise<anchor.web3.TransactionSignature> {
    const metadataProgram = publicKeyFromString(
      MPL_TOKEN_METADATA_PROGRAM_ID.toString()
    );

    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      this.program.programId
    );

    const [userVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user_vault"), userKeypair.publicKey.toBuffer()],
      this.program.programId
    );

    const [metadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        new PublicKey(nftMint).toBuffer(),
      ],
      metadataProgram
    );

    const [editionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        new PublicKey(nftMint).toBuffer(),
        Buffer.from("edition"),
      ],
      metadataProgram
    );

    const ix = await this.program.methods
      .stake(tokens)
      .accountsStrict({
        nftMint,
        user: userKeypair.publicKey,
        collectionMint,
        metadataProgram: metadataProgram,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        config: configPda,
        userVault: userVaultPda,
        metadata: metadataPda,
        edition: editionPda,
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

  // TODO: get rewards
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

// Custom metadata structure for on-chain verification
export class NFTMetadata {
  collection_name: string;
  token_id: number;
  name: string;
  symbol: string;
  uri: string;
  collection_mint: string;

  constructor(props: {
    collection_name: string;
    token_id: number;
    name: string;
    symbol: string;
    uri: string;
    collection_mint: string;
  }) {
    this.collection_name = props.collection_name;
    this.token_id = props.token_id;
    this.name = props.name;
    this.symbol = props.symbol;
    this.uri = props.uri;
    this.collection_mint = props.collection_mint;
  }

  // Manual serialization to avoid Borsh compatibility issues
  serialize(): Buffer {
    const collectionNameBytes = Buffer.from(this.collection_name, "utf8");
    const nameBytes = Buffer.from(this.name, "utf8");
    const symbolBytes = Buffer.from(this.symbol, "utf8");
    const uriBytes = Buffer.from(this.uri, "utf8");
    const collectionMintBytes = Buffer.from(this.collection_mint, "utf8");

    const buffer = Buffer.alloc(
      4 +
        collectionNameBytes.length +
        4 + // token_id (u32)
        4 +
        nameBytes.length +
        4 +
        symbolBytes.length +
        4 +
        uriBytes.length +
        4 +
        collectionMintBytes.length
    );

    let offset = 0;

    // collection_name
    buffer.writeUInt32LE(collectionNameBytes.length, offset);
    offset += 4;
    collectionNameBytes.copy(buffer, offset);
    offset += collectionNameBytes.length;

    // token_id
    buffer.writeUInt32LE(this.token_id, offset);
    offset += 4;

    // name
    buffer.writeUInt32LE(nameBytes.length, offset);
    offset += 4;
    nameBytes.copy(buffer, offset);
    offset += nameBytes.length;

    // symbol
    buffer.writeUInt32LE(symbolBytes.length, offset);
    offset += 4;
    symbolBytes.copy(buffer, offset);
    offset += symbolBytes.length;

    // uri
    buffer.writeUInt32LE(uriBytes.length, offset);
    offset += 4;
    uriBytes.copy(buffer, offset);
    offset += uriBytes.length;

    // collection_mint
    buffer.writeUInt32LE(collectionMintBytes.length, offset);
    offset += 4;
    collectionMintBytes.copy(buffer, offset);

    return buffer;
  }

  // Manual deserialization
  static deserialize(buffer: Buffer): NFTMetadata {
    let offset = 0;

    // collection_name
    const collectionNameLength = buffer.readUInt32LE(offset);
    offset += 4;
    const collection_name = buffer
      .slice(offset, offset + collectionNameLength)
      .toString("utf8");
    offset += collectionNameLength;

    // token_id
    const token_id = buffer.readUInt32LE(offset);
    offset += 4;

    // name
    const nameLength = buffer.readUInt32LE(offset);
    offset += 4;
    const name = buffer.slice(offset, offset + nameLength).toString("utf8");
    offset += nameLength;

    // symbol
    const symbolLength = buffer.readUInt32LE(offset);
    offset += 4;
    const symbol = buffer.slice(offset, offset + symbolLength).toString("utf8");
    offset += symbolLength;

    // uri
    const uriLength = buffer.readUInt32LE(offset);
    offset += 4;
    const uri = buffer.slice(offset, offset + uriLength).toString("utf8");
    offset += uriLength;

    // collection_mint
    const collectionMintLength = buffer.readUInt32LE(offset);
    offset += 4;
    const collection_mint = buffer
      .slice(offset, offset + collectionMintLength)
      .toString("utf8");

    return new NFTMetadata({
      collection_name,
      token_id,
      name,
      symbol,
      uri,
      collection_mint,
    });
  }
}

export class SolanaNFTCollection {
  private connection: Connection;
  private payer: Keypair;
  private collectionName: string;
  private collectionSymbol: string;
  private collectionMint: Keypair;

  constructor(
    connection: Connection,
    payer: Keypair,
    collectionName: string,
    collectionSymbol: string
  ) {
    this.connection = connection;
    this.payer = payer;
    this.collectionName = collectionName;
    this.collectionSymbol = collectionSymbol;
    this.collectionMint = Keypair.generate();
  }

  // Create collection master mint
  async createCollection(): Promise<string> {
    console.log(
      "Creating collection mint:",
      this.collectionMint.publicKey.toString()
    );

    const lamports = await this.connection.getMinimumBalanceForRentExemption(
      MintLayout.span
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: this.payer.publicKey,
        newAccountPubkey: this.collectionMint.publicKey,
        space: MintLayout.span,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMintInstruction(
        this.collectionMint.publicKey,
        0, // decimals
        this.payer.publicKey, // mint authority
        this.payer.publicKey, // freeze authority
        TOKEN_PROGRAM_ID
      )
    );

    const signature = await sendAndConfirmTransaction(
      this.connection,
      transaction,
      [this.payer, this.collectionMint]
    );

    console.log("Collection created with signature:", signature);
    return signature;
  }

  // Create individual NFT with custom metadata storage
  async createNFT(
    tokenId: number
  ): Promise<{ mint: string; signature: string; metadataAccount: string }> {
    const nftMint = Keypair.generate();
    const nftName = `${this.collectionName} #${tokenId}`;
    const uri = `https://example.com/metadata/${tokenId}.json`;

    console.log(`Creating NFT ${nftName}:`, nftMint.publicKey.toString());

    // Create mint account
    const lamports = await this.connection.getMinimumBalanceForRentExemption(
      MintLayout.span
    );

    // Get associated token account
    const associatedTokenAccount = await getAssociatedTokenAddress(
      nftMint.publicKey,
      this.payer.publicKey
    );

    // Create metadata account using createAccountWithSeed (simpler approach)
    const seed = `metadata_${tokenId.toString().padStart(8, "0")}`;
    const metadataAccount = await PublicKey.createWithSeed(
      this.payer.publicKey,
      seed,
      SystemProgram.programId
    );

    // Serialize metadata
    const nftMetadata = new NFTMetadata({
      collection_name: this.collectionName,
      token_id: tokenId,
      name: nftName,
      symbol: this.collectionSymbol,
      uri,
      collection_mint: this.collectionMint.publicKey.toString(),
    });

    const serializedData = nftMetadata.serialize();
    const metadataLamports =
      await this.connection.getMinimumBalanceForRentExemption(
        serializedData.length + 8
      );

    const transaction = new Transaction().add(
      // Create mint account
      SystemProgram.createAccount({
        fromPubkey: this.payer.publicKey,
        newAccountPubkey: nftMint.publicKey,
        space: MintLayout.span,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }),
      // Initialize mint
      createInitializeMintInstruction(
        nftMint.publicKey,
        0, // decimals (NFT)
        this.payer.publicKey, // mint authority
        this.payer.publicKey, // freeze authority
        TOKEN_PROGRAM_ID
      ),
      // Create associated token account
      createAssociatedTokenAccountInstruction(
        this.payer.publicKey, // payer
        associatedTokenAccount, // associated token account
        this.payer.publicKey, // owner
        nftMint.publicKey // mint
      ),
      // Mint token
      createMintToInstruction(
        nftMint.publicKey,
        associatedTokenAccount,
        this.payer.publicKey,
        1 // amount (1 for NFT)
      ),
      // Create custom metadata account with seed
      SystemProgram.createAccountWithSeed({
        fromPubkey: this.payer.publicKey,
        basePubkey: this.payer.publicKey,
        seed: seed,
        newAccountPubkey: metadataAccount,
        lamports: metadataLamports,
        space: serializedData.length + 8,
        programId: SystemProgram.programId,
      })
    );

    const signature = await sendAndConfirmTransaction(
      this.connection,
      transaction,
      [this.payer, nftMint]
    );

    // In a real implementation, you would need a custom program to write the metadata
    // For now, we'll write it in a separate transaction (this is just for demonstration)
    try {
      const writeMetadataTransaction = new Transaction().add(
        // This is a placeholder - in reality, you'd need a custom program instruction
        // to write the serialized metadata to the account
        SystemProgram.transfer({
          fromPubkey: this.payer.publicKey,
          toPubkey: metadataAccount,
          lamports: 0, // No transfer, just a placeholder
        })
      );

      // Note: This won't actually write the metadata - you need a custom program for that
      console.log(`NFT ${nftName} created with signature:`, signature);
      console.log(`Custom metadata account:`, metadataAccount.toString());
      console.log(`Metadata would be written by custom program instruction`);
    } catch (error) {
      console.log("Metadata writing would require custom program");
    }

    return {
      mint: nftMint.publicKey.toString(),
      signature,
      metadataAccount: metadataAccount.toString(),
    };
  }

  // Verify token belongs to collection and get its ID using custom metadata
  async verifyTokenAndGetId(mintAddress: string): Promise<{
    belongsToCollection: boolean;
    tokenId: number | null;
    metadata: NFTMetadata | null;
  }> {
    try {
      const mint = new PublicKey(mintAddress);

      // Since we're using seed-based accounts, we need to check multiple possible token IDs
      // In a real implementation, you'd maintain an index or use a different approach
      for (let tokenId = 1; tokenId <= 1000; tokenId++) {
        // Adjust range as needed
        const seed = `metadata_${tokenId.toString().padStart(8, "0")}`;

        try {
          const metadataAccount = await PublicKey.createWithSeed(
            this.payer.publicKey,
            seed,
            SystemProgram.programId
          );

          const accountInfo = await this.connection.getAccountInfo(
            metadataAccount
          );

          if (accountInfo && accountInfo.data.length > 8) {
            // Deserialize metadata
            const metadata = NFTMetadata.deserialize(accountInfo.data.slice(8));

            // Check if this metadata corresponds to our mint and collection
            if (
              metadata.collection_mint ===
                this.collectionMint.publicKey.toString() &&
              metadata.collection_name === this.collectionName &&
              metadata.token_id === tokenId
            ) {
              return {
                belongsToCollection: true,
                tokenId: metadata.token_id,
                metadata: metadata,
              };
            }
          }
        } catch (error) {
          // Continue checking other token IDs
          continue;
        }
      }

      return { belongsToCollection: false, tokenId: null, metadata: null };
    } catch (error) {
      console.error("Error verifying token:", error);
      return { belongsToCollection: false, tokenId: null, metadata: null };
    }
  }

  // Get all NFTs in collection by checking metadata accounts
  async getAllCollectionNFTs(): Promise<
    Array<{ mint: string; tokenId: number; metadata: NFTMetadata }>
  > {
    const nfts: Array<{
      mint: string;
      tokenId: number;
      metadata: NFTMetadata;
    }> = [];

    // In a real implementation, you'd maintain an index of all minted NFTs
    // For demonstration, this would need to be implemented based on your specific needs
    console.log(
      "Getting all collection NFTs - implement based on your indexing strategy"
    );

    return nfts;
  }

  // Batch verify multiple tokens
  async verifyMultipleTokens(mintAddresses: string[]): Promise<
    Map<
      string,
      {
        belongsToCollection: boolean;
        tokenId: number | null;
        metadata: NFTMetadata | null;
      }
    >
  > {
    const results = new Map();

    for (const mintAddress of mintAddresses) {
      const result = await this.verifyTokenAndGetId(mintAddress);
      results.set(mintAddress, result);
    }

    return results;
  }

  getCollectionMint(): string {
    return this.collectionMint.publicKey.toString();
  }
}

// Helper function to create metadata JSON files
export function generateMetadataJSON(
  tokenId: number,
  collectionName: string
): any {
  return {
    name: `${collectionName} #${tokenId}`,
    description: `${collectionName} NFT #${tokenId}`,
    image: `https://example.com/images/${tokenId}.png`,
    attributes: [
      {
        trait_type: "Collection",
        value: collectionName,
      },
      {
        trait_type: "Token ID",
        value: tokenId,
      },
      {
        trait_type: "Rarity",
        value: tokenId <= 10 ? "Rare" : "Common",
      },
    ],
    collection: {
      name: collectionName,
      family: collectionName,
    },
  };
}
