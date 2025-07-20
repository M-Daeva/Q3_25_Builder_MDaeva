use {
    crate::{helpers::get_space, state::Config},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token},
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, // TODO: use Interface<'info, TokenInterface>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = get_space(Config::INIT_SPACE),
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = rewards_mint,
        mint::freeze_authority = rewards_mint,
        seeds = [b"rewards_mint"],
        bump
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub nft_mint: Account<'info, Mint>,
}

impl<'info> Init<'info> {
    pub fn init(
        &mut self,
        config_bump: u8,
        rewards_bump: u8,
        rewards_rate: u8,
        max_stake: u64,
    ) -> Result<()> {
        let Init {
            config,
            rewards_mint,
            nft_mint,
            ..
        } = self;

        config.set_inner(Config {
            config_bump,
            rewards_bump,
            rewards_rate,
            max_stake,
            nft_mint: nft_mint.key(),
            rewards_mint: rewards_mint.key(),
        });

        Ok(())
    }
}
