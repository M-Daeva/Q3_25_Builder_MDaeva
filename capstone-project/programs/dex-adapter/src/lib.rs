#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod types;

// use {instructions::init::*, types::SwapSpec};

declare_id!("3XEw4Ta4PU5NMET3xJhc71yagoB85awhTzzcNdFbAyBt");

// #[program]
pub mod dex_adapter {
    // use super::*;

    // pub fn init(
    //     registry: Option<Pubkey>,
    //     rotation_timeout: Option<u32>,
    //     token_in_whitelist: Option<Vec<Pubkey>>,
    // ) -> Result<()> {
    //     unimplemented!()
    // }

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
