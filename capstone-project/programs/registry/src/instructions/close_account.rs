use {
    crate::state::{
        RotationState, UserAccount, UserId, SEED_USER_ACCOUNT, SEED_USER_ID,
        SEED_USER_ROTATION_STATE,
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        mut,
        seeds = [SEED_USER_ID.as_bytes(), sender.key().as_ref()],
        bump
    )]
    pub user_id: Account<'info, UserId>,

    #[account(
        mut,
        close = sender,
        seeds = [SEED_USER_ACCOUNT.as_bytes(), user_id.id.to_le_bytes().as_ref()],
        bump = user_id.account_bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        close = sender,
        seeds = [SEED_USER_ROTATION_STATE.as_bytes(), user_id.id.to_le_bytes().as_ref()],
        bump = user_id.rotation_state_bump
    )]
    pub user_rotation_state: Account<'info, RotationState>,
}

impl<'info> CloseAccount<'info> {
    pub fn close_account(&mut self) -> Result<()> {
        let CloseAccount { user_id, .. } = self;

        user_id.is_open = false;

        Ok(())
    }
}
