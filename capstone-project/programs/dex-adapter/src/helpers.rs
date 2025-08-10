use {
    anchor_lang::{prelude::*, solana_program},
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
    },
    base::helpers::get_discriminator,
    dex_adapter_cpi::{error::CustomError, types::SwapRouterBaseInData},
};

pub fn execute_clmm_swap<'a>(
    amount_in: u64,
    amount_out_minimum: u64,
    token_program: &Interface<'a, TokenInterface>,
    token_program_2022: &UncheckedAccount<'a>,
    memo_program: &UncheckedAccount<'a>,
    dex_program_id: &Pubkey,
    sender: &Signer<'a>,
    input_token_mint: &InterfaceAccount<'a, Mint>,
    input_token_sender_ata: &InterfaceAccount<'a, TokenAccount>,
    remaining_accounts: &'a [AccountInfo<'a>],
) -> Result<()> {
    // validate that remaining accounts length is correct (multiple of 7)
    if remaining_accounts.len() % 7 != 0 {
        Err(CustomError::InvalidRemainingAccounts)?;
    }

    let account_infos = [
        &[
            sender.to_account_info(),
            input_token_sender_ata.to_account_info(),
            input_token_mint.to_account_info(),
            token_program.to_account_info(),
            token_program_2022.to_account_info(),
            memo_program.to_account_info(),
        ],
        remaining_accounts,
    ]
    .concat();

    // build accounts for CPI call to clmm_mock
    let mut accounts = vec![
        AccountMeta::new(sender.key(), true), // payer (signer)
        AccountMeta::new(input_token_sender_ata.key(), false), // input_token_account (writable)
        AccountMeta::new(input_token_mint.key(), false), // input_token_mint (writable)
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
        program_id: *dex_program_id,
        accounts,
        data: instruction_data.try_to_vec()?,
    };

    // execute CPI call with user as signer
    Ok(solana_program::program::invoke(
        &instruction,
        &account_infos,
    )?)
}

pub fn activate_account_on_registry<'a>(
    user_to_activate: &Pubkey,
    system_program: &Program<'a, System>,
    token_program: &Interface<'a, TokenInterface>,
    associated_token_program: &Program<'a, AssociatedToken>,
    registry_program: &UncheckedAccount<'a>,
    sender: &Signer<'a>,
    registry_bump: &Account<'a, registry_cpi::state::Bump>,
    registry_config: &Account<'a, registry_cpi::state::Config>,
    registry_user_id: &Account<'a, registry_cpi::state::UserId>,
    output_token_mint: &InterfaceAccount<'a, Mint>,
    output_token_sender_ata: &InterfaceAccount<'a, TokenAccount>,
    revenue_app_ata: &InterfaceAccount<'a, TokenAccount>,
) -> Result<()> {
    let cpi_accounts = registry::cpi::accounts::ActivateAccount {
        system_program: system_program.to_account_info(),
        token_program: token_program.to_account_info(),
        associated_token_program: associated_token_program.to_account_info(),
        sender: sender.to_account_info(),
        bump: registry_bump.to_account_info(),
        config: registry_config.to_account_info(),
        user_id: registry_user_id.to_account_info(),
        revenue_mint: output_token_mint.to_account_info(),
        revenue_sender_ata: output_token_sender_ata.to_account_info(),
        revenue_app_ata: revenue_app_ata.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(registry_program.to_account_info(), cpi_accounts);

    registry::cpi::activate_account(cpi_ctx, *user_to_activate)
}

pub fn unwrap_wsol<'a>(
    token_program: &Interface<'a, TokenInterface>,
    sender: &Signer<'a>,
    output_token_sender_ata: &InterfaceAccount<'a, TokenAccount>,
) -> Result<()> {
    let cpi_accounts = CloseAccount {
        account: output_token_sender_ata.to_account_info(),
        destination: sender.to_account_info(),
        authority: sender.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);

    close_account(cpi_ctx)
}
