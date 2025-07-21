use {
    crate::{
        math::get_updated_vault,
        state::{Config, Vault},
    },
    anchor_lang::prelude::*,
    anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface},
    base::{error::NftError, helpers::transfer_from_program},
};

#[derive(Accounts)]
#[instruction(token_id: u16)]
pub struct Unstake<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,

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

    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = config
    )]
    pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self, token_id: u16) -> Result<()> {
        let clock_time = Clock::get()?.unix_timestamp as u64;
        let Unstake {
            token_program,
            user_vault,
            config,
            nft_mint,
            user_nft_ata,
            app_nft_ata,
            ..
        } = self;

        if !user_vault.tokens.contains(&token_id) {
            Err(NftError::NftIsNotFound)?;
        }

        user_vault.set_inner(get_updated_vault(
            &user_vault,
            config.rewards_rate,
            clock_time,
        ));
        user_vault.tokens.retain(|x| x != &token_id);

        transfer_from_program(
            1,
            nft_mint,
            &app_nft_ata,
            &user_nft_ata,
            &[b"config"],
            config.config_bump,
            config,
            token_program,
        )?;

        Ok(())
    }
}
