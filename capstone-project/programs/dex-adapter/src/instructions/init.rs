use {
    anchor_lang::prelude::*,
    base::helpers::{get_clock_time, get_space},
    dex_adapter_cpi::state::{
        Bump, Config, RotationState, MAINNET_USDC, MAINNET_WBTC, MAINNET_WSOL, ROTATION_TIMEOUT,
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
        space = get_space(Bump::INIT_SPACE),
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        init,
        payer = sender,
        space = get_space(Config::INIT_SPACE),
        seeds = [SEED_CONFIG.as_bytes()],
        bump
    )]
    pub config: Account<'info, Config>,

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
        token_in_whitelist: Option<Vec<Pubkey>>,
    ) -> Result<()> {
        let Self {
            sender,
            bump,
            config,
            admin_rotation_state,
            ..
        } = self;

        bump.set_inner(Bump {
            config: bumps.config,
            rotation_state: bumps.admin_rotation_state,
        });

        config.set_inner(Config {
            admin: sender.key(),
            dex,
            registry,
            is_paused: false,
            rotation_timeout: rotation_timeout.unwrap_or(ROTATION_TIMEOUT),
            token_in_whitelist: token_in_whitelist.unwrap_or(vec![
                MAINNET_USDC,
                MAINNET_WSOL,
                MAINNET_WBTC,
            ]),
        });

        admin_rotation_state.set_inner(RotationState {
            new_owner: None,
            expiration_date: get_clock_time()?,
        });

        Ok(())
    }
}
