use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ProgError,
    helpers::{get_space, has_duplicates},
    math::get_updated_vault,
    state::{Config, Vault},
};

#[derive(Accounts)]
pub struct Stake<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = get_space(Vault::INIT_SPACE),
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

impl<'info> Stake<'info> {
    pub fn stake(&mut self, tokens: Vec<u16>) -> Result<()> {
        let clock_time = Clock::get()?.unix_timestamp as u64;
        let Stake {
            system_program,
            token_program,
            associated_token_program,
            user,
            user_vault,
            config,
            nft_mint,
            user_nft_ata,
            app_nft_ata,
        } = self;

        if tokens.is_empty() {
            Err(ProgError::EmptyTokenList)?;
        }

        user_vault.set_inner(get_updated_vault(
            &user_vault,
            config.rewards_rate,
            clock_time,
        ));
        user_vault.tokens.extend(tokens);

        if has_duplicates(&user_vault.tokens) {
            Err(ProgError::NftDuplication)?;
        }

        if user_vault.rewards > config.max_stake {
            Err(ProgError::ExceededTokenLimit)?;
        }

        // TODO: handle nft

        Ok(())
    }
}
