use {
    crate::helpers::{activate_account_on_registry, execute_clmm_swap},
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
pub struct SwapAndActivate<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: token_program_2022
    pub token_program_2022: UncheckedAccount<'info>,
    /// CHECK: memo_program
    pub memo_program: UncheckedAccount<'info>,
    /// CHECK: clmm_mock_program
    pub clmm_mock_program: UncheckedAccount<'info>,
    /// CHECK: registry_program
    pub registry_program: UncheckedAccount<'info>,

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

    #[account(
        seeds = [registry_cpi::state::SEED_BUMP.as_bytes()],
        bump,
        seeds::program = registry_program.key()
    )]
    pub registry_bump: Account<'info, registry_cpi::state::Bump>,

    #[account(
        seeds = [registry_cpi::state::SEED_CONFIG.as_bytes()],
        bump = registry_bump.config,
        seeds::program = registry_program.key()
    )]
    pub registry_config: Box<Account<'info, registry_cpi::state::Config>>,

    #[account(
        mut,
        seeds = [registry_cpi::state::SEED_USER_ID.as_bytes(), &sender.key().to_bytes()],
        bump,
        seeds::program = registry_program.key()
    )]
    pub registry_user_id: Box<Account<'info, registry_cpi::state::UserId>>,

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
        mut,
        associated_token::mint = output_token_mint,
        associated_token::authority = registry_config
    )]
    pub revenue_app_ata: Box<InterfaceAccount<'info, TokenAccount>>,
}

impl<'info> SwapAndActivate<'info> {
    pub fn swap_and_activate(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        amount_in: u64,
        amount_out_minimum: u64,
    ) -> Result<()> {
        let Self {
            system_program,
            token_program,
            associated_token_program,
            token_program_2022,
            memo_program,
            registry_program,
            sender,
            config,
            registry_bump,
            registry_config,
            registry_user_id,
            input_token_mint,
            output_token_mint,
            input_token_sender_ata,
            output_token_sender_ata,
            revenue_app_ata,
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

        // activate account on registry program
        activate_account_on_registry(
            sender.key,
            system_program,
            token_program,
            associated_token_program,
            registry_program,
            sender,
            registry_bump,
            registry_config,
            registry_user_id,
            output_token_mint,
            output_token_sender_ata,
            revenue_app_ata,
        )?;

        Ok(())
    }
}
