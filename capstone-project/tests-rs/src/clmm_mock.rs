use {
    crate::helpers::{
        extensions::clmm_mock::ClmmMockExtension,
        suite::{
            core::{token::WithTokenKeys, App},
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    pretty_assertions::assert_eq,
};

#[test]
fn swap_default() -> Result<()> {
    const ID: u8 = 0;
    const DECIMALS_MULTIPLIER: u64 = 1_000_000;
    const AMOUNT_A: u64 = 10 * DECIMALS_MULTIPLIER;
    const AMOUNT_B: u64 = 10 * AMOUNT_A;

    let mut app = App::new();

    let mint_a = &AppToken::USDC.pubkey(&app);
    let mint_b = &AppToken::PYTH.pubkey(&app);

    app.clmm_mock_try_create_pool(
        AppUser::Admin,
        ID,
        (AMOUNT_A, AppToken::USDC),
        (AMOUNT_B, AppToken::PYTH),
    )?;

    let pool_state_pda = &app.pda.clmm_mock_pool_state(ID);

    assert_eq!(app.get_token_balance(pool_state_pda, mint_a), AMOUNT_A);
    assert_eq!(app.get_token_balance(pool_state_pda, mint_b), AMOUNT_B);

    Ok(())
}
