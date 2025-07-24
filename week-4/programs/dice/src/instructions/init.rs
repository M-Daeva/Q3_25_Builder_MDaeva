use {anchor_lang::prelude::*, base::helpers::transfer_sol_from_user};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub house: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self, amount: u64) -> Result<()> {
        let Init {
            system_program,
            house,
            vault,
        } = self;

        transfer_sol_from_user(amount, house, vault, system_program)
    }
}
