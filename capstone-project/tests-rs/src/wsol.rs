use {
    crate::helpers::{
        extensions::wsol::WsolExtension,
        suite::{
            core::App,
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
};

#[test]
fn wrap_unwrap() -> Result<()> {
    const AMOUNT: u64 = 1_00;

    let mut app = App::new();

    assert_eq!(app.get_balance(AppUser::Alice, AppToken::WSOL), 0);

    app.wsol_try_wrap(AppUser::Alice, AMOUNT)?;
    assert_eq!(app.get_balance(AppUser::Alice, AppToken::WSOL), AMOUNT);

    app.wsol_try_unwrap(AppUser::Alice)?;
    assert_eq!(app.get_balance(AppUser::Alice, AppToken::WSOL), 0);

    Ok(())
}
