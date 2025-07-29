use {
    crate::state::{
        Bump, CommonConfig, RotationState, SEED_ADMIN_ROTATION_STATE, SEED_BUMP, SEED_COMMON_CONFIG,
    },
    anchor_lang::prelude::*,
    base::{error::AuthError, helpers::get_clock_time},
};

#[derive(Accounts)]
pub struct ConfirmAdminRotation<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        mut,
        seeds = [SEED_COMMON_CONFIG.as_bytes()],
        bump = bump.common_config
    )]
    pub common_config: Account<'info, CommonConfig>,

    #[account(
        mut,
        seeds = [SEED_ADMIN_ROTATION_STATE.as_bytes()],
        bump = bump.rotation_state
    )]
    pub admin_rotation_state: Account<'info, RotationState>,
}

impl<'info> ConfirmAdminRotation<'info> {
    pub fn confirm_admin_rotation(&mut self) -> Result<()> {
        let ConfirmAdminRotation {
            sender,
            common_config,
            admin_rotation_state,
            ..
        } = self;

        let clock_time = get_clock_time()?;

        match admin_rotation_state.new_owner {
            None => Err(AuthError::NoNewAdmin)?,
            Some(new_admin) => {
                if sender.key() != new_admin {
                    Err(AuthError::Unauthorized)?;
                }

                if clock_time >= admin_rotation_state.expiration_date {
                    Err(AuthError::TransferAdminDeadline)?;
                }

                common_config.admin = new_admin;
                admin_rotation_state.new_owner = None;
                admin_rotation_state.expiration_date = clock_time;
            }
        }

        Ok(())
    }
}
