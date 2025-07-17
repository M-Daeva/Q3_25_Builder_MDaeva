use {
    crate::helpers::{
        amm::AmmExtension,
        suite::{
            core::{token::WithTokenKeys, App},
            types::{AppCoin, AppToken, AppUser},
        },
    },
    amm::state::PoolConfig,
    anchor_lang::Result,
    pretty_assertions::assert_eq,
    solana_pubkey::Pubkey,
};

#[test]
fn default() -> Result<()> {
    let mut app = App::new();

    let bob_pyth_balance = app.get_balance(AppUser::Bob, AppToken::PYTH)?;
    let bob_sol_balance = app.get_balance(AppUser::Bob, AppCoin::SOL)?;

    assert_eq!(bob_pyth_balance, 1_000_000_000);
    assert_eq!(bob_sol_balance, 1_000_000_000_000);

    let pool_id = 0;
    app.amm_try_create_pool(AppUser::Admin, pool_id, AppToken::USDC, AppToken::PYTH, 300)?;

    let pool_config = app.amm_query_pool_config(pool_id)?;
    assert_eq!(
        pool_config,
        PoolConfig {
            config_bump: 254,
            balance_bump: 254,
            lp_bump: 255,
            id: 0,
            authority: Some(AppUser::Admin.pubkey()),
            mint_x: AppToken::USDC.pubkey(&app),
            mint_y: AppToken::PYTH.pubkey(&app),
            mint_lp: Pubkey::from_str_const("CbFuANF3F33Nda8QhWDXgzB4YFaBJECznD5XdiRHKHVQ"),
            fee_bps: 300,
            is_locked: false,
        }
    );

    Ok(())
}
