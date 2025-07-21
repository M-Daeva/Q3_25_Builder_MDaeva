use {
    crate::{
        math::get_updated_vault,
        state::{Config, Vault},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::mint_to,
};

#[derive(Accounts)]
pub struct Claim<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_vault", user.key().as_ref()],
        bump
    )]
    pub user_vault: Account<'info, Vault>,

    #[account(
        seeds = [b"config"],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"rewards_mint"],
        bump = config.rewards_bump
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user
    )]
    pub user_rewards_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let clock_time = Clock::get()?.unix_timestamp as u64;
        let Claim {
            token_program,
            user_vault,
            config,
            rewards_mint,
            user_rewards_ata,
            ..
        } = self;

        user_vault.set_inner(get_updated_vault(
            &user_vault,
            config.rewards_rate,
            clock_time,
        ));
        let amount_to_mint = user_vault.rewards;
        user_vault.rewards = 0;

        mint_to(
            amount_to_mint,
            rewards_mint,
            user_rewards_ata,
            &[b"rewards_mint"],
            config.rewards_bump,
            rewards_mint,
            token_program,
        )?;

        Ok(())
    }
}
