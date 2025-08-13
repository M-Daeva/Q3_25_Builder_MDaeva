use {
    crate::{
        clmm_mock::{prepare_dex, AMM_CONFIG_INDEX_0, AMM_CONFIG_INDEX_1},
        helpers::{
            extensions::{
                dex_adapter::DexAdapterExtension, registry::RegistryExtension, wsol::WsolExtension,
            },
            suite::{
                core::App,
                types::{AppCoin, AppToken, AppUser},
            },
        },
    },
    anchor_lang::Result,
    dex_adapter_cpi::{
        state::{DaConfig, ROTATION_TIMEOUT},
        types::RouteItem,
    },
    pretty_assertions::assert_eq,
    registry_cpi::{state::ACCOUNT_REGISTRATION_FEE_AMOUNT, types::AssetItem},
    solana_program::native_token::LAMPORTS_PER_SOL,
    solana_pubkey::Pubkey,
};

#[test]
fn init_default() -> Result<()> {
    let mut app = App::new();

    app.dex_adapter_try_init(AppUser::Admin, Pubkey::default(), None, None)?;

    assert_eq!(
        app.dex_adapter_query_config()?,
        DaConfig {
            admin: AppUser::Admin.pubkey(),
            dex: Pubkey::default(),
            registry: None,
            is_paused: false,
            rotation_timeout: ROTATION_TIMEOUT,
        }
    );

    Ok(())
}

#[test]
fn swap_multihop() -> Result<()> {
    let mut app = App::new();
    prepare_dex(
        &mut app,
        &[
            (AMM_CONFIG_INDEX_0, AppToken::USDC, AppToken::PYTH),
            (AMM_CONFIG_INDEX_1, AppToken::WBTC, AppToken::USDC),
        ],
        None,
    )?;

    app.dex_adapter_try_init(AppUser::Admin, app.program_id.clmm_mock, None, None)?;
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

#[test]
fn swap_multihop_2() -> Result<()> {
    let mut app = App::new();
    prepare_dex(
        &mut app,
        &[
            (AMM_CONFIG_INDEX_0, AppToken::WBTC, AppToken::PYTH),
            (AMM_CONFIG_INDEX_1, AppToken::PYTH, AppToken::USDC),
        ],
        None,
    )?;

    app.dex_adapter_try_init(AppUser::Admin, app.program_id.clmm_mock, None, None)?;
    app.dex_adapter_try_save_route(
        AppUser::Admin,
        AppToken::WBTC,
        AppToken::USDC,
        &[
            RouteItem {
                amm_index: AMM_CONFIG_INDEX_0,
                token_out: AppToken::PYTH.pubkey(),
            },
            RouteItem {
                amm_index: AMM_CONFIG_INDEX_1,
                token_out: AppToken::USDC.pubkey(),
            },
        ],
    )?;

    // swap WBTC -> PYTH -> USDC
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);

    app.dex_adapter_try_swap_multihop(
        AppUser::Bob,
        AppToken::WBTC,
        AppToken::USDC,
        1_000,
        995_000,
    )?;

    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 1_000);
    assert_eq!(bob_usdc_after - bob_usdc_before, 996_002);

    Ok(())
}

#[test]
fn swap_and_activate_default() -> Result<()> {
    const MAX_DATA_SIZE_0: u32 = 1_000;

    let mut app = App::new();
    prepare_dex(
        &mut app,
        &[
            (AMM_CONFIG_INDEX_0, AppToken::WBTC, AppToken::PYTH),
            (AMM_CONFIG_INDEX_1, AppToken::PYTH, AppToken::USDC),
        ],
        None,
    )?;

    app.registry_try_init(
        AppUser::Admin,
        None,
        Some(AssetItem {
            amount: ACCOUNT_REGISTRATION_FEE_AMOUNT,
            asset: AppToken::USDC.pubkey(),
        }),
        None,
    )?;

    app.registry_try_create_account(AppUser::Bob, MAX_DATA_SIZE_0, None)?;

    app.dex_adapter_try_init(AppUser::Admin, app.program_id.clmm_mock, None, None)?;
    app.dex_adapter_try_save_route(
        AppUser::Admin,
        AppToken::WBTC,
        AppToken::USDC,
        &[
            RouteItem {
                amm_index: AMM_CONFIG_INDEX_0,
                token_out: AppToken::PYTH.pubkey(),
            },
            RouteItem {
                amm_index: AMM_CONFIG_INDEX_1,
                token_out: AppToken::USDC.pubkey(),
            },
        ],
    )?;

    // swap WBTC -> PYTH -> USDC
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);

    app.dex_adapter_try_swap_and_activate(
        AppUser::Bob,
        AppToken::WBTC,
        AppToken::USDC,
        11_000,
        app.registry_query_config()?.registration_fee.amount,
    )?;

    assert_eq!(app.registry_query_user_id(AppUser::Bob)?.is_activated, true);

    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 11_000);
    assert_eq!(bob_usdc_after - bob_usdc_before, 955_803);

    Ok(())
}

#[test]
fn swap_to_wsol_default() -> Result<()> {
    const BASE_AMOUNT: u128 = 10_000;

    let mut app = App::new();
    app.wsol_try_wrap(AppUser::Admin, BASE_AMOUNT as u64 * LAMPORTS_PER_SOL)?;
    prepare_dex(
        &mut app,
        &[(AMM_CONFIG_INDEX_0, AppToken::USDC, AppToken::WSOL)],
        Some(BASE_AMOUNT),
    )?;

    app.dex_adapter_try_init(AppUser::Admin, app.program_id.clmm_mock, None, None)?;
    app.dex_adapter_try_save_route(
        AppUser::Admin,
        AppToken::USDC,
        AppToken::WSOL,
        &[RouteItem {
            amm_index: AMM_CONFIG_INDEX_0,
            token_out: AppToken::WSOL.pubkey(),
        }],
    )?;

    // swap USDC -> WSOL
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wsol_before = app.get_balance(AppUser::Bob, AppToken::WSOL);

    app.dex_adapter_try_swap_multihop(AppUser::Bob, AppToken::USDC, AppToken::WSOL, 1_000, 1)?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wsol_after = app.get_balance(AppUser::Bob, AppToken::WSOL);

    assert_eq!(bob_usdc_before - bob_usdc_after, 1_000);
    assert_eq!(bob_wsol_after - bob_wsol_before, 9_979);

    Ok(())
}

#[test]
fn swap_and_unwrap_wsol_default() -> Result<()> {
    const BASE_AMOUNT: u128 = 10_000;

    let mut app = App::new();
    app.wsol_try_wrap(AppUser::Admin, BASE_AMOUNT as u64 * LAMPORTS_PER_SOL)?;
    prepare_dex(
        &mut app,
        &[(AMM_CONFIG_INDEX_0, AppToken::USDC, AppToken::WSOL)],
        Some(BASE_AMOUNT),
    )?;

    app.dex_adapter_try_init(AppUser::Admin, app.program_id.clmm_mock, None, None)?;
    app.dex_adapter_try_save_route(
        AppUser::Admin,
        AppToken::USDC,
        AppToken::WSOL,
        &[RouteItem {
            amm_index: AMM_CONFIG_INDEX_0,
            token_out: AppToken::WSOL.pubkey(),
        }],
    )?;

    // swap USDC -> SOL
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wsol_before = app.get_balance(AppUser::Bob, AppToken::WSOL);
    let bob_sol_before = app.get_balance(AppUser::Bob, AppCoin::SOL);

    app.dex_adapter_try_swap_and_unwrap_wsol(
        AppUser::Bob,
        AppToken::USDC,
        AppToken::WSOL,
        1_000_000,
        1,
    )?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wsol_after = app.get_balance(AppUser::Bob, AppToken::WSOL);
    let bob_sol_after = app.get_balance(AppUser::Bob, AppCoin::SOL);

    assert_eq!(bob_usdc_before - bob_usdc_after, 1_000_000);
    assert_eq!(bob_wsol_after - bob_wsol_before, 0);
    assert_eq!(bob_sol_after - bob_sol_before, 9_969_004);

    Ok(())
}
