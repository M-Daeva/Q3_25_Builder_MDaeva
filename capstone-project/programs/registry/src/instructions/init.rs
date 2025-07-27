use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {}
