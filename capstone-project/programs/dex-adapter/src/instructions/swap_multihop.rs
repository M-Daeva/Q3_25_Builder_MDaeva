use {
    crate::helpers::execute_clmm_swap,
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::{transfer_token_from_program, transfer_token_from_user},
    dex_adapter_cpi::{
        error::CustomError,
        state::{DaBump, DaConfig, Route, SEED_BUMP, SEED_CONFIG, SEED_ROUTE},
    },
};

#[derive(Accounts)]
pub struct SwapMultihop<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: token_program_2022
    pub token_program_2022: UncheckedAccount<'info>,
    /// CHECK: memo_program
    pub memo_program: UncheckedAccount<'info>,
    /// CHECK: clmm_mock_program
    pub clmm_mock_program: UncheckedAccount<'info>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, DaBump>,

    #[account(
        mut,
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config,
    )]
    pub config: Account<'info, DaConfig>,

    #[account(
        seeds = [SEED_ROUTE.as_bytes(), &input_token_mint.key().to_bytes(), &output_token_mint.key().to_bytes()],
        bump
    )]
    pub route: Account<'info, Route>,

    // mint
    //
    #[account(mut)]
    pub input_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub output_token_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        mut,
        associated_token::mint = input_token_mint,
        associated_token::authority = sender
    )]
    pub input_token_sender_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = output_token_mint,
        associated_token::authority = sender
    )]
    pub output_token_sender_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = input_token_mint,
        associated_token::authority = config
    )]
    pub input_token_app_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = output_token_mint,
        associated_token::authority = config
    )]
    pub output_token_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> SwapMultihop<'info> {
    pub fn swap_multihop(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        amount_in: u64,
        amount_out_minimum: u64,
    ) -> Result<()> {
        let Self {
            token_program,
            token_program_2022,
            memo_program,
            sender,
            bump,
            config,
            input_token_mint,
            output_token_mint,
            input_token_sender_ata,
            output_token_sender_ata,
            input_token_app_ata,
            output_token_app_ata,
            ..
        } = self;

        if amount_in == 0 {
            Err(CustomError::InvalidAmount)?;
        }

        // store initial balances to track the swap
        let initial_output_user_balance = output_token_sender_ata.amount;
        let initial_output_app_balance = output_token_app_ata.amount;

        // transfer input tokens from sender to app ATA
        transfer_token_from_user(
            amount_in,
            input_token_mint,
            input_token_sender_ata,
            input_token_app_ata,
            sender,
            token_program,
        )?;

        // execute multihop swap on clmm_mock
        execute_clmm_swap(
            amount_in,
            amount_out_minimum,
            token_program,
            token_program_2022,
            memo_program,
            bump,
            config,
            input_token_mint,
            input_token_app_ata,
            remaining_accounts,
        )?;

        // reload accounts to get updated balances
        output_token_sender_ata.reload()?;
        output_token_app_ata.reload()?;

        // check both app ATA and user ATA for output tokens
        let app_balance_change = output_token_app_ata.amount - initial_output_app_balance;
        let user_balance_change = output_token_sender_ata.amount - initial_output_user_balance;

        // transfer any tokens from app ATA to user ATA
        if user_balance_change == 0 {
            Err(CustomError::NoOutputTokens)?;
        }

        transfer_token_from_program(
            app_balance_change,
            output_token_mint,
            output_token_app_ata,
            output_token_sender_ata,
            &[SEED_CONFIG.as_bytes()],
            bump.config,
            config,
            token_program,
        )?;

        Ok(())
    }
}
