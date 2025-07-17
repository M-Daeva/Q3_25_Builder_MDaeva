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
use strum::IntoEnumIterator;

use crate::helpers::suite::{
    core::{App, WithTokenKeys},
    types::{AppCoin, AppToken, AppUser, GetDecimals},
};

pub struct MakeOfferAccounts {
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub maker_token_account_a: Pubkey,
    pub offer_account: Pubkey,
    pub vault: Pubkey,
}

#[test]
fn default() -> Result<()> {
    let mut app = App::new();

    let bob_pyth_balance = app.get_balance(AppUser::Bob, AppToken::PYTH)?;
    let bob_sol_balance = app.get_balance(AppUser::Bob, AppCoin::SOL)?;

    assert_eq!(bob_pyth_balance, 1_000_000_000);
    assert_eq!(bob_sol_balance, 1_000_000_000_000);

    println!("amm_pk {:#?}", app.get_program_amm());

    // ----------------------------------------------------------

    // let offer_id = 0;
    // let program_id = Pubkey::from_str_const("CpuYGzAZWKWBHXUoBSfEg3qnvRd8pMcRa9XV29Xoj3KU");

    // let (offer_account, _offer_bump) = get_pda_and_bump(&seeds!["offer", offer_id], &program_id);
    // let vault = spl_associated_token_account::get_associated_token_address(
    //     &offer_account,
    //     &AppToken::USDC.pubkey(&app),
    // );

    // let make_offer_accounts = MakeOfferAccounts {
    //     associated_token_program: spl_associated_token_account::ID,
    //     token_program: spl_token::ID,
    //     system_program: anchor_lang::system_program::ID,
    //     maker: AppUser::Alice.pubkey(),
    //     token_mint_a: AppToken::USDC.pubkey(&app),
    //     token_mint_b: AppToken::PYTH.pubkey(&app),
    //     maker_token_account_a: spl_associated_token_account::get_associated_token_address(
    //         &AppUser::Alice.pubkey(),
    //         &AppToken::USDC.pubkey(&app),
    //     ),
    //     offer_account,
    //     vault,
    // };

    // let mut instruction_data =
    //     anchor_lang::solana_program::hash::hash(b"global:make_offer").to_bytes()[..8].to_vec();

    // instruction_data.extend_from_slice(&offer_id.to_le_bytes());

    // let make_offer_instruction = Instruction::new_with_bytes(
    //     program_id,
    //     &instruction_data,
    //     vec![
    //         AccountMeta::new_readonly(make_offer_accounts.associated_token_program, false),
    //         AccountMeta::new(make_offer_accounts.maker, true),
    //     ],
    // );

    // let result = send_transaction_from_instructions(
    //     &mut app.litesvm,
    //     vec![make_offer_instruction],
    //     &[&AppUser::Alice.keypair()],
    //     &AppUser::Alice.pubkey(),
    // )
    // .unwrap();

    // ----------------------------------------------------------

    let pool_creator = AppUser::Admin.keypair();
    let mint_x_keypair = AppToken::USDC.keypair(&app);
    let mint_y_keypair = AppToken::PYTH.keypair(&app);

    let mint_x = mint_x_keypair.pubkey();
    let mint_y = mint_y_keypair.pubkey();

    // Pool parameters
    let pool_id: u64 = 1;
    let fee_bps: u16 = 300; // 3%

    // Derive PDAs
    let (pool_config, _config_bump) =
        Pubkey::find_program_address(&[b"config", pool_id.to_le_bytes().as_ref()], &amm::ID);

    let (pool_balance, _balance_bump) =
        Pubkey::find_program_address(&[b"balance", pool_id.to_le_bytes().as_ref()], &amm::ID);

    let (mint_lp, _lp_bump) =
        Pubkey::find_program_address(&[b"lp", pool_id.to_le_bytes().as_ref()], &amm::ID);

    // let (mint_lp, _lp_bump) = get_pda_and_bump(&seeds!["lp", pool_id], &amm::ID);

    // Derive ATAs
    let liquidity_pool_mint_lp_ata =
        spl_associated_token_account::get_associated_token_address(&pool_config, &mint_lp);

    let liquidity_pool_mint_x_ata =
        spl_associated_token_account::get_associated_token_address(&pool_config, &mint_x);

    let liquidity_pool_mint_y_ata =
        spl_associated_token_account::get_associated_token_address(&pool_config, &mint_y);

    // Create instruction data
    let instruction_data = amm::instruction::CreatePool {
        id: pool_id,
        mint_x,
        mint_y,
        fee_bps,
    };

    // Build accounts for the instruction
    let accounts = amm::accounts::CreatePool {
        system_program: system_program::ID,
        token_program: spl_token::ID,
        associated_token_program: AssociatedToken::id(),
        pool_creator: pool_creator.pubkey(),
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
        program_id: amm::ID,
        accounts: accounts.to_account_metas(None),
        data: instruction_data.data(),
    };

    // Create and send transaction
    let recent_blockhash = app.litesvm.latest_blockhash();
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&pool_creator.pubkey()),
        &[&pool_creator],
        recent_blockhash,
    );

    // Execute the transaction
    let _result = app.litesvm.send_transaction(transaction).unwrap();

    // Verify the pool config account was created correctly
    let pool_config_account = app.litesvm.get_account(&pool_config).unwrap();
    assert!(!pool_config_account.data.is_empty());

    // Deserialize and verify pool config data
    let mut pool_config_data = &pool_config_account.data[8..]; // Skip discriminator
    let pool_config_struct = amm::state::PoolConfig::deserialize(&mut pool_config_data).unwrap();

    assert_eq!(
        pool_config_struct,
        amm::state::PoolConfig {
            config_bump: 251,
            balance_bump: 255,
            lp_bump: 253,
            id: 1,
            authority: Some(pool_creator.pubkey()),
            mint_x,
            mint_y,
            mint_lp,
            fee_bps: 300,
            is_locked: false,
        }
    );

    // Verify the pool balance account was created
    let pool_balance_account = app.litesvm.get_account(&pool_balance).unwrap();
    assert!(!pool_balance_account.data.is_empty());

    // Verify the LP mint was created
    let mint_lp_account = app.litesvm.get_account(&mint_lp).unwrap();
    assert!(!mint_lp_account.data.is_empty());

    // Verify ATAs were created
    let lp_ata_account = app
        .litesvm
        .get_account(&liquidity_pool_mint_lp_ata)
        .unwrap();
    assert!(!lp_ata_account.data.is_empty());

    let x_ata_account = app.litesvm.get_account(&liquidity_pool_mint_x_ata).unwrap();
    assert!(!x_ata_account.data.is_empty());

    let y_ata_account = app.litesvm.get_account(&liquidity_pool_mint_y_ata).unwrap();
    assert!(!y_ata_account.data.is_empty());

    println!("{:#?}", &pool_config_account);
    println!("{:#?}", &pool_config_struct);

    Ok(())
}
