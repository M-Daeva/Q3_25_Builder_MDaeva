use {
    crate::state::{
        Bump, CommonConfig, RotationState, UserId, SEED_BUMP, SEED_COMMON_CONFIG, SEED_USER_ID,
        SEED_USER_ROTATION_STATE,
    },
    anchor_lang::prelude::*,
    base::helpers::get_clock_time,
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
        seeds = [SEED_COMMON_CONFIG.as_bytes()],
        bump = bump.common_config
    )]
    pub common_config: Account<'info, CommonConfig>,

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
        let RequestAccountRotation {
            user_rotation_state,
            common_config,
            ..
        } = self;

        user_rotation_state.new_owner = Some(new_owner);
        user_rotation_state.expiration_date =
            get_clock_time()? + common_config.rotation_timeout as u64;

        Ok(())
    }
}
