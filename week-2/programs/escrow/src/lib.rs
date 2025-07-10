#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod helpers;
pub mod instructions;
pub mod state;

use instructions::{make::*, refund::*, take::*};
use state::TraderInfo;

declare_id!("3F2bCYNtdEw5GcYupVe6g3CepC5VD2yAicZPYz6zjo5W");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, id: u8, maker: TraderInfo, taker: TraderInfo) -> Result<()> {
        ctx.accounts.make(ctx.bumps.escrow_state, id, maker, taker)
    }

    pub fn refund(ctx: Context<Refund>, id: u8) -> Result<()> {
        ctx.accounts.refund(id)
    }

    pub fn take(ctx: Context<Take>, id: u8, maker: Pubkey) -> Result<()> {
        ctx.accounts.take(id, maker)
    }
}
