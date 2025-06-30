import path from "path";
import { readFile, writeFile } from "fs/promises";

import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "../.test-wallets/solana/dev-keypair.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const MPL_CORE_PROGRAM_ID = new PublicKey(
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
);
const mintCollection = new PublicKey(
  "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"
);
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const l = console.log.bind(console);

// export function rootPath(dir: string): string {
//   return path.resolve(__dirname, "./", dir);
// }

// export async function readKeypair(keypairPath: string): Promise<Keypair> {
//   const secretKey = await readFile(keypairPath, {
//     encoding: "utf8",
//   }).then(JSON.parse);

//   return Keypair.fromSecretKey(new Uint8Array(secretKey));
// }

async function main() {
  // Create a Solana devnet connection
  const connection = new Connection("https://api.devnet.solana.com");

  // Create our anchor provider
  const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed",
  });

  // Create our program
  const program: Program<Turbin3Prereq> = new Program(IDL, provider);

  // Create the PDA for our enrollment account
  const account_seeds: Buffer[] = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
  ];
  const [accountPda, _account_bump] = PublicKey.findProgramAddressSync(
    account_seeds,
    program.programId
  );
  l({ accountPda });

  // // Execute the initialize transaction
  // try {
  //   const txhash = await program.methods
  //     .initialize("M-Daeva")
  //     .accountsPartial({
  //       user: keypair.publicKey,
  //       account: accountPda,
  //       system_program: SYSTEM_PROGRAM_ID,
  //     })
  //     .signers([keypair])
  //     .rpc();

  //   l(
  //     `Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
  //   );
  // } catch (e) {
  //   l(`Oops, something went wrong: ${e}`);
  // }

  // check acc
  const pda = await program.account.applicationAccount.fetch(accountPda);
  l(pda);

  const mintKeypair = Keypair.generate();

  // Create the PDA
  const authority: Buffer[] = [
    Buffer.from("collection"),
    mintCollection.toBuffer(),
  ];
  const [authorityPda] = PublicKey.findProgramAddressSync(
    authority,
    program.programId
  );
  l({ authorityPda });

  // submit
  try {
    const txhash = await program.methods
      .submitTs()
      .accountsPartial({
        user: keypair.publicKey,
        account: accountPda,
        mint: mintKeypair.publicKey,
        collection: mintCollection,
        authority: authorityPda,
        mpl_core_program: MPL_CORE_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
      })
      .signers([keypair, mintKeypair])
      .rpc();

    l(
      `Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
    );
  } catch (e) {
    l(`Oops, something went wrong: ${e}`);
  }
}

main();
