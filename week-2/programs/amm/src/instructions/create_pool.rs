use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{helpers::get_space, state::PoolConfig};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreatePool<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub pool_creator: Signer<'info>,

    #[account(
        init,
        payer = pool_creator,
        space = get_space(PoolConfig::INIT_SPACE),
        seeds = [b"pool", id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool_config: Account<'info, PoolConfig>,
    //
    // pub mint_x: InterfaceAccount<'info, Mint>,
    // pub mint_y: InterfaceAccount<'info, Mint>,
    //
    #[account(
        init,
        payer = pool_creator,
        mint::decimals = 6,
        mint::authority = pool_config.key(),
        mint::freeze_authority = pool_config.key(),
        seeds = [b"lp", id.to_le_bytes().as_ref()],
        bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
}

impl<'info> CreatePool<'info> {
    pub fn create_pool(
        &mut self,
        id: u64,
        config_bump: u8,
        lp_bump: u8,
        mint_x: Pubkey,
        mint_y: Pubkey,
        fee_bps: u16,
    ) -> Result<()> {
        let CreatePool {
            pool_creator,
            pool_config,
            mint_lp,
            ..
        } = self;

        pool_config.set_inner(PoolConfig {
            config_bump,
            lp_bump,
            id,
            authority: Some(pool_creator.key()),
            mint_x,
            mint_y,
            mint_lp: mint_lp.key(),
            fee_bps,
            is_locked: false,
        });

        Ok(())
    }
}
