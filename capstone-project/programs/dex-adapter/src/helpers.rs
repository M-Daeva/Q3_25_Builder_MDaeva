use {
    crate::{
        error::CustomError,
        state::{Bump, Config, SEED_CONFIG},
        types::SwapRouterBaseInData,
    },
    anchor_lang::{prelude::*, solana_program},
    anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface},
};

pub fn execute_clmm_swap<'a>(
    bump: &Account<'a, Bump>,
    config: &Account<'a, Config>,
    input_token_app_ata: &InterfaceAccount<'a, TokenAccount>,
    input_token_mint: &InterfaceAccount<'a, Mint>,
    token_program: &Interface<'a, TokenInterface>,
    token_program_2022: &UncheckedAccount<'a>,
    memo_program: &UncheckedAccount<'a>,
    remaining_accounts: &'a [AccountInfo<'a>],
    amount_in: u64,
    amount_out_minimum: u64,
) -> Result<()> {
    // validate that remaining accounts length is correct (multiple of 7)
    if remaining_accounts.len() % 7 != 0 {
        Err(CustomError::InvalidRemainingAccounts)?;
    }

    let account_infos = [
        &[
            config.to_account_info(),
            input_token_app_ata.to_account_info(),
            input_token_mint.to_account_info(),
            token_program.to_account_info(),
            token_program_2022.to_account_info(),
            memo_program.to_account_info(),
        ],
        remaining_accounts,
    ]
    .concat();

    // build accounts for CPI call to clmm_mock - match exact structure from clmm-mock
    let mut accounts = vec![
        AccountMeta::new(config.key(), true), // payer (signer)
        AccountMeta::new(input_token_app_ata.key(), false), // input_token_account (writable)
        AccountMeta::new(input_token_mint.key(), false), // input_token_mint (writable) ← FIXED
        AccountMeta::new_readonly(token_program.key(), false), // token_program
        AccountMeta::new_readonly(token_program_2022.key(), false), // token_program_2022
        AccountMeta::new_readonly(memo_program.key(), false), // memo_program
    ];

    // process remaining accounts in groups of 7
    for chunk in remaining_accounts.chunks_exact(7) {
        accounts.extend(vec![
            AccountMeta::new_readonly(chunk[0].key(), false), // amm_config (readonly)
            AccountMeta::new(chunk[1].key(), false),          // pool_state (writable)
            AccountMeta::new(chunk[2].key(), false),          // output_token_account (writable)
            AccountMeta::new(chunk[3].key(), false),          // input_vault (writable)
            AccountMeta::new(chunk[4].key(), false),          // output_vault (writable)
            AccountMeta::new_readonly(chunk[5].key(), false), // output_mint (readonly)
            AccountMeta::new(chunk[6].key(), false),          // observation_state (writable)
        ]);
    }

    // prepare instruction data
    let instruction_data = SwapRouterBaseInData {
        discriminator: get_discriminator("swap_router_base_in"),
        amount_in,
        amount_out_minimum,
    };

    let instruction = solana_program::instruction::Instruction {
        program_id: config.dex,
        accounts,
        data: instruction_data.try_to_vec()?,
    };

    // create signer seeds for config PDA
    let config_seeds = &[SEED_CONFIG.as_bytes(), &[bump.config]];
    let signer_seeds = &[&config_seeds[..]];

    // execute CPI call with config as signer
    anchor_lang::solana_program::program::invoke_signed(
        &instruction,
        &account_infos,
        signer_seeds,
    )?;

    Ok(())
}

fn get_discriminator(instruction_name: &str) -> [u8; 8] {
    let mut discriminator = [0u8; 8];
    let hash = solana_program::hash::hash(format!("global:{}", instruction_name).as_bytes());
    discriminator.copy_from_slice(&hash.to_bytes()[..8]);
    discriminator
}
