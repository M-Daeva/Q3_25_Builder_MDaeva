use {
    crate::helpers::{execute_clmm_swap, unwrap_wsol},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    dex_adapter_cpi::{
        error::CustomError,
        state::{DaBump, DaConfig, Route, SEED_BUMP, SEED_CONFIG, SEED_ROUTE},
    },
};

#[derive(Accounts)]
pub struct SwapAndUnwrapWsol<'info> {
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
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config,
    )]
    pub config: Box<Account<'info, DaConfig>>,

    #[account(
        seeds = [SEED_ROUTE.as_bytes(), &input_token_mint.key().to_bytes(), &output_token_mint.key().to_bytes()],
        bump
    )]
    pub route: Box<Account<'info, Route>>,

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
}

impl<'info> SwapAndUnwrapWsol<'info> {
    pub fn swap_and_unwrap_wsol(
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
            config,
            input_token_mint,
            input_token_sender_ata,
            output_token_sender_ata,
            ..
        } = self;

        if amount_in == 0 {
            Err(CustomError::InvalidAmount)?;
        }

        // execute multihop swap on clmm_mock
        execute_clmm_swap(
            amount_in,
            amount_out_minimum,
            token_program,
            token_program_2022,
            memo_program,
            &config.dex,
            sender,
            input_token_mint,
            input_token_sender_ata,
            remaining_accounts,
        )?;

        // exhange wsol -> sol
        unwrap_wsol(token_program, sender, output_token_sender_ata)?;

        Ok(())
    }
}
