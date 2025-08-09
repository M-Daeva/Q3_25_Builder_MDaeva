use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    dex_adapter_cpi::state::{Bump, Config, Route, SEED_BUMP, SEED_CONFIG, SEED_ROUTE},
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

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        mut,
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config,
    )]
    pub config: Account<'info, Config>,

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

    // Registry program accounts for CPI
    //
    /// CHECK: registry program ID
    pub registry_program: UncheckedAccount<'info>,

    #[account(
        seeds = [b"bump"], // Registry's SEED_BUMP
        bump,
        seeds::program = registry_program.key()
    )]
    /// CHECK: Registry bump account
    pub registry_bump: UncheckedAccount<'info>,

    #[account(
        seeds = [b"config"], // Registry's SEED_CONFIG  
        bump,
        seeds::program = registry_program.key()
    )]
    /// CHECK: Registry config account
    pub registry_config: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"user_id", &sender.key().to_bytes()], // Registry's SEED_USER_ID
        bump,
        seeds::program = registry_program.key()
    )]
    /// CHECK: Registry user_id account
    pub registry_user_id: UncheckedAccount<'info>,

    // Registry token accounts
    //
    pub revenue_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = revenue_mint,
        associated_token::authority = config // Changed from sender to config
    )]
    pub revenue_sender_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = revenue_mint,
        associated_token::authority = registry_config
    )]
    pub revenue_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> SwapAndActivate<'info> {
    pub fn swap_and_activate(&mut self, _amount_in: u64, _amount_out_minimum: u64) -> Result<()> {
        // if amount_in == 0 {
        //     Err(CustomError::InvalidAmount)?;
        // }

        // // store initial balances to track the swap
        // let initial_output_user_balance = self.output_token_sender_ata.amount;
        // let initial_output_app_balance = self.output_token_app_ata.amount;

        // // transfer input tokens from sender to app ATA
        // transfer_token_from_user(
        //     amount_in,
        //     &self.input_token_mint,
        //     &self.input_token_sender_ata,
        //     &self.input_token_app_ata,
        //     &self.sender,
        //     &self.token_program,
        // )?;

        // // execute multihop swap on clmm_mock
        // execute_clmm_swap(
        //     &self.bump,
        //     &self.config,
        //     &self.input_token_app_ata,
        //     &self.input_token_mint,
        //     &self.token_program,
        //     &self.token_program_2022,
        //     &self.memo_program,
        //     remaining_accounts,
        //     amount_in,
        //     amount_out_minimum,
        // )?;

        // // reload accounts to get updated balances
        // self.output_token_sender_ata.reload()?;
        // self.output_token_app_ata.reload()?;

        // // check both app ATA and user ATA for output tokens
        // let app_balance_change = self.output_token_app_ata.amount - initial_output_app_balance;
        // let user_balance_change = self.output_token_sender_ata.amount - initial_output_user_balance;

        // // transfer any tokens from app ATA to user ATA
        // if user_balance_change == 0 {
        //     Err(CustomError::NoOutputTokens)?;
        // }

        // transfer_token_from_program(
        //     app_balance_change,
        //     &self.output_token_mint,
        //     &self.output_token_app_ata,
        //     &self.output_token_sender_ata,
        //     &[SEED_CONFIG.as_bytes()],
        //     self.bump.config,
        //     &self.config,
        //     &self.token_program,
        // )?;

        Ok(())
    }
}
