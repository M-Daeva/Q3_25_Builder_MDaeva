use {
    crate::state::{RotationState, UserId, SEED_USER_ID, SEED_USER_ROTATION_STATE},
    anchor_lang::prelude::*,
    base::{
        error::AuthError,
        helpers::{get_clock_time, get_space},
    },
};

#[derive(Accounts)]
pub struct ConfirmAccountRotation<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        mut,
        close = sender,
        seeds = [SEED_USER_ID.as_bytes(), user_rotation_state.owner.as_ref()],
        bump
    )]
    pub user_id_pre: Account<'info, UserId>,

    #[account(
        init,
        payer = sender,
        space = get_space(UserId::INIT_SPACE),
        seeds = [SEED_USER_ID.as_bytes(), sender.key().as_ref()],
        bump
    )]
    pub user_id: Account<'info, UserId>,

    #[account(
        mut,
        seeds = [SEED_USER_ROTATION_STATE.as_bytes(), user_id_pre.id.to_le_bytes().as_ref()],
        bump = user_id_pre.rotation_state_bump
    )]
    pub user_rotation_state: Account<'info, RotationState>,
}

impl<'info> ConfirmAccountRotation<'info> {
    pub fn confirm_account_rotation(&mut self) -> Result<()> {
        let ConfirmAccountRotation {
            sender,
            user_id_pre,
            user_id,
            user_rotation_state,
            ..
        } = self;

        let clock_time = get_clock_time()?;

        match user_rotation_state.new_owner {
            None => Err(AuthError::NoNewOwner)?,
            Some(new_owner) => {
                if sender.key() != new_owner {
                    Err(AuthError::Unauthorized)?;
                }

                if clock_time >= user_rotation_state.expiration_date {
                    Err(AuthError::TransferOwnerDeadline)?;
                }

                user_id.set_inner(user_id_pre.clone().into_inner());

                user_rotation_state.set_inner(RotationState {
                    owner: new_owner,
                    new_owner: None,
                    expiration_date: clock_time,
                });
            }
        }

        Ok(())
    }
}
