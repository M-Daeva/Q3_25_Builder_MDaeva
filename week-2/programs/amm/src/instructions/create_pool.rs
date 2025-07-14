use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    helpers::get_space,
    state::{PoolBalance, PoolConfig},
};

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
        seeds = [b"config", id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool_config: Account<'info, PoolConfig>,

    #[account(
        init,
        payer = pool_creator,
        space = get_space(PoolBalance::INIT_SPACE),
        seeds = [b"balance", id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool_balance: Account<'info, PoolBalance>,

    #[account(
        init,
        payer = pool_creator,
        mint::decimals = 6,
        mint::authority = mint_lp.key(),
        mint::freeze_authority = mint_lp.key(),
        seeds = [b"lp", id.to_le_bytes().as_ref()],
        bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,

    // pool creator should pay for creating pool's ata
    #[account(
        init,
        payer = pool_creator,
        associated_token::token_program = token_program,
        associated_token::mint = mint_lp,
        associated_token::authority = pool_config,
    )]
    pub liquidity_pool_mint_lp_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = pool_creator,
        associated_token::token_program = token_program,
        associated_token::mint = mint_x,
        associated_token::authority = pool_config,
    )]
    pub liquidity_pool_mint_x_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = pool_creator,
        associated_token::token_program = token_program,
        associated_token::mint = mint_y,
        associated_token::authority = pool_config,
    )]
    pub liquidity_pool_mint_y_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreatePool<'info> {
    pub fn create_pool(
        &mut self,
        id: u64,
        config_bump: u8,
        balance_bump: u8,
        lp_bump: u8,
        mint_x: Pubkey,
        mint_y: Pubkey,
        fee_bps: u16,
    ) -> Result<()> {
        let CreatePool {
            pool_creator,
            pool_config,
            pool_balance,
            mint_lp,
            ..
        } = self;

        pool_config.set_inner(PoolConfig {
            config_bump,
            balance_bump,
            lp_bump,
            id,
            authority: Some(pool_creator.key()),
            mint_x,
            mint_y,
            mint_lp: mint_lp.key(),
            fee_bps,
            is_locked: false,
        });

        pool_balance.set_inner(PoolBalance::default());

        Ok(())
    }
}
