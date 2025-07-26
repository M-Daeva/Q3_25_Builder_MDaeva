#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::init::*;

declare_id!("3XEw4Ta4PU5NMET3xJhc71yagoB85awhTzzcNdFbAyBt");

// #[program]
pub mod dex_adapter {
    use super::*;

    pub fn init(amount: u64) -> Result<()> {
        unimplemented!()
    }
}
