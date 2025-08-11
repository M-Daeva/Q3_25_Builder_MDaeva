use {
    anchor_lang::prelude::*,
    base::{error::AuthError, helpers::get_clock_time},
    dex_adapter_cpi::{
        error::CustomError,
        state::{
            DaBump, DaConfig, RotationState, SEED_ADMIN_ROTATION_STATE, SEED_BUMP, SEED_CONFIG,
        },
    },
};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
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

impl<'info> UpdateConfig<'info> {
    pub fn update_config(
        &mut self,
        admin: Option<Pubkey>,
        dex: Option<Pubkey>,
        registry: Option<Pubkey>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
    ) -> Result<()> {
        let Self {
            sender,
            config,
            admin_rotation_state,
            ..
        } = self;

        if sender.key() != config.admin {
            Err(AuthError::Unauthorized)?;
        }

        let mut is_config_updated = false;

        if let Some(new_admin) = admin {
            admin_rotation_state.new_owner = Some(new_admin);
            admin_rotation_state.expiration_date =
                get_clock_time()? + config.rotation_timeout as u64;
            is_config_updated = true;
        }

        if let Some(x) = dex {
            config.dex = x;
            is_config_updated = true;
        }

        if let Some(x) = registry {
            config.registry = Some(x);
            is_config_updated = true;
        }

        if let Some(x) = is_paused {
            config.is_paused = x;
            is_config_updated = true;
        }

        if let Some(x) = rotation_timeout {
            config.rotation_timeout = x;
            is_config_updated = true;
        }

        // don't allow empty instructions
        if !is_config_updated {
            Err(CustomError::NoParameters)?;
        }

        Ok(())
    }
}
