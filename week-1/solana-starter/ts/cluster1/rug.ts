import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { getWallet } from "./wallet";
import base58 from "bs58";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
  createGenericFile,
} from "@metaplex-foundation/umi";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const mint = generateSigner(umi);

let imageUri: string = "";
let metadataUri: string = "";

(async () => {
  try {
    const { secretKey } = await getWallet();
    const keypair = umi.eddsa.createKeypairFromSecretKey(secretKey);
    const signer = createSignerFromKeypair(umi, keypair);

    umi.use(irysUploader());
    umi.use(signerIdentity(signer));
    umi.use(mplTokenMetadata());

    // upload image
    const filePath = "./cluster1/img/generug.png";
    const buffer = await readFile(filePath);
    const imageFile = createGenericFile(buffer, filePath, {
      contentType: "image/jpeg",
    });

    [imageUri] = await umi.uploader.upload([imageFile]);

    console.log("Your image URI: ", imageUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }

  // upload metadata
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    const metadata = {
      name: "Rug",
      symbol: "RUG",
      description: "Just a nice rug",
      image: imageUri,
      attributes: [{ trait_type: "rarity", value: "9000+" }],
      properties: {
        files: [
          {
            type: "image/png",
            uri: imageUri,
          },
        ],
      },
      creators: [],
    };
    metadataUri = await umi.uploader.uploadJson(metadata);
    console.log("Your metadata URI: ", metadataUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }

  // mint NFT
  try {
    const tx = createNft(umi, {
      mint,
      name: "Rug",
      symbol: "RUG",
      uri: metadataUri,
      updateAuthority: umi.identity.publicKey,
      sellerFeeBasisPoints: percentAmount(4.2),
    });

    const result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);

    console.log(
      `Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
    );

    console.log("Mint Address: ", mint.publicKey);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }

  // Your image URI:  https://gateway.irys.xyz/3B5dnaMpD18X3ZxiWiYxoA5qpL51QiKMi8goLWyjCHHi
  // Your metadata URI:  https://gateway.irys.xyz/t1NsxAaR8jQMF9nS9gTbUtZ56Btg4mXQaK38DjSpz2F
  // Succesfully Minted! Check out your TX here:
  // https://explorer.solana.com/tx/648uysMxMg6nQh24qWZ1q3cYZXSH29JVVKbKQoCv6KiBF5ydrywhZv8J2ZL9T9kEy2q6AwG1FYZVuZ2pkKEVFPSw?cluster=devnet
  // Mint Address:  FygAKLQbZLokYoF5wUZJYbuDgEd1A4U6x2FWuAqAaWES
})();
