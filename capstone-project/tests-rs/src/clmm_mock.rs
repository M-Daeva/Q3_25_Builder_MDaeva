use {
    crate::helpers::{
        extensions::clmm_mock::{get_token_info_for_pool_creation, ClmmMockExtension},
        suite::{
            core::{token::WithTokenKeys, App},
            types::{AppToken, AppUser, GetDecimals, GetPrice},
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

    let (token_mint_0, token_mint_1) = get_token_info_for_pool_creation(&[
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

    app.clmm_mock_try_create_operation_account(AppUser::Admin)?;
    app.clmm_mock_try_create_amm_config(AppUser::Admin, AMM_CONFIG_INDEX, 1, 1, 1, 1)?;
    app.clmm_mock_try_create_pool(
        AppUser::Alice,
        1,
        app.get_clock_time() - 1,
        AMM_CONFIG_INDEX,
        &token_mint_0,
        &token_mint_1,
    )?;

    let alice_token_0_before = app.get_ata_token_balance(&AppUser::Alice.pubkey(), &token_mint_0);
    let alice_token_1_before = app.get_ata_token_balance(&AppUser::Alice.pubkey(), &token_mint_1);

    // TODO: set amounts based on price
    app.clmm_mock_try_open_position(
        AppUser::Alice,
        0,
        0,
        0,
        0,
        1,
        1_000_000,  // 1 USDC (6 decimals)
        10_000_000, // 10 PYTH (6 decimals)
        false,
        None,
        AMM_CONFIG_INDEX,
        &token_mint_0,
        &token_mint_1,
    )?;

    let alice_token_0_after = app.get_ata_token_balance(&AppUser::Alice.pubkey(), &token_mint_0);
    let alice_token_1_after = app.get_ata_token_balance(&AppUser::Alice.pubkey(), &token_mint_1);

    let amm_config = app.pda.clmm_mock_amm_config(AMM_CONFIG_INDEX);
    let pool_state = app
        .pda
        .clmm_mock_pool_state(amm_config, token_mint_0, token_mint_1);
    let token_vault_0 = app.pda.clmm_mock_token_vault_0(pool_state, token_mint_0);
    let token_vault_1 = app.pda.clmm_mock_token_vault_1(pool_state, token_mint_1);

    let token_vault_0_balance = app.get_pda_token_balance(&token_vault_0);
    let token_vault_1_balance = app.get_pda_token_balance(&token_vault_1);

    assert_eq!(
        alice_token_0_before - alice_token_0_after,
        token_vault_0_balance
    );
    assert_eq!(
        alice_token_1_before - alice_token_1_after,
        token_vault_1_balance
    );

    let bob_token_0_before = app.get_ata_token_balance(&AppUser::Bob.pubkey(), &token_mint_0);
    let bob_token_1_before = app.get_ata_token_balance(&AppUser::Bob.pubkey(), &token_mint_1);

    app.clmm_mock_try_swap(
        AppUser::Bob,
        1_000,
        100,
        0,
        true,
        AMM_CONFIG_INDEX,
        &token_mint_0,
        &token_mint_1,
    )?;

    let bob_token_0_after = app.get_ata_token_balance(&AppUser::Bob.pubkey(), &token_mint_0);
    let bob_token_1_after = app.get_ata_token_balance(&AppUser::Bob.pubkey(), &token_mint_1);

    assert_eq!(bob_token_0_before - bob_token_0_after, 1_000);
    assert_eq!(bob_token_1_after - bob_token_1_before, 9_960);

    Ok(())
}
