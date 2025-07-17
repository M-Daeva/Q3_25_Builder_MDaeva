use {
    crate::helpers::{
        amm::AmmExtension,
        suite::{
            core::{token::WithTokenKeys, App},
            types::{AppCoin, AppToken, AppUser},
        },
    },
    amm::state::{PoolBalance, PoolConfig},
    anchor_lang::Result,
    pretty_assertions::assert_eq,
    solana_pubkey::Pubkey,
};

#[test]
fn check_initial_token_balances() -> Result<()> {
    let app = App::new();

    let bob_pyth_balance = app.get_balance(AppUser::Bob, AppToken::PYTH)?;
    let bob_sol_balance = app.get_balance(AppUser::Bob, AppCoin::SOL)?;

    assert_eq!(bob_pyth_balance, 1_000_000_000);
    assert_eq!(bob_sol_balance, 1_000_000_000_000);

    Ok(())
}

#[test]
fn create_pool_and_provide_liquidity() -> Result<()> {
    const POOL_ID: u64 = 0;
    const FEE_BPS: u16 = 100;
    const MINT_X_AMOUNT: u64 = 2_000_000;
    const MINT_Y_AMOUNT: u64 = 8_000_000;
    const MINT_LP_AMOUNT: u64 = 4_000_000;

    let mut app = App::new();

    app.amm_try_create_pool(
        AppUser::Admin,
        POOL_ID,
        AppToken::USDC,
        AppToken::PYTH,
        FEE_BPS,
    )?;

    assert_eq!(
        app.amm_query_pool_config(POOL_ID)?,
        PoolConfig {
            config_bump: 254,
            balance_bump: 254,
            lp_bump: 255,
            id: POOL_ID,
            authority: Some(AppUser::Admin.pubkey()),
            mint_x: AppToken::USDC.pubkey(&app),
            mint_y: AppToken::PYTH.pubkey(&app),
            mint_lp: Pubkey::from_str_const("CbFuANF3F33Nda8QhWDXgzB4YFaBJECznD5XdiRHKHVQ"),
            fee_bps: FEE_BPS,
            is_locked: false,
        }
    );

    app.amm_try_provide_liquidity(AppUser::Alice, POOL_ID, MINT_X_AMOUNT, MINT_Y_AMOUNT)?;

    assert_eq!(
        app.amm_query_pool_balance(POOL_ID)?,
        PoolBalance {
            mint_x_amount: MINT_X_AMOUNT,
            mint_y_amount: MINT_Y_AMOUNT,
            mint_lp_amount: MINT_LP_AMOUNT
        }
    );
    assert_eq!(
        app.get_token_balance(
            &app.pda.amm_pool_config(POOL_ID),
            &AppToken::USDC.pubkey(&app)
        )?,
        MINT_X_AMOUNT
    );
    assert_eq!(
        app.get_token_balance(
            &app.pda.amm_pool_config(POOL_ID),
            &AppToken::PYTH.pubkey(&app)
        )?,
        MINT_Y_AMOUNT
    );
    assert_eq!(
        app.get_token_balance(
            &AppUser::Alice.pubkey(),
            &app.amm_query_pool_config(POOL_ID)?.mint_lp
        )?,
        MINT_LP_AMOUNT
    );

    Ok(())
}

#[test]
fn swap_default() -> Result<()> {
    const POOL_ID: u64 = 0;
    const FEE_BPS: u16 = 100;
    const MINT_X_AMOUNT: u64 = 2_000_000;
    const MINT_Y_AMOUNT: u64 = 8_000_000;
    const AMOUNT_IN: u64 = 1_000;
    const AMOUNT_OUT: u64 = 3_960;

    let mut app = App::new();

    app.amm_try_create_pool(
        AppUser::Admin,
        POOL_ID,
        AppToken::USDC,
        AppToken::PYTH,
        FEE_BPS,
    )?;

    app.amm_try_provide_liquidity(AppUser::Alice, POOL_ID, MINT_X_AMOUNT, MINT_Y_AMOUNT)?;

    let bob_usdc_before = app.get_balance(AppUser::Bob, AppToken::USDC)?;
    let bob_pyth_before = app.get_balance(AppUser::Bob, AppToken::PYTH)?;

    app.amm_try_swap(AppUser::Bob, POOL_ID, AMOUNT_IN, AppToken::USDC)?;

    let bob_usdc_after = app.get_balance(AppUser::Bob, AppToken::USDC)?;
    let bob_pyth_after = app.get_balance(AppUser::Bob, AppToken::PYTH)?;

    assert_eq!(bob_usdc_before - bob_usdc_after, AMOUNT_IN);
    // mint_y_amount ≈ (1 - 0.01) * 1_000 * (8_000_000 / 2_000_000) = 3_960
    assert_eq!(bob_pyth_after - bob_pyth_before, AMOUNT_OUT);

    Ok(())
}

#[test]
fn withdraw_liquidity() -> Result<()> {
    const POOL_ID: u64 = 0;
    const FEE_BPS: u16 = 100;
    const MINT_X_AMOUNT: u64 = 2_000_000;
    const MINT_Y_AMOUNT: u64 = 8_000_000;
    const MINT_LP_AMOUNT: u64 = 4_000_000;
    const AMOUNT_IN: u64 = 1_000;
    const AMOUNT_OUT: u64 = 3_960;

    let mut app = App::new();

    app.amm_try_create_pool(
        AppUser::Admin,
        POOL_ID,
        AppToken::USDC,
        AppToken::PYTH,
        FEE_BPS,
    )?;

    app.amm_try_provide_liquidity(AppUser::Alice, POOL_ID, MINT_X_AMOUNT, MINT_Y_AMOUNT)?;

    app.amm_try_swap(AppUser::Bob, POOL_ID, AMOUNT_IN, AppToken::USDC)?;
    app.amm_try_swap(AppUser::Bob, POOL_ID, AMOUNT_OUT, AppToken::PYTH)?;

    let alice_usdc_before = app.get_balance(AppUser::Alice, AppToken::USDC)?;
    let alice_pyth_before = app.get_balance(AppUser::Alice, AppToken::PYTH)?;

    app.amm_try_withdraw_liquidity(AppUser::Alice, POOL_ID, MINT_LP_AMOUNT)?;

    let alice_usdc_after = app.get_balance(AppUser::Alice, AppToken::USDC)?;
    let alice_pyth_after = app.get_balance(AppUser::Alice, AppToken::PYTH)?;

    assert_eq!(alice_usdc_after - alice_usdc_before, MINT_X_AMOUNT + 18);
    assert_eq!(alice_pyth_after - alice_pyth_before, MINT_Y_AMOUNT);

    Ok(())
}
