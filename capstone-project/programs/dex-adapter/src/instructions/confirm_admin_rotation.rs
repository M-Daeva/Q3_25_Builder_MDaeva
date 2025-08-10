use {
    anchor_lang::prelude::*,
    base::{error::AuthError, helpers::get_clock_time},
    dex_adapter_cpi::state::{
        DaBump, DaConfig, RotationState, SEED_ADMIN_ROTATION_STATE, SEED_BUMP, SEED_CONFIG,
    },
};

#[derive(Accounts)]
pub struct ConfirmAdminRotation<'info> {
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, DaBump>,

    #[account(
        mut,
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config
    )]
    pub config: Account<'info, DaConfig>,

    #[account(
        mut,
        seeds = [SEED_ADMIN_ROTATION_STATE.as_bytes()],
        bump = bump.rotation_state
    )]
    pub admin_rotation_state: Account<'info, RotationState>,
}

impl<'info> ConfirmAdminRotation<'info> {
    pub fn confirm_admin_rotation(&mut self) -> Result<()> {
        let Self {
            sender,
            config,
            admin_rotation_state,
            ..
        } = self;

        let clock_time = get_clock_time()?;

        match admin_rotation_state.new_owner {
            None => Err(AuthError::NoNewOwner)?,
            Some(new_admin) => {
                if sender.key() != new_admin {
                    Err(AuthError::Unauthorized)?;
                }

                if clock_time >= admin_rotation_state.expiration_date {
                    Err(AuthError::TransferOwnerDeadline)?;
                }

                config.admin = new_admin;

                admin_rotation_state.set_inner(RotationState {
                    owner: new_admin,
                    new_owner: None,
                    expiration_date: clock_time,
                });
            }
        }

        Ok(())
    }
}
