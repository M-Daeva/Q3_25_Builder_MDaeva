use {
    crate::{
        state::{
            AccountConfig, Bump, CommonConfig, RotationState, UserCounter, ACCOUNT_DATA_SIZE_MAX,
            ACCOUNT_DATA_SIZE_MIN, ACCOUNT_LIFETIME_MARGIN_BPS, ACCOUNT_LIFETIME_MAX,
            ACCOUNT_LIFETIME_MIN, ACCOUNT_REGISTRATION_FEE_AMOUNT, ACCOUNT_REGISTRATION_FEE_ASSET,
            ROTATION_TIMEOUT, SEED_ACCOUNT_CONFIG, SEED_ADMIN_ROTATION_STATE, SEED_BUMP,
            SEED_COMMON_CONFIG, SEED_USER_COUNTER,
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
        seeds = [SEED_BUMP],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        init,
        payer = sender,
        space = get_space(CommonConfig::INIT_SPACE),
        seeds = [SEED_COMMON_CONFIG],
        bump
    )]
    pub common_config: Account<'info, CommonConfig>,

    #[account(
        init,
        payer = sender,
        space = get_space(AccountConfig::INIT_SPACE),
        seeds = [SEED_ACCOUNT_CONFIG],
        bump
    )]
    pub account_config: Account<'info, AccountConfig>,

    #[account(
        init,
        payer = sender,
        space = get_space(UserCounter::INIT_SPACE),
        seeds = [SEED_USER_COUNTER],
        bump
    )]
    pub user_counter: Account<'info, UserCounter>,

    #[account(
        init,
        payer = sender,
        space = get_space(RotationState::INIT_SPACE),
        seeds = [SEED_ADMIN_ROTATION_STATE],
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
        associated_token::authority = bump
    )]
    pub revenue_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Init<'info> {
    pub fn init(
        &mut self,
        bumps: InitBumps,
        dex_adapter: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
        account_lifetime_range: Option<Range>,
        account_lifetime_margin_bps: Option<u16>,
    ) -> Result<()> {
        let Init {
            sender,
            bump,
            common_config,
            account_config,
            user_counter,
            admin_rotation_state,
            ..
        } = self;

        bump.set_inner(Bump {
            common_config: bumps.common_config,
            account_config: bumps.account_config,
            user_counter: bumps.user_counter,
            rotation_state: bumps.admin_rotation_state,
        });

        common_config.set_inner(CommonConfig {
            admin: sender.key(),
            dex_adapter,
            is_paused: false,
            rotation_timeout: rotation_timeout.unwrap_or(ROTATION_TIMEOUT),
        });

        account_config.set_inner(AccountConfig {
            registration_fee: account_registration_fee.unwrap_or(AssetItem {
                amount: ACCOUNT_REGISTRATION_FEE_AMOUNT,
                asset: ACCOUNT_REGISTRATION_FEE_ASSET,
            }),
            data_size_range: account_data_size_range.unwrap_or(Range {
                min: ACCOUNT_DATA_SIZE_MIN,
                max: ACCOUNT_DATA_SIZE_MAX,
            }),
            lifetime_range: account_lifetime_range.unwrap_or(Range {
                min: ACCOUNT_LIFETIME_MIN,
                max: ACCOUNT_LIFETIME_MAX,
            }),
            lifetime_margin_bps: account_lifetime_margin_bps.unwrap_or(ACCOUNT_LIFETIME_MARGIN_BPS),
        });

        user_counter.set_inner(UserCounter::default());

        admin_rotation_state.set_inner(RotationState {
            new_owner: None,
            expiration_date: get_clock_time()?,
        });

        Ok(())
    }
}
