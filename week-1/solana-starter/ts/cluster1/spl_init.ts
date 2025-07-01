import { Connection, Commitment } from "@solana/web3.js";
import { createMint, getMint } from "@solana/spl-token";
import { getWallet } from "./wallet";

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
  try {
    // Start here
    const keypair = await getWallet();

    const mint = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      keypair.publicKey,
      6,
      undefined,
      { commitment }
    );

    const mintAccount = await getMint(connection, mint, commitment);
    console.log(mintAccount);
    // {
    //   address: PublicKey [PublicKey(fPcP9vGoowPikgu7oTRCJKHUvSNn9N5WZhYshR4UXyo)] {
    //     _bn: <BN: 9d5a2d05858ea871faab2a8b7035e3112e3c63b509f256c63838a66b228c57e>
    //   },
    //   mintAuthority: PublicKey [PublicKey(AH9JvTDAiQy2zAuFfzteNyUrW5DYoTsTLoeNjXrxTTSt)] {
    //     _bn: <BN: 89dbdecb5c0d93f4a43c089acf3cff9a78da8143c4753ac628ed53f50afb5125>
    //   },
    //   supply: 0n,
    //   decimals: 6,
    //   isInitialized: true,
    //   freezeAuthority: PublicKey [PublicKey(AH9JvTDAiQy2zAuFfzteNyUrW5DYoTsTLoeNjXrxTTSt)] {
    //     _bn: <BN: 89dbdecb5c0d93f4a43c089acf3cff9a78da8143c4753ac628ed53f50afb5125>
    //   },
    //   tlvData: <Buffer >
    // }
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
