#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::{init::*, place_bet::*, refund_bet::*, resolve_bet::*};

declare_id!("3XEw4Ta4PU5NMET3xJhc71yagoB85awhTzzcNdFbAyBt");

#[program]
pub mod dice {
    use super::*;

    pub fn init(ctx: Context<Init>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, id: u128, roll: u8, amount: u64) -> Result<()> {
        ctx.accounts.create_bet(&ctx.bumps, id, roll, amount)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, id: u128, sig: [u8; 64]) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&ctx.bumps, id, &sig)
    }

    pub fn refund_bet(ctx: Context<RefundBet>, id: u128) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps, id)
    }
}
