use {crate::state::Bet, anchor_lang::prelude::*, base::helpers::transfer_sol_from_program};

#[derive(Accounts)]
#[instruction(id: u128)]
pub struct RefundBet<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: this is safe
    pub house: UncheckedAccount<'info>,

    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(&mut self, bumps: &RefundBetBumps, _id: u128) -> Result<()> {
        let RefundBet {
            system_program,
            player,
            house,
            bet,
            vault,
        } = self;

        transfer_sol_from_program(
            bet.amount,
            vault,
            player,
            &[b"vault", house.key().as_ref()],
            bumps.vault,
            system_program,
        )
    }
}
