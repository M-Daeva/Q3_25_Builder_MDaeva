import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { getWallet } from "./wallet";

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
  try {
    // Start here
    const keypair = await getWallet();

    // const mint = ???
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
