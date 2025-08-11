use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::transfer_token_from_user,
    registry_cpi::{
        error::CustomError,
        state::{Bump, Config, UserId, SEED_BUMP, SEED_CONFIG, SEED_USER_ID},
    },
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
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config
    )]
    pub config: Account<'info, Config>,

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
        associated_token::authority = config
    )]
    pub revenue_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> ActivateAccount<'info> {
    pub fn activate_account(&mut self, _user: Pubkey) -> Result<()> {
        let Self {
            token_program,
            sender,
            config,
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

        if revenue_mint.key() != config.registration_fee.asset {
            Err(CustomError::WrongAssetType)?;
        }

        user_id.is_activated = true;

        transfer_token_from_user(
            config.registration_fee.amount,
            revenue_mint,
            revenue_sender_ata,
            revenue_app_ata,
            sender,
            token_program,
        )?;

        Ok(())
    }
}
