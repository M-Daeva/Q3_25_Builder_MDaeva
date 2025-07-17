use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use anchor_lang::{Id, Result};
use anchor_spl::associated_token::AssociatedToken;
use pretty_assertions::assert_eq;
use solana_instruction::Instruction;
use solana_program::system_program;
use solana_signer::Signer;
use solana_transaction::Transaction;

use amm::state::PoolConfig;

use crate::helpers::{
    amm::AppExtension,
    suite::{
        core::{App, WithTokenKeys},
        types::{AppCoin, AppToken, AppUser},
    },
};

#[test]
fn default() -> Result<()> {
    let mut app = App::new();

    let bob_pyth_balance = app.get_balance(AppUser::Bob, AppToken::PYTH)?;
    let bob_sol_balance = app.get_balance(AppUser::Bob, AppCoin::SOL)?;

    assert_eq!(bob_pyth_balance, 1_000_000_000);
    assert_eq!(bob_sol_balance, 1_000_000_000_000);

    // ----------------------------------------------------------

    let program_id = app.program_id.amm;

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
    let pool_config = app.pda.amm_pool_config(id);
    let pool_balance = app.pda.amm_pool_balance(id);
    let mint_lp = app.pda.amm_mint_lp(id);

    // Derive ATAs
    let liquidity_pool_mint_lp_ata = App::get_ata(&pool_config, &mint_lp);
    let liquidity_pool_mint_x_ata = App::get_ata(&pool_config, &mint_x);
    let liquidity_pool_mint_y_ata = App::get_ata(&pool_config, &mint_y);

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
    let pool_config = app.amm_query_pool_config(id)?;

    assert_eq!(
        pool_config,
        PoolConfig {
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
    println!("{:#?}", &pool_config);

    Ok(())
}
