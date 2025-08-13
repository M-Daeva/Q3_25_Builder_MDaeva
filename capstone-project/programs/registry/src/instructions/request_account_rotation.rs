use {
    anchor_lang::prelude::*,
    base::{error::AuthError, helpers::get_clock_time},
    registry_cpi::state::{
        Bump, Config, RotationState, UserId, SEED_BUMP, SEED_CONFIG, SEED_USER_ID,
        SEED_USER_ROTATION_STATE,
    },
};

#[derive(Accounts)]
pub struct RequestAccountRotation<'info> {
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [SEED_USER_ID.as_bytes(), sender.key().as_ref()],
        bump
    )]
    pub user_id: Account<'info, UserId>,

    #[account(
        mut,
        seeds = [SEED_USER_ROTATION_STATE.as_bytes(), user_id.id.to_le_bytes().as_ref()],
        bump = user_id.rotation_state_bump
    )]
    pub user_rotation_state: Account<'info, RotationState>,
}

impl<'info> RequestAccountRotation<'info> {
    pub fn request_account_rotation(&mut self, new_owner: Pubkey) -> Result<()> {
        let Self {
            sender,
            user_rotation_state,
            config,
            ..
        } = self;

        if new_owner == sender.key() {
            Err(AuthError::UselessRotation)?;
        }

        user_rotation_state.new_owner = Some(new_owner);
        user_rotation_state.expiration_date = get_clock_time()? + config.rotation_timeout as u64;

        Ok(())
    }
}
