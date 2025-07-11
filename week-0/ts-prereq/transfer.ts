import wallet from "./dev-wallet.json";
import {
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

const l = console.log.bind(console);

async function main() {
  // Import our dev wallet keypair from the wallet file
  const from = Keypair.fromSecretKey(new Uint8Array(wallet));
  // Define our Turbin3 public key
  const to = new PublicKey("AH9JvTDAiQy2zAuFfzteNyUrW5DYoTsTLoeNjXrxTTSt");

  // Create a Solana devnet connection
  const connection = new Connection("https://api.devnet.solana.com");

  // (async () => {
  //   try {
  //     const transaction = new Transaction().add(
  //       SystemProgram.transfer({
  //         fromPubkey: from.publicKey,
  //         toPubkey: to,
  //         lamports: LAMPORTS_PER_SOL / 100,
  //       })
  //     );
  //     transaction.recentBlockhash = (
  //       await connection.getLatestBlockhash("confirmed")
  //     ).blockhash;
  //     transaction.feePayer = from.publicKey;

  //     // Sign transaction, broadcast, and confirm
  //     const signature = await sendAndConfirmTransaction(
  //       connection,
  //       transaction,
  //       [from]
  //     );

  //     l(
  //       `Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`
  //     );
  //   } catch (e) {
  //     l(`Oops, something went wrong: ${e}`);
  //   }
  // })();

  (async () => {
    try {
      // Get balance of dev wallet
      const balance = await connection.getBalance(from.publicKey);
      // Create a test transaction to calculate fees
      const transaction = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance,
        })
      );
      transaction.recentBlockhash = (
        await connection.getLatestBlockhash("confirmed")
      ).blockhash;
      transaction.feePayer = from.publicKey;
      // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
      const fee =
        (
          await connection.getFeeForMessage(
            transaction.compileMessage(),
            "confirmed"
          )
        ).value || 0;
      // Remove our transfer instruction to replace it
      transaction.instructions.pop();
      // Now add the instruction back with correct amount of lamports
      transaction.add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance - fee,
        })
      );

      // Sign transaction, broadcast, and confirm
      const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [from]
      );

      l(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`
      );
    } catch (e) {
      l(`Oops, something went wrong: ${e}`);
    }
  })();
}

main();
