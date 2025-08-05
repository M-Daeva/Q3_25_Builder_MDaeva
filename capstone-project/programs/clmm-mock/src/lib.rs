#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::create_pool::*};

declare_id!("AyzvQE5M1Xqs4YQxP4Sf6giH82X4bKFUF3oVv7DTkoD4");

#[program]
pub mod clmm_mock {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        id: u8,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        ctx.accounts.create_pool(id, amount_a, amount_b)
    }
}
