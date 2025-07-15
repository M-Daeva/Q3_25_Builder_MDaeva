use anchor_lang::{
    error::{AnchorError, Error, ProgramErrorWithOrigin},
    prelude::ProgramError,
    Result,
};
use litesvm::LiteSVM;
use pretty_assertions::assert_eq;
use solana_keypair::Keypair;
use solana_kite::{
    create_associated_token_account, create_token_mint, deploy_program, get_pda_and_bump,
    get_token_account_balance, mint_tokens_to_account, send_transaction_from_instructions,
    SolanaKiteError,
};
use solana_program::{msg, native_token::LAMPORTS_PER_SOL};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use strum::IntoEnumIterator;

use crate::helpers::suite::{
    core::{App, WithTokenPubkey},
    types::{AppCoin, AppToken, AppUser, GetDecimals},
};

#[test]
fn default() -> Result<()> {
    let mut app = App::new();

    let bob_pyth_balance = app.get_balance(AppUser::Bob, AppToken::PYTH)?;
    let bob_sol_balance = app.get_balance(AppUser::Bob, AppCoin::SOL)?;

    assert_eq!(bob_pyth_balance, 1_000_000_000);
    assert_eq!(bob_sol_balance, 1_000_000_000_000);

    println!("amm_pk {:#?}", app.get_program_amm());

    // println!("{:#?}", AppUser::Admin);
    // println!("{:#?}", AppUser::Admin.pubkey());
    // println!("{:#?}\n", AppUser::Admin.keypair().pubkey());

    Ok(())
}
