use {
    crate::helpers::{
        extensions::clmm::{get_token_info_for_pool_creation, ClmmExtension},
        suite::{
            core::{token::WithTokenKeys, App},
            types::{AppToken, AppUser, GetDecimals, GetPrice},
        },
    },
    anchor_lang::Result,
};

#[test]
fn swap_default() -> Result<()> {
    const AMM_CONFIG_INDEX: u16 = 0;

    let mut app = App::new();
    app.wait(1_000);

    app.clmm_try_create_operation_account(AppUser::Admin)?;
    let _operation_account = app.clmm_query_operation_account()?;

    // https://explorer.solana.com/address/9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x/anchor-account
    app.clmm_try_create_amm_config(AppUser::Admin, AMM_CONFIG_INDEX, 1, 100, 120_000, 40_000)?;
    let _amm_config = app.clmm_query_amm_config(AMM_CONFIG_INDEX)?;

    let (token_mint_0, token_mint_1, sqrt_price_x64) = get_token_info_for_pool_creation(&[
        (
            AppToken::USDC.pubkey(&app),
            AppToken::USDC.get_decimals(),
            AppToken::USDC.get_price(),
        ),
        (
            AppToken::PYTH.pubkey(&app),
            AppToken::PYTH.get_decimals(),
            AppToken::PYTH.get_price(),
        ),
    ]);

    app.clmm_try_create_pool(
        AppUser::Alice,
        sqrt_price_x64,
        app.get_clock_time() - 1,
        AMM_CONFIG_INDEX,
        &token_mint_0,
        &token_mint_1,
    )?;
    let _pool_state = app.clmm_query_pool_state(
        &app.pda.clmm_amm_config(AMM_CONFIG_INDEX),
        &token_mint_0,
        &token_mint_1,
    )?;

    app.clmm_try_open_position(
        AppUser::Alice,
        -1000,         // Lower price boundary
        1000,          // Upper price boundary
        -1024,         // Tick array containing lower tick
        0,             // Tick array containing upper tick
        1_000_000_000, // 1B units of liquidity
        1_000_000,     // Max 1 USDC (6 decimals)
        100_000_000,   // Max 0.1 SOL (9 decimals)
        true,          // Create NFT with metadata
        None,          // Use exact liquidity amount
        AMM_CONFIG_INDEX,
        &token_mint_0,
        &token_mint_1,
    )?;

    Ok(())
}
