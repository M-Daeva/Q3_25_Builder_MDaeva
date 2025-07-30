use {
    crate::{
        error::CustomError,
        state::{
            AccountConfig, Bump, CommonConfig, SEED_ACCOUNT_CONFIG, SEED_BUMP, SEED_COMMON_CONFIG,
        },
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::{error::AuthError, helpers::transfer_token_from_program},
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
        seeds = [SEED_COMMON_CONFIG.as_bytes()],
        bump = bump.common_config
    )]
    pub common_config: Account<'info, CommonConfig>,

    #[account(
        seeds = [SEED_ACCOUNT_CONFIG.as_bytes()],
        bump = bump.account_config
    )]
    pub account_config: Account<'info, AccountConfig>,

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
        associated_token::authority = common_config
    )]
    pub revenue_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> WithdrawRevenue<'info> {
    pub fn withdraw_revenue(&mut self, amount: Option<u64>) -> Result<()> {
        let WithdrawRevenue {
            token_program,
            sender,
            bump,
            common_config,
            account_config,
            revenue_mint,
            revenue_recipient_ata,
            revenue_app_ata,
            ..
        } = self;

        if sender.key() != common_config.admin {
            Err(AuthError::Unauthorized)?;
        }

        if revenue_mint.key() != account_config.registration_fee.asset {
            Err(CustomError::WrongAssetType)?;
        }

        let amount = amount.unwrap_or(revenue_app_ata.amount);

        if amount == 0 {
            Err(CustomError::ZeroAmount)?;
        }

        if amount > revenue_app_ata.amount {
            Err(CustomError::ExceededAvailableAssetAmount)?;
        }

        transfer_token_from_program(
            amount,
            revenue_mint,
            revenue_app_ata,
            revenue_recipient_ata,
            &[SEED_COMMON_CONFIG.as_bytes()],
            bump.common_config,
            common_config,
            token_program,
        )?;

        Ok(())
    }
}
