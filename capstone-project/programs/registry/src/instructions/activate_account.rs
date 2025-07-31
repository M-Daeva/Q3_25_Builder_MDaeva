use {
    crate::{
        error::CustomError,
        state::{
            AccountConfig, Bump, CommonConfig, UserId, SEED_ACCOUNT_CONFIG, SEED_BUMP,
            SEED_COMMON_CONFIG, SEED_USER_ID,
        },
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::transfer_token_from_user,
};

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct ActivateAccount<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

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
        seeds = [SEED_COMMON_CONFIG.as_bytes()],
        bump = bump.common_config
    )]
    pub common_config: Account<'info, CommonConfig>,

    #[account(
        seeds = [SEED_ACCOUNT_CONFIG.as_bytes()],
        bump = bump.account_config
    )]
    pub account_config: Account<'info, AccountConfig>,

    #[account(
        mut,
        seeds = [SEED_USER_ID.as_bytes(), user.as_ref()],
        bump
    )]
    pub user_id: Account<'info, UserId>,

    // mint
    //
    pub revenue_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = revenue_mint,
        associated_token::authority = sender
    )]
    pub revenue_sender_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = revenue_mint,
        associated_token::authority = common_config
    )]
    pub revenue_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> ActivateAccount<'info> {
    pub fn activate_account(&mut self, _user: Pubkey) -> Result<()> {
        let ActivateAccount {
            token_program,
            sender,
            account_config,
            user_id,
            revenue_mint,
            revenue_sender_ata,
            revenue_app_ata,
            ..
        } = self;

        // can't be activated twice
        if user_id.is_activated {
            Err(CustomError::ActivateAccountTwice)?;
        }

        if revenue_mint.key() != account_config.registration_fee.asset {
            Err(CustomError::WrongAssetType)?;
        }

        user_id.is_activated = true;

        transfer_token_from_user(
            account_config.registration_fee.amount,
            revenue_mint,
            revenue_sender_ata,
            revenue_app_ata,
            sender,
            token_program,
        )?;

        Ok(())
    }
}
