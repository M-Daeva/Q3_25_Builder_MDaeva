use {anchor_lang::prelude::*, base::helpers::transfer_sol_from_user};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {}
