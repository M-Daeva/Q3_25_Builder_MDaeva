import { getWallet } from "./wallet";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

(async () => {
  try {
    const { secretKey } = await getWallet();
    const keypair = umi.eddsa.createKeypairFromSecretKey(secretKey);
    const signer = createSignerFromKeypair(umi, keypair);

    umi.use(irysUploader());
    umi.use(signerIdentity(signer));

    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    const image =
      "https://gateway.irys.xyz/DB6vMrkFNqapPT488Hqvi2cQFJAnMc1QYr11Ttw5fpfA";
    const metadata = {
      name: "Not A Bootcamp",
      symbol: "NBC",
      description:
        "It's not a bootcamp. It's an elite Solana builders program (IYKYK)",
      image,
      attributes: [{ trait_type: "cute kitty", value: "100" }],
      properties: {
        files: [
          {
            type: "image/png",
            uri: image,
          },
        ],
      },
      creators: [],
    };
    const myUri = await umi.uploader.uploadJson(metadata);
    console.log("Your metadata URI: ", myUri); // https://gateway.irys.xyz/2BA3CVNRnG48uEG6UVsxq6vzLf8FQ7q6NRWQEKW5wGNf
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
