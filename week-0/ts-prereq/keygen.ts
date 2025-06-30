import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";
import pk from "./dev-wallet.json";
// import prompt from "prompt-sync";
// prompt()("Enter your string:").trim();

const l = console.log.bind(console);

async function main() {
  // Generate a new keypair
  let kp = Keypair.generate();
  console.log(
    `You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`
  );

  console.log(`[${kp.secretKey}]`);
}

function base58ToWallet() {
  const base58 = "";
  const wallet = bs58.decode(base58);
  l({ base58, wallet });
}

function walletToBase58() {
  // [...]
  const wallet = pk;
  const base58 = bs58.encode(wallet);
  l({ wallet, base58 });
}

// main();

// base58ToWallet();
walletToBase58();
