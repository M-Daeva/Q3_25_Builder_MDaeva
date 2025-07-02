import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { getWallet } from "./wallet";
import base58 from "bs58";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
} from "@metaplex-foundation/umi";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

const mint = generateSigner(umi);

(async () => {
  const { secretKey } = await getWallet();
  const keypair = umi.eddsa.createKeypairFromSecretKey(secretKey);
  const myKeypairSigner = createSignerFromKeypair(umi, keypair);
  umi.use(signerIdentity(myKeypairSigner));
  umi.use(mplTokenMetadata());

  const tx = createNft(umi, {
    mint,
    name: "Not A Bootcamp",
    symbol: "NBC",
    uri: "https://gateway.irys.xyz/2BA3CVNRnG48uEG6UVsxq6vzLf8FQ7q6NRWQEKW5wGNf",
    updateAuthority: umi.identity.publicKey,
    sellerFeeBasisPoints: percentAmount(4.2),
  });

  const result = await tx.sendAndConfirm(umi);
  const signature = base58.encode(result.signature);

  console.log(
    `Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  ); // https://explorer.solana.com/tx/5wPAtpj1X9PZsxuaUR6JyZ6NfR6C1A8SAF6wLn7boGavpURDpLRTdXwz2uAViBqF18mR7WbTsiqvrZaSne9TFHrd?cluster=devnet

  console.log("Mint Address: ", mint.publicKey); // EjMwLrhrzgu1uUEdjZiZ4ybppDkXUWNJ3enE7gtSG9vu
})();
