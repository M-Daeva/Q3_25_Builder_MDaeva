#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use instructions::{delist::*, init::*, list::*, purchase::*};

declare_id!("8Y1PPAsKbeKiT361EbKeCrU9yE1bNLXWNnM7va2PMQ67");

#[program]
pub mod marketplace {
    use super::*;

    //     pub fn init(ctx: Context<Init>, rewards_rate: u8, max_stake: u64) -> Result<()> {
    //         unimplemented!()
    //     }
    // }
}
