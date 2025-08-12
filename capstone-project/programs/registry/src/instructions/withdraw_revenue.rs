use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::{error::AuthError, helpers::transfer_token_from_program},
    registry_cpi::{
        error::CustomError,
        state::{Bump, Config, SEED_BUMP, SEED_CONFIG},
    },
};

#[derive(Accounts)]
pub struct WithdrawRevenue<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // handle the option on client
    pub recipient: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config
    )]
    pub config: Account<'info, Config>,

    // mint
    //
    pub revenue_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = revenue_mint,
        associated_token::authority = recipient
    )]
    pub revenue_recipient_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = revenue_mint,
        associated_token::authority = config
    )]
    pub revenue_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> WithdrawRevenue<'info> {
    pub fn withdraw_revenue(&mut self, amount: Option<u64>) -> Result<()> {
        let Self {
            token_program,
            sender,
            bump,
            config,
            revenue_mint,
            revenue_recipient_ata,
            revenue_app_ata,
            ..
        } = self;

        // check sender
        if sender.key() != config.admin {
            Err(AuthError::Unauthorized)?;
        }

        // validate fee token
        if revenue_mint.key() != config.registration_fee.asset {
            Err(CustomError::WrongAssetType)?;
        }

        let amount = amount.unwrap_or(revenue_app_ata.amount);

        // lower limit of amount to withdraw
        if amount == 0 {
            Err(CustomError::ZeroAmount)?;
        }

        // higher limit of amount to withdraw
        if amount > revenue_app_ata.amount {
            Err(CustomError::ExceededAvailableAssetAmount)?;
        }

        transfer_token_from_program(
            amount,
            revenue_mint,
            revenue_app_ata,
            revenue_recipient_ata,
            &[SEED_CONFIG.as_bytes()],
            bump.config,
            config,
            token_program,
        )?;

        Ok(())
    }
}
