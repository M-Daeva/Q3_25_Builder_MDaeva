use {
    crate::helpers::{
        extensions::clmm_mock::{calc_token_amount_for_pool, sort_tokens, ClmmMockExtension},
        suite::{
            core::App,
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    pretty_assertions::assert_eq,
};

const AMM_CONFIG_INDEX_0: u16 = 0;
const AMM_CONFIG_INDEX_1: u16 = 1;

fn prepare_dex(app: &mut App) -> Result<()> {
    app.wait(1_000);
    app.clmm_mock_try_create_operation_account(AppUser::Admin)?;

    for (amm_index, token_0, token_1) in [
        (AMM_CONFIG_INDEX_0, AppToken::USDC, AppToken::PYTH),
        (AMM_CONFIG_INDEX_1, AppToken::WBTC, AppToken::USDC),
    ] {
        let (token_0, token_1) = sort_tokens(token_0, token_1);

        app.clmm_mock_try_create_amm_config(AppUser::Admin, amm_index, 1, 1, 1, 1)?;
        app.clmm_mock_try_create_pool(
            AppUser::Admin,
            1,
            app.get_clock_time() - 1,
            amm_index,
            token_0,
            token_1,
        )?;
        app.clmm_mock_try_open_position(
            AppUser::Admin,
            0,
            0,
            0,
            0,
            1,
            calc_token_amount_for_pool(token_0),
            calc_token_amount_for_pool(token_1),
            false,
            None,
            amm_index,
            token_0,
            token_1,
        )?;
    }

    Ok(())
}

#[test]
fn swap_default() -> Result<()> {
    const AMM_CONFIG_INDEX: u16 = 0;

    let mut app = App::new();
    app.wait(1_000);

    app.clmm_mock_try_create_operation_account(AppUser::Admin)?;
    app.clmm_mock_try_create_amm_config(AppUser::Admin, AMM_CONFIG_INDEX, 1, 1, 1, 1)?;
    app.clmm_mock_try_create_pool(
        AppUser::Alice,
        1,
        app.get_clock_time() - 1,
        AMM_CONFIG_INDEX,
        AppToken::USDC,
        AppToken::PYTH,
    )?;

    let alice_usdc_before = app.get_balance(AppUser::Alice, AppToken::USDC);
    let alice_pyth_before = app.get_balance(AppUser::Alice, AppToken::PYTH);

    app.clmm_mock_try_open_position(
        AppUser::Alice,
        0,
        0,
        0,
        0,
        1,
        calc_token_amount_for_pool(AppToken::USDC),
        calc_token_amount_for_pool(AppToken::PYTH),
        false,
        None,
        AMM_CONFIG_INDEX,
        AppToken::USDC,
        AppToken::PYTH,
    )?;

    let alice_usdc_after = app.get_balance(AppUser::Alice, AppToken::USDC);
    let alice_pyth_after = app.get_balance(AppUser::Alice, AppToken::PYTH);

    let amm_config = app.pda.clmm_mock_amm_config(AMM_CONFIG_INDEX);
    let pool_state =
        app.pda
            .clmm_mock_pool_state(amm_config, AppToken::USDC.pubkey(), AppToken::PYTH.pubkey());
    let token_vault_0 = app
        .pda
        .clmm_mock_token_vault_0(pool_state, AppToken::USDC.pubkey());
    let token_vault_1 = app
        .pda
        .clmm_mock_token_vault_1(pool_state, AppToken::PYTH.pubkey());

    let token_vault_0_balance = app.get_pda_token_balance(&token_vault_0);
    let token_vault_1_balance = app.get_pda_token_balance(&token_vault_1);

    assert_eq!(alice_usdc_before - alice_usdc_after, token_vault_0_balance);
    assert_eq!(alice_pyth_before - alice_pyth_after, token_vault_1_balance);

    // swap USDC -> PYTH
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_pyth_before = app.get_balance(AppUser::Bob, AppToken::PYTH);

    app.clmm_mock_try_swap(
        AppUser::Bob,
        100_000, // amount (USDC amount)
        995_000, // min output threshold
        0,       // no price limit
        true,    // exact input (we know exactly how much USDC we want to swap)
        AMM_CONFIG_INDEX,
        AppToken::USDC, // USDC mint (input)
        AppToken::PYTH, // PYTH mint (output)
    )?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_pyth_after = app.get_balance(AppUser::Bob, AppToken::PYTH);

    assert_eq!(bob_usdc_before - bob_usdc_after, 100_000);
    assert_eq!(bob_pyth_after - bob_pyth_before, 997_999);

    // swap PYTH -> USDC
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_pyth_before = app.get_balance(AppUser::Bob, AppToken::PYTH);

    app.clmm_mock_try_swap(
        AppUser::Bob,
        1_000_000,
        99_500, // min output threshold
        0,      // no price limit
        true,   // exact input (we know exactly how much PYTH we want to swap)
        AMM_CONFIG_INDEX,
        AppToken::PYTH, // USDC mint (input)
        AppToken::USDC, // PYTH mint (output)
    )?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_pyth_after = app.get_balance(AppUser::Bob, AppToken::PYTH);

    assert_eq!(bob_pyth_before - bob_pyth_after, 1_000_000);
    assert_eq!(bob_usdc_after - bob_usdc_before, 99_800);

    Ok(())
}

#[test]
fn swap_with_decimals() -> Result<()> {
    let mut app = App::new();
    prepare_dex(&mut app)?;

    // swap USDC -> WBTC
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);

    app.clmm_mock_try_swap(
        AppUser::Bob,
        1_000_000,
        995,
        0,
        true,
        AMM_CONFIG_INDEX_1,
        AppToken::USDC,
        AppToken::WBTC,
    )?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);

    assert_eq!(bob_usdc_before - bob_usdc_after, 1_000_000);
    // 1_000_000 usdc = 1 $ = 0.00001 WBTC = 1_000 wbtc
    assert_eq!(bob_wbtc_after - bob_wbtc_before, 997);

    // swap WBTC -> USDC
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);

    app.clmm_mock_try_swap(
        AppUser::Bob,
        1_000,
        995_000,
        0,
        true,
        AMM_CONFIG_INDEX_1,
        AppToken::WBTC,
        AppToken::USDC,
    )?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 1_000);
    assert_eq!(bob_usdc_after - bob_usdc_before, 998_000);

    Ok(())
}

#[test]
fn swap_multihop_default() -> Result<()> {
    let mut app = App::new();
    prepare_dex(&mut app)?;

    // swap WBTC -> USDC -> PYTH
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_pyth_before = app.get_balance(AppUser::Bob, AppToken::PYTH);

    app.clmm_mock_try_swap_multihop(
        AppUser::Bob,
        1_000,
        9_950_000,
        &[
            (AppToken::WBTC, 42),                 // unused
            (AppToken::USDC, AMM_CONFIG_INDEX_1), // WBTC -> USDC uses config_1
            (AppToken::PYTH, AMM_CONFIG_INDEX_0), // USDC -> PYTH uses config_0
        ],
    )?;

    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_pyth_after = app.get_balance(AppUser::Bob, AppToken::PYTH);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 1_000);
    assert_eq!(bob_pyth_after - bob_pyth_before, 9_960_020);

    Ok(())
}

#[test]
fn swap_multihop_reversed() -> Result<()> {
    let mut app = App::new();
    prepare_dex(&mut app)?;

    // swap PYTH -> USDC -> WBTC
    let bob_pyth_before = app.get_balance(AppUser::Bob, AppToken::PYTH);
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);

    app.clmm_mock_try_swap_multihop(
        AppUser::Bob,
        10_000_000,
        1,
        &[
            (AppToken::PYTH, AMM_CONFIG_INDEX_0),
            (AppToken::USDC, AMM_CONFIG_INDEX_0),
            (AppToken::WBTC, AMM_CONFIG_INDEX_1),
        ],
    )?;

    let bob_pyth_after = app.get_balance(AppUser::Bob, AppToken::PYTH);
    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);

    assert_eq!(bob_pyth_before - bob_pyth_after, 10_000_000);
    assert_eq!(bob_wbtc_after - bob_wbtc_before, 996);

    Ok(())
}

#[test]
fn swap_multihop_single_pool() -> Result<()> {
    let mut app = App::new();
    prepare_dex(&mut app)?;

    // swap WBTC -> USDC
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);

    app.clmm_mock_try_swap_multihop(
        AppUser::Bob,
        1_000,
        0,
        &[
            (AppToken::WBTC, AMM_CONFIG_INDEX_1),
            (AppToken::USDC, AMM_CONFIG_INDEX_1),
        ],
    )?;

    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 1_000);
    assert_eq!(bob_usdc_after - bob_usdc_before, 997_999);

    Ok(())
}

#[test]
fn swap_multihop_same_config() -> Result<()> {
    const AMM_CONFIG_INDEX_0: u16 = 0;

    let mut app = App::new();
    app.wait(1_000);
    app.clmm_mock_try_create_operation_account(AppUser::Admin)?;
    app.clmm_mock_try_create_amm_config(AppUser::Admin, AMM_CONFIG_INDEX_0, 1, 1, 1, 1)?;

    for (amm_index, token_0, token_1) in [
        (AMM_CONFIG_INDEX_0, AppToken::USDC, AppToken::PYTH),
        (AMM_CONFIG_INDEX_0, AppToken::WBTC, AppToken::USDC),
    ] {
        let (token_0, token_1) = sort_tokens(token_0, token_1);

        app.clmm_mock_try_create_pool(
            AppUser::Admin,
            1,
            app.get_clock_time() - 1,
            amm_index,
            token_0,
            token_1,
        )?;
        app.clmm_mock_try_open_position(
            AppUser::Admin,
            0,
            0,
            0,
            0,
            1,
            calc_token_amount_for_pool(token_0),
            calc_token_amount_for_pool(token_1),
            false,
            None,
            amm_index,
            token_0,
            token_1,
        )?;
    }

    // swap WBTC -> USDC -> PYTH
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_pyth_before = app.get_balance(AppUser::Bob, AppToken::PYTH);

    app.clmm_mock_try_swap_multihop(
        AppUser::Bob,
        1_000,
        9_950_000,
        &[
            (AppToken::WBTC, 42),                 // unused
            (AppToken::USDC, AMM_CONFIG_INDEX_0), // WBTC -> USDC uses config_1
            (AppToken::PYTH, AMM_CONFIG_INDEX_0), // USDC -> PYTH uses config_0
        ],
    )?;

    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);
    let bob_pyth_after = app.get_balance(AppUser::Bob, AppToken::PYTH);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 1_000);
    assert_eq!(bob_pyth_after - bob_pyth_before, 9_960_020);

    Ok(())
}
