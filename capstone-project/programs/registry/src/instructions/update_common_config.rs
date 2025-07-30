use {
    crate::{
        error::CustomError,
        state::{
            Bump, CommonConfig, RotationState, SEED_ADMIN_ROTATION_STATE, SEED_BUMP,
            SEED_COMMON_CONFIG,
        },
    },
    anchor_lang::prelude::*,
    base::{error::AuthError, helpers::get_clock_time},
};

#[derive(Accounts)]
pub struct UpdateCommonConfig<'info> {
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

impl<'info> UpdateCommonConfig<'info> {
    pub fn update_common_config(
        &mut self,
        admin: Option<Pubkey>,
        dex_adapter: Option<Pubkey>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
    ) -> Result<()> {
        let UpdateCommonConfig {
            sender,
            common_config,
            admin_rotation_state,
            ..
        } = self;

        if sender.key() != common_config.admin {
            Err(AuthError::Unauthorized)?;
        }

        let mut is_config_updated = false;

        if let Some(new_admin) = admin {
            admin_rotation_state.new_owner = Some(new_admin);
            admin_rotation_state.expiration_date =
                get_clock_time()? + common_config.rotation_timeout as u64;
            is_config_updated = true;
        }

        if let Some(x) = dex_adapter {
            common_config.dex_adapter = Some(x);
            is_config_updated = true;
        }

        if let Some(x) = is_paused {
            common_config.is_paused = x;
            is_config_updated = true;
        }

        if let Some(x) = rotation_timeout {
            common_config.rotation_timeout = x;
            is_config_updated = true;
        }

        // don't allow empty instructions
        if !is_config_updated {
            Err(CustomError::NoParameters)?;
        }

        Ok(())
    }
}
