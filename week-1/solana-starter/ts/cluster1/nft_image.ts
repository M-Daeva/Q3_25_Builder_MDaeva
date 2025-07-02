import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";
import { getWallet } from "./wallet";
import {
  createGenericFile,
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

    //1. Load image
    //2. Convert image to generic file.
    //3. Upload image

    const filePath = "./cluster1/img/nbc.jpg";
    const buffer = await readFile(filePath);
    const image = createGenericFile(buffer, filePath, {
      contentType: "image/jpeg",
    });

    const [myUri] = await umi.uploader.upload([image]);
    console.log("Your image URI: ", myUri); // https://gateway.irys.xyz/DB6vMrkFNqapPT488Hqvi2cQFJAnMc1QYr11Ttw5fpfA
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
