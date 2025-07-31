use {
    crate::{
        state::{
            Bump, Config, RotationState, UserCounter, ACCOUNT_DATA_SIZE_MAX, ACCOUNT_DATA_SIZE_MIN,
            ACCOUNT_REGISTRATION_FEE_AMOUNT, ACCOUNT_REGISTRATION_FEE_ASSET, ROTATION_TIMEOUT,
            SEED_ADMIN_ROTATION_STATE, SEED_BUMP, SEED_CONFIG, SEED_USER_COUNTER,
        },
        types::{AssetItem, Range},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::{get_clock_time, get_space},
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

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
        space = get_space(UserCounter::INIT_SPACE),
        seeds = [SEED_USER_COUNTER.as_bytes()],
        bump
    )]
    pub user_counter: Account<'info, UserCounter>,

    #[account(
        init,
        payer = sender,
        space = get_space(RotationState::INIT_SPACE),
        seeds = [SEED_ADMIN_ROTATION_STATE.as_bytes()],
        bump
    )]
    pub admin_rotation_state: Account<'info, RotationState>,

    // mint
    //
    pub revenue_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        init,
        payer = sender,
        associated_token::mint = revenue_mint,
        associated_token::authority = config
    )]
    pub revenue_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Init<'info> {
    pub fn init(
        &mut self,
        bumps: InitBumps,
        rotation_timeout: Option<u32>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
    ) -> Result<()> {
        let Init {
            sender,
            bump,
            config,
            user_counter,
            admin_rotation_state,
            ..
        } = self;

        bump.set_inner(Bump {
            config: bumps.config,
            user_counter: bumps.user_counter,
            rotation_state: bumps.admin_rotation_state,
        });

        config.set_inner(Config {
            admin: sender.key(),
            is_paused: false,
            rotation_timeout: rotation_timeout.unwrap_or(ROTATION_TIMEOUT),
            registration_fee: account_registration_fee.unwrap_or(AssetItem {
                amount: ACCOUNT_REGISTRATION_FEE_AMOUNT,
                asset: ACCOUNT_REGISTRATION_FEE_ASSET,
            }),
            data_size_range: account_data_size_range.unwrap_or(Range {
                min: ACCOUNT_DATA_SIZE_MIN,
                max: ACCOUNT_DATA_SIZE_MAX,
            }),
        });

        user_counter.set_inner(UserCounter::default());

        admin_rotation_state.set_inner(RotationState {
            owner: sender.key(),
            new_owner: None,
            expiration_date: get_clock_time()?,
        });

        Ok(())
    }
}
