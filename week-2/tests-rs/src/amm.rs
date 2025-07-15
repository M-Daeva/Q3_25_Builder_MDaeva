use anchor_lang::Result;
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_kite::{
    create_associated_token_account, create_token_mint, deploy_program, get_pda_and_bump,
    mint_tokens_to_account, send_transaction_from_instructions, SolanaKiteError,
};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_signer::Signer;
use strum::IntoEnumIterator;

use crate::helpers::suite::types::ProjectAccount;

#[test]
fn default() -> Result<()> {
    let mut litesvm = LiteSVM::new().with_spl_programs();

    //  let mint_x = create_token_mint(&mut litesvm, mint_authority, decimals).unwrap();

    // get SOL
    for user in ProjectAccount::iter() {
        litesvm
            .airdrop(
                &user.pubkey(),
                user.get_initial_sol_amount() * LAMPORTS_PER_SOL,
            )
            .unwrap();

        // let b = litesvm.get_balance(&user.pubkey()).unwrap_or_default();
        // println!("{:#?}", b / LAMPORTS_PER_SOL);
    }

    println!("{:#?}", ProjectAccount::Admin);
    println!("{:#?}", ProjectAccount::Admin.pubkey());
    println!("{:#?}\n", ProjectAccount::Admin.keypair().pubkey());

    Ok(())
}
