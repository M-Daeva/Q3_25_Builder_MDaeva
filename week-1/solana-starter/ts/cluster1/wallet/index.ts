import { Keypair } from "@solana/web3.js";
import { readFile } from "fs/promises";

export async function getWallet(): Promise<Keypair> {
  const keypairPath = "../../../../.test-wallets/solana/dev-keypair.json";
  const secretKey = await readFile(keypairPath, {
    encoding: "utf-8",
  }).then(JSON.parse);

  return Keypair.fromSecretKey(new Uint8Array(secretKey));
}
