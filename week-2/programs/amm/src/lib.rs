#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod helpers;
pub mod instructions;
pub mod state;

use instructions::create_pool::*;

declare_id!("3F2bCYNtdEw5GcYupVe6g3CepC5VD2yAicZPYz6zjo5W");

#[program]
pub mod amm {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        id: u64,
        mint_x: Pubkey,
        mint_y: Pubkey,
        fee_bps: u16,
    ) -> Result<()> {
        ctx.accounts.create_pool(
            id,
            ctx.bumps.pool_config,
            ctx.bumps.mint_lp,
            mint_x,
            mint_y,
            fee_bps,
        )
    }

    // TODO: provide liquidity
    // TODO: swap
    // TODO: claim
}
