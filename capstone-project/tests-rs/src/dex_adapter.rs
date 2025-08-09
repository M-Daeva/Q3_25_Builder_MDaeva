use {
    crate::{
        clmm_mock::{prepare_dex, AMM_CONFIG_INDEX_0, AMM_CONFIG_INDEX_1},
        helpers::{
            extensions::dex_adapter::DexAdapterExtension,
            suite::{
                core::App,
                types::{AppToken, AppUser},
            },
        },
    },
    anchor_lang::Result,
    dex_adapter_cpi::{
        state::{Config, ROTATION_TIMEOUT},
        types::RouteItem,
    },
    pretty_assertions::assert_eq,
    solana_pubkey::Pubkey,
};

#[test]
fn init_default() -> Result<()> {
    let mut app = App::new();

    app.dex_adapter_try_init(
        AppUser::Admin,
        Pubkey::default(),
        None,
        None,
        Some(vec![AppToken::USDC, AppToken::WBTC]),
    )?;

    assert_eq!(
        app.dex_adapter_query_config()?,
        Config {
            admin: AppUser::Admin.pubkey(),
            dex: Pubkey::default(),
            registry: None,
            is_paused: false,
            rotation_timeout: ROTATION_TIMEOUT,
            token_in_whitelist: [AppToken::USDC, AppToken::WBTC]
                .iter()
                .map(|y| y.pubkey())
                .collect()
        }
    );

    Ok(())
}

#[test]
fn swap_multihop() -> Result<()> {
    let mut app = App::new();
    prepare_dex(&mut app)?;

    app.dex_adapter_try_init(AppUser::Admin, app.program_id.clmm_mock, None, None, None)?;

    app.dex_adapter_try_save_route(
        AppUser::Admin,
        AppToken::WBTC,
        AppToken::PYTH,
        &[
            RouteItem {
                amm_index: AMM_CONFIG_INDEX_1,
                token_out: AppToken::USDC.pubkey(), // WBTC -> USDC (first hop output)
            },
            RouteItem {
                amm_index: AMM_CONFIG_INDEX_0,
                token_out: AppToken::PYTH.pubkey(), // USDC -> PYTH (second hop output)
            },
        ],
    )?;

    // swap WBTC -> USDC -> PYTH
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_pyth_before = app.get_balance(AppUser::Bob, AppToken::PYTH);

    app.dex_adapter_try_swap_multihop(
        AppUser::Bob,
        AppToken::WBTC,
        AppToken::PYTH,
        1_000,
        9_950_000,
    )?;

    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_pyth_after = app.get_balance(AppUser::Bob, AppToken::PYTH);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 1_000);
    assert_eq!(bob_pyth_after - bob_pyth_before, 9_960_020);

    Ok(())
}
