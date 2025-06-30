#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::{client_error::Result, rpc_client::RpcClient};
    use solana_program::system_instruction::transfer;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
        transaction::Transaction,
    };
    use std::{
        io::{self, BufRead},
        str::FromStr,
    };

    const RPC_URL: &str =
        "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    // const RPC_URL: &str = "https://api.devnet.solana.com";

    // #[test]
    // fn keygen() {
    //     // Create a new keypair
    //     let kp = Keypair::new();
    //     println!(
    //         "You've generated a new Solana wallet: {}",
    //         kp.pubkey().to_string()
    //     );
    //     println!("");
    //     println!("To save your wallet, copy and paste the following into a JSON file:");
    //     println!("{:?}", kp.to_bytes());
    // }

    // // vec -> str
    // #[test]
    // fn wallet_to_base58() {
    //     // println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
    //     // let stdin = io::stdin();
    //     let wallet: Vec<u8> = [
    //     ]
    //     .to_vec();
    //     println!("Your Base58-encoded private key is:");
    //     let base58 = bs58::encode(wallet).into_string();
    //     println!("{:?}", base58);
    // }

    // // str -> vec
    // #[test]
    // fn base58_to_wallet() {
    //     // println!("Input your private key as a base58 string:");
    //     // let stdin = io::stdin();
    //     let base58: &str = "";
    //     println!("Your wallet file format is:");
    //     let wallet = bs58::decode(base58).into_vec().unwrap();
    //     println!("{:?}", wallet);
    // }

    // #[test]
    // fn airdrop() -> Result<()> {
    //     // Import our keypair
    //     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    //     println!("keypair {:#?}", keypair.pubkey());

    //     // we'll establish a connection to Solana devnet using the const we defined above
    //     let client = RpcClient::new(RPC_URL);

    //     let bal = client.get_balance(&keypair.pubkey())?;
    //     println!("bal {:#?}", bal);

    //     // // We're going to claim 2 devnet SOL tokens (2 billion lamports)
    //     // match client.request_airdrop(&keypair.pubkey(), 2_000_000_000_u64) {
    //     //     Ok(sig) => {
    //     //         println!("Success! Check your TX here:");
    //     //         println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
    //     //     }
    //     //     Err(err) => {
    //     //         println!("Airdrop failed: {}", err);
    //     //     }
    //     // }

    //     // let bal = client.get_balance(&keypair.pubkey())?;
    //     // println!("bal {:#?}", bal);

    //     Ok(())
    // }

    // #[test]
    // fn transfer_sol() -> Result<()> {
    //     // Import our keypair
    //     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    //     println!("keypair {:#?}", keypair.pubkey());

    //     // we'll establish a connection to Solana devnet using the const we defined above
    //     let rpc_client = RpcClient::new(RPC_URL);

    //     let bal = rpc_client.get_balance(&keypair.pubkey())?;
    //     println!("bal {:#?}", bal);

    //     // Generate a signature from the keypair
    //     let pubkey = keypair.pubkey();
    //     let message_bytes = b"I verify my Solana Keypair!";
    //     let sig = keypair.sign_message(message_bytes);

    //     // Verify the signature using the public key
    //     match sig.verify(&pubkey.to_bytes(), message_bytes) {
    //         true => println!("Signature verified"),
    //         false => println!("Verification failed"),
    //     }

    //     let to_pubkey = Pubkey::from_str("AH9JvTDAiQy2zAuFfzteNyUrW5DYoTsTLoeNjXrxTTSt").unwrap();

    //     let recent_blockhash = rpc_client
    //         .get_latest_blockhash()
    //         .expect("Failed to get recent blockhash");

    //     println!("block {:#?}", recent_blockhash);

    //     let transaction = Transaction::new_signed_with_payer(
    //         &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
    //         Some(&keypair.pubkey()),
    //         &vec![&keypair],
    //         recent_blockhash,
    //     );

    //     let signature = rpc_client
    //         .send_and_confirm_transaction(&transaction)
    //         .expect("Failed to send transaction");
    //     println!(
    //         "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
    //         signature
    //     );

    //     let bal = rpc_client.get_balance(&keypair.pubkey())?;
    //     println!("bal {:#?}", bal);

    //     Ok(())
    // }

    // #[test]
    // fn transfer_sol_2() -> Result<()> {
    //     // Import our keypair
    //     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    //     // we'll establish a connection to Solana devnet using the const we defined above
    //     let rpc_client = RpcClient::new(RPC_URL);

    //     let balance = rpc_client.get_balance(&keypair.pubkey())?;
    //     println!("bal {:#?}", balance);

    //     let to_pubkey = Pubkey::from_str("AH9JvTDAiQy2zAuFfzteNyUrW5DYoTsTLoeNjXrxTTSt").unwrap();
    //     let recent_blockhash = rpc_client
    //         .get_latest_blockhash()
    //         .expect("Failed to get recent blockhash");

    //     let message = Message::new_with_blockhash(
    //         &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
    //         Some(&keypair.pubkey()),
    //         &recent_blockhash,
    //     );

    //     let fee = rpc_client
    //         .get_fee_for_message(&message)
    //         .expect("Failed to get fee calculator");

    //     let transaction = Transaction::new_signed_with_payer(
    //         &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
    //         Some(&keypair.pubkey()),
    //         &vec![&keypair],
    //         recent_blockhash,
    //     );

    //     let signature = rpc_client
    //         .send_and_confirm_transaction(&transaction)
    //         .expect("Failed to send transaction");
    //     println!(
    //         "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
    //         signature
    //     );

    //     let bal = rpc_client.get_balance(&keypair.pubkey())?;
    //     println!("bal {:#?}", bal);

    //     Ok(())
    // }

    #[test]
    fn enroll() -> Result<()> {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("../.test-wallets/solana/dev-keypair.json")
            .expect("Couldn't find wallet file");

        let system_program_id = system_program::id();
        let turbin3_prereq_program =
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let mpl_core_program_id =
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let mint_collection =
            Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

        let mint_keypair = Keypair::new();

        let (account_pda, ..) = Pubkey::find_program_address(
            &[b"prereqs", signer.pubkey().as_ref()],
            &turbin3_prereq_program,
        );
        let (authority_pda, ..) = Pubkey::find_program_address(
            &[b"collection", mint_collection.as_ref()],
            &turbin3_prereq_program,
        );

        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),       // user signer
            AccountMeta::new(account_pda, false),          // PDA account
            AccountMeta::new(mint_keypair.pubkey(), true), // mint keypair
            AccountMeta::new(mint_collection, false),      // collection
            AccountMeta::new_readonly(authority_pda, false), // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program_id, false), // mpl core program
            AccountMeta::new_readonly(system_program_id, false), // system program
        ];

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint_keypair],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        Ok(())
    }
}
