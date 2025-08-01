#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod types;

use instructions::init::*;

declare_id!("FMsjKKPk7FQb1B9H8UQTLrdCUZ9MaoAeTnNK9kdVJmtt");

#[program]
pub mod dex_adapter {
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        registry: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        token_in_whitelist: Option<Vec<Pubkey>>,
    ) -> Result<()> {
        ctx.accounts
            .init(ctx.bumps, registry, rotation_timeout, token_in_whitelist)
    }

    // pub fn update_config(
    //     admin: Option<Pubkey>,
    //     registry: Option<Pubkey>,
    //     is_paused: Option<bool>,
    //     rotation_timeout: Option<u32>,
    //     token_in_whitelist: Option<Vec<Pubkey>>,
    // ) -> Result<()> {
    //     unimplemented!()
    // }

    // pub fn confirm_admin_rotation() -> Result<()> {
    //     unimplemented!()
    // }

    // /// swap tokens and forward result to registry program (call receive_payment)
    // pub fn swap_and_forward(amount_in: u64, token_out: Pubkey, min_amount_out: u64) -> Result<()> {
    //     unimplemented!()
    // }

    // /// multi-output swap: one input token → multiple output tokens
    // pub fn multi_swap(
    //     amount_in: u64,
    //     swap_specs: Vec<SwapSpec>, // each spec defines output token and ratio
    // ) -> Result<()> {
    //     unimplemented!()
    // }

    // /// unwrap WSOL and send native SOL to user
    // pub fn unwrap_and_send_sol(amount_in: u64, recipient: Option<Pubkey>) -> Result<()> {
    //     unimplemented!()
    // }
}
