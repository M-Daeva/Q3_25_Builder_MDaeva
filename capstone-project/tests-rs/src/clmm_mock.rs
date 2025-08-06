use {
    crate::helpers::{
        extensions::clmm_mock::{calc_token_amount_for_pool, ClmmMockExtension},
        suite::{
            core::App,
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    pretty_assertions::assert_eq,
};

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
        true,   // exact input (we know exactly how much USDC we want to swap)
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
        AppToken::WBTC,
    )?;

    app.clmm_mock_try_open_position(
        AppUser::Alice,
        0,
        0,
        0,
        0,
        1,
        calc_token_amount_for_pool(AppToken::USDC),
        calc_token_amount_for_pool(AppToken::WBTC),
        false,
        None,
        AMM_CONFIG_INDEX,
        AppToken::USDC,
        AppToken::WBTC,
    )?;

    // swap USDC -> WBTC
    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wbtc_before = app.get_balance(AppUser::Bob, AppToken::WBTC);

    app.clmm_mock_try_swap(
        AppUser::Bob,
        1_000_000, // amount (USDC amount)
        995,       // min output threshold
        0,         // no price limit
        true,      // exact input (we know exactly how much USDC we want to swap)
        AMM_CONFIG_INDEX,
        AppToken::USDC, // USDC mint (input)
        AppToken::WBTC, // WBTC mint (output)
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
        995_000, // min output threshold
        0,       // no price limit
        true,    // exact input (we know exactly how much USDC we want to swap)
        AMM_CONFIG_INDEX,
        AppToken::WBTC, // USDC mint (input)
        AppToken::USDC, // PYTH mint (output)
    )?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC);
    let bob_wbtc_after = app.get_balance(AppUser::Bob, AppToken::WBTC);

    assert_eq!(bob_wbtc_before - bob_wbtc_after, 1_000);
    assert_eq!(bob_usdc_after - bob_usdc_before, 998_000);

    Ok(())
}
