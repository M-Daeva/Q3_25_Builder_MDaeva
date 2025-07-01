import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { getWallet } from "./wallet";
import { Commitment, Connection, PublicKey } from "@solana/web3.js";

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("fPcP9vGoowPikgu7oTRCJKHUvSNn9N5WZhYshR4UXyo");

// Recipient address
const to = new PublicKey("7y7UhiE6wZ9FsjMgsGRqLygYQja37U5BTbPtPThyHf8s");

(async () => {
  try {
    const keypair = await getWallet();

    // Get the token account of the fromWallet address, and if it does not exist, create it
    const ataFrom = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey,
      false,
      commitment
    );
    console.log(`Your ataFrom is: ${ataFrom.address.toBase58()}`); // CeKxSzyeBojUuLVX5WnjhDHewDEzyX2oAGCSy2outzR1

    // Get the token account of the toWallet address, and if it does not exist, create it
    const ataTo = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to,
      false,
      commitment
    );
    console.log(`Your ataTo is: ${ataTo.address.toBase58()}`); // 5LCx6Wwgj7mygs2DyTCWw7JGvdRzMpeTbfrgKr4eRFpy

    // Transfer the new token to the "toTokenAccount" we just created
    const sig = await transfer(
      connection,
      keypair,
      ataFrom.address,
      ataTo.address,
      keypair.publicKey,
      token_decimals * 2n,
      undefined,
      { commitment }
    );
    console.log({ sig }); // 2nTe7FgBnWeybDVgTTyTDeCbh73mgZMs8rXDC18fH6LS6qKa5AkCJpgNqReCq9Nk7gHXq3g1L9EEh6w7fHLPAWC

    const balanceTo = await connection.getTokenAccountBalance(ataTo.address);
    console.log({ balanceTo: balanceTo.value.uiAmount }); // 2
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
