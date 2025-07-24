use {
    crate::state::Bet,
    anchor_lang::prelude::*,
    base::helpers::{get_space, transfer_sol_from_user},
};

#[derive(Accounts)]
#[instruction(id: u128)]
pub struct PlaceBet<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: this is safe
    pub house: UncheckedAccount<'info>,

    #[account(
        init,
        payer = player,
        space = get_space(Bet::INIT_SPACE),
        seeds = [b"bet", vault.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        bumps: &PlaceBetBumps,
        id: u128,
        roll: u8,
        amount: u64,
    ) -> Result<()> {
        let PlaceBet {
            system_program,
            player,
            bet,
            vault,
            ..
        } = self;

        bet.set_inner(Bet {
            bump: bumps.bet,
            id,
            player: player.key(),
            amount,
            slot: Clock::get()?.slot,
            roll,
        });

        transfer_sol_from_user(amount, player, vault, system_program)
    }
}
