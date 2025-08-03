use {
    crate::helpers::{
        extensions::clmm::ClmmExtension,
        suite::{core::App, types::AppUser},
    },
    anchor_lang::Result,
};

#[test]
fn operation_account_default() -> Result<()> {
    let mut app = App::new();

    // account doesn't exist
    app.clmm_query_operation_account().unwrap_err();

    app.clmm_try_create_operation_account(AppUser::Admin)?;

    // account is created
    app.clmm_query_operation_account()?;

    Ok(())
}
