use anchor_lang::AnchorDeserialize;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use anchor_lang::{
    error::{AnchorError, Error, ProgramErrorWithOrigin},
    prelude::ProgramError,
    Id, Result,
};
use anchor_spl::associated_token::AssociatedToken;
use litesvm::LiteSVM;
use pretty_assertions::assert_eq;
use solana_instruction::AccountMeta;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_kite::{
    create_associated_token_account, create_token_mint, deploy_program, get_pda_and_bump,
    get_token_account_balance, mint_tokens_to_account, seeds, send_transaction_from_instructions,
    SolanaKiteError,
};
use solana_program::{msg, native_token::LAMPORTS_PER_SOL, system_program};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use strum::IntoEnumIterator;

use crate::helpers::suite::{
    core::{App, WithTokenKeys},
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

    // ----------------------------------------------------------

    let program_id = amm::ID;

    let pool_creator_keypair = AppUser::Admin.keypair();
    let mint_x_keypair = AppToken::USDC.keypair(&app);
    let mint_y_keypair = AppToken::PYTH.keypair(&app);

    let pool_creator = pool_creator_keypair.pubkey();
    let mint_x = mint_x_keypair.pubkey();
    let mint_y = mint_y_keypair.pubkey();

    // Pool parameters
    let id: u64 = 1; // pool_id
    let fee_bps: u16 = 300; // 3%

    // Derive PDAs
    let (pool_config, _) = get_pda_and_bump(&seeds!["config", id], &program_id);
    let (pool_balance, _) = get_pda_and_bump(&seeds!["balance", id], &program_id);
    let (mint_lp, _) = get_pda_and_bump(&seeds!["lp", id], &program_id);

    // Derive ATAs
    let liquidity_pool_mint_lp_ata = get_associated_token_address(&pool_config, &mint_lp);
    let liquidity_pool_mint_x_ata = get_associated_token_address(&pool_config, &mint_x);
    let liquidity_pool_mint_y_ata = get_associated_token_address(&pool_config, &mint_y);

    // Create instruction data
    let instruction_data = amm::instruction::CreatePool {
        id,
        mint_x,
        mint_y,
        fee_bps,
    };

    // Build accounts for the instruction
    let accounts = amm::accounts::CreatePool {
        system_program: system_program::ID,
        token_program: spl_token::ID,
        associated_token_program: AssociatedToken::id(),
        pool_creator,
        pool_config,
        pool_balance,
        mint_lp,
        mint_x,
        mint_y,
        liquidity_pool_mint_lp_ata,
        liquidity_pool_mint_x_ata,
        liquidity_pool_mint_y_ata,
    };

    // Create the instruction
    let ix = Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: instruction_data.data(),
    };

    // Create and send transaction
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&pool_creator),
        &[&pool_creator_keypair],
        app.litesvm.latest_blockhash(),
    );

    // Execute the transaction
    let res = app.litesvm.send_transaction(transaction).unwrap();

    // Verify the pool config account was created correctly
    // Deserialize and verify pool config data
    let mut pool_config_data = &app.litesvm.get_account(&pool_config).unwrap().data[8..]; // Skip discriminator
    let pool_config_struct = amm::state::PoolConfig::deserialize(&mut pool_config_data).unwrap();

    assert_eq!(
        pool_config_struct,
        amm::state::PoolConfig {
            config_bump: 251,
            balance_bump: 255,
            lp_bump: 253,
            id: 1,
            authority: Some(pool_creator),
            mint_x,
            mint_y,
            mint_lp,
            fee_bps: 300,
            is_locked: false,
        }
    );

    println!("{:#?}", res.compute_units_consumed);
    println!("{:#?}", res.logs);
    println!("{:#?}", &pool_config_struct);

    Ok(())
}
