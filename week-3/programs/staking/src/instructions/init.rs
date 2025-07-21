use {
    crate::state::Config,
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::get_space,
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

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
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = nft_mint,
        associated_token::authority = config
    )]
    pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Init<'info> {
    pub fn init(
        &mut self,
        config_bump: u8,
        rewards_bump: u8,
        rewards_rate: u8,
        max_stake: u64,
        collection: Pubkey,
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
            collection,
            nft_mint: nft_mint.key(),
            rewards_mint: rewards_mint.key(),
        });

        Ok(())
    }
}
