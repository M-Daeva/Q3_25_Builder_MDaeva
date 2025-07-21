#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::{create_collection::*, mint_token::*};

declare_id!("7sHJDPbAHy5uj7Guz5XD2AgRFKp3KMvYYvNwwTvyw9Lg");

#[program]
pub mod nft {
    use super::*;

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        id: u8,
        metadata: String,
    ) -> Result<()> {
        ctx.accounts
            .create_collection(ctx.bumps.collection, id, metadata)
    }

    pub fn mint_token(ctx: Context<MintToken>, id: u8, metadata: String) -> Result<()> {
        ctx.accounts
            .mint_token(ctx.bumps.token, ctx.bumps.mint, id, metadata)
    }
}
