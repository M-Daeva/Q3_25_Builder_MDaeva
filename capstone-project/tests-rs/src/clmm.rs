use {
    crate::helpers::{
        extensions::clmm::ClmmExtension,
        suite::{core::App, types::AppUser},
    },
    anchor_lang::Result,
};

#[test]
fn operation_account_default() -> Result<()> {
    const AMM_CONFIG_INDEX: u16 = 0;

    let mut app = App::new();

    // account doesn't exist
    app.clmm_query_operation_account().unwrap_err();

    app.clmm_try_create_operation_account(AppUser::Admin)?;

    // account is created
    let _operation_account = app.clmm_query_operation_account()?;

    // https://explorer.solana.com/address/9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x/anchor-account
    app.clmm_try_create_amm_config(AppUser::Admin, AMM_CONFIG_INDEX, 1, 100, 120_000, 40_000)?;

    let _amm_config = app.clmm_query_amm_config(AMM_CONFIG_INDEX)?;

    Ok(())
}
