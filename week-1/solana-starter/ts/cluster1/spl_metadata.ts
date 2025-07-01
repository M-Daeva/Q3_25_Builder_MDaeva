import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { getWallet } from "./wallet";
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";

// Define our Mint address
const mint = publicKey("fPcP9vGoowPikgu7oTRCJKHUvSNn9N5WZhYshR4UXyo");

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");

(async () => {
  try {
    // Start here
    const { secretKey } = await getWallet();
    const keypair = umi.eddsa.createKeypairFromSecretKey(secretKey);
    const signer = createSignerFromKeypair(umi, keypair);
    umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

    const accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint,
      payer: signer,
      mintAuthority: signer,
      updateAuthority: signer,
    };

    const data: DataV2Args = {
      name: "awesome token",
      symbol: "AWSM",
      uri: "https://assets.coingecko.com/coins/images/4128/standard/solana.png?1718769756",
      sellerFeeBasisPoints: 0,
      creators: [{ address: keypair.publicKey, share: 100, verified: false }],
      collection: null,
      uses: null,
    };

    const args: CreateMetadataAccountV3InstructionArgs = {
      data,
      isMutable: true,
      collectionDetails: null,
    };

    const tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });
    const result = await tx.sendAndConfirm(umi);
    console.log(bs58.encode(result.signature)); // 5jPh8Mban2AUDio5SsEpbtEUJdJbV4vgNwSnktzYcoYosh2xjCfUeuWQp4Hdt3cWjv89SpmuLgtR6PyDaiAtFofr
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
