use {
    crate::helpers::{
        extensions::clmm_mock::{
            calc_token_amount_for_pool, get_token_info_for_pool_creation, ClmmMockExtension,
        },
        suite::{
            core::App,
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

    let token_info_for_pool_creation = get_token_info_for_pool_creation(&[
        (
            AppToken::USDC.pubkey(),
            AppToken::USDC.get_decimals(),
            AppToken::USDC.get_price(),
        ),
        (
            AppToken::PYTH.pubkey(),
            AppToken::PYTH.get_decimals(),
            AppToken::PYTH.get_price(),
        ),
    ]);
    let (token_mint_0, token_decimals_0, token_price_0) = token_info_for_pool_creation[0];
    let (token_mint_1, token_decimals_1, token_price_1) = token_info_for_pool_creation[1];

    let token_amount_0 = calc_token_amount_for_pool(token_decimals_0, token_price_0);
    let token_amount_1 = calc_token_amount_for_pool(token_decimals_1, token_price_1);

    println!("usdc {:#?}", AppToken::USDC.pubkey());
    println!("token_amount_0 {:#?}", (token_amount_0, token_mint_0));
    println!("token_amount_1 {:#?}", (token_amount_1, token_mint_1));

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

    app.clmm_mock_try_open_position(
        AppUser::Alice,
        0,
        0,
        0,
        0,
        1,
        token_amount_0,
        token_amount_1,
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

    let input_is_usdc = token_price_0 > token_price_1; // USDC ($1) > PYTH ($0.1)

    // determine which accounts to use based on which token is USDC
    let (input_token_mint, output_token_mint) = if input_is_usdc {
        (&token_mint_0, &token_mint_1) // USDC is token_0, PYTH is token_1
    } else {
        (&token_mint_1, &token_mint_0) // USDC is token_1, PYTH is token_0
    };

    let bob_token_in_before = app.get_ata_token_balance(&AppUser::Bob.pubkey(), &input_token_mint);
    let bob_token_out_before =
        app.get_ata_token_balance(&AppUser::Bob.pubkey(), &output_token_mint);

    app.clmm_mock_try_swap(
        AppUser::Bob,
        100_000, // amount (USDC amount)
        100,     // min output threshold
        0,       // no price limit
        true,    // exact input (we know exactly how much USDC we want to swap)
        AMM_CONFIG_INDEX,
        input_token_mint,  // USDC mint (input)
        output_token_mint, // PYTH mint (output)
    )?;

    let bob_token_in_after = app.get_ata_token_balance(&AppUser::Bob.pubkey(), &input_token_mint);
    let bob_token_out_after = app.get_ata_token_balance(&AppUser::Bob.pubkey(), &output_token_mint);

    assert_eq!(bob_token_in_before - bob_token_in_after, 100_000);
    assert_eq!(bob_token_out_after - bob_token_out_before, 997_999); // sometimes 9_979

    Ok(())
}
