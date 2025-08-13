use {
    anchor_lang::prelude::*,
    base::{
        error::AuthError,
        helpers::{get_clock_time, get_space},
    },
    dex_adapter_cpi::state::{
        DaBump, DaConfig, RotationState, CLOCK_TIME_MIN, MAINNET_ADMIN, ROTATION_TIMEOUT,
        SEED_ADMIN_ROTATION_STATE, SEED_BUMP, SEED_CONFIG,
    },
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        init,
        payer = sender,
        space = get_space(DaBump::INIT_SPACE),
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, DaBump>,

    #[account(
        init,
        payer = sender,
        space = get_space(DaConfig::INIT_SPACE),
        seeds = [SEED_CONFIG.as_bytes()],
        bump
    )]
    pub config: Account<'info, DaConfig>,

    #[account(
        init,
        payer = sender,
        space = get_space(RotationState::INIT_SPACE),
        seeds = [SEED_ADMIN_ROTATION_STATE.as_bytes()],
        bump
    )]
    pub admin_rotation_state: Account<'info, RotationState>,
}

impl<'info> Init<'info> {
    pub fn init(
        &mut self,
        bumps: InitBumps,
        dex: Pubkey,
        registry: Option<Pubkey>,
        rotation_timeout: Option<u32>,
    ) -> Result<()> {
        let clock_time = get_clock_time()?;
        let Self {
            sender,
            bump,
            config,
            admin_rotation_state,
            ..
        } = self;

        // devnet/mainnet program must be initialized by specified address
        if clock_time > CLOCK_TIME_MIN && sender.key() != MAINNET_ADMIN {
            Err(AuthError::Unauthorized)?;
        }

        bump.set_inner(DaBump {
            config: bumps.config,
            rotation_state: bumps.admin_rotation_state,
        });

        config.set_inner(DaConfig {
            admin: sender.key(),
            dex,
            registry,
            is_paused: false,
            rotation_timeout: rotation_timeout.unwrap_or(ROTATION_TIMEOUT),
        });

        admin_rotation_state.set_inner(RotationState {
            owner: sender.key(),
            new_owner: None,
            expiration_date: clock_time,
        });

        Ok(())
    }
}
