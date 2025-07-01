import { PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { getWallet } from "./wallet";

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("fPcP9vGoowPikgu7oTRCJKHUvSNn9N5WZhYshR4UXyo");

(async () => {
  try {
    const keypair = await getWallet();

    // Create an ATA
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey,
      false,
      commitment
    );
    console.log(`Your ata is: ${ata.address.toBase58()}`); // CeKxSzyeBojUuLVX5WnjhDHewDEzyX2oAGCSy2outzR1

    // Mint to ATA
    const mintTx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair.publicKey,
      token_decimals * 42n,
      undefined,
      { commitment }
    );
    console.log(`Your mint txid: ${mintTx}`); // 25aLvLbqqUhRzxL82ER9XEsUDbv5y3riQsNhYCzATru6EpBQWHpW61azcMKt1fuQy2stVzXvMwh1DEkJ7b1Ma6sM

    const balance = await connection.getTokenAccountBalance(ata.address);
    console.log({ balance: balance.value.uiAmount }); // 42
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
