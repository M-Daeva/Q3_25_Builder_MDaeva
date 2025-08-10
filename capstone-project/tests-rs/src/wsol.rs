use {
    crate::helpers::{
        extensions::wsol::WsolExtension,
        suite::{
            core::App,
            types::{AppCoin, AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    solana_program::native_token::LAMPORTS_PER_SOL,
};

#[test]
fn wrap_unwrap() -> Result<()> {
    const AMOUNT: u64 = 100;

    let mut app = App::new();

    let sol_before = app.get_balance(AppUser::Alice, AppCoin::SOL);
    let wsol_before = app.get_balance(AppUser::Alice, AppToken::WSOL);

    app.wsol_try_wrap(AppUser::Alice, AMOUNT * LAMPORTS_PER_SOL)?;

    let sol_after = app.get_balance(AppUser::Alice, AppCoin::SOL);
    let wsol_after = app.get_balance(AppUser::Alice, AppToken::WSOL);

    assert_eq!((wsol_after - wsol_before) / LAMPORTS_PER_SOL, AMOUNT);
    assert_eq!((sol_before - sol_after) / LAMPORTS_PER_SOL, AMOUNT);

    let sol_before = app.get_balance(AppUser::Alice, AppCoin::SOL);
    let wsol_before = app.get_balance(AppUser::Alice, AppToken::WSOL);

    app.wsol_try_unwrap(AppUser::Alice)?;

    let sol_after = app.get_balance(AppUser::Alice, AppCoin::SOL);
    let wsol_after = app.get_balance(AppUser::Alice, AppToken::WSOL);

    assert_eq!((wsol_before - wsol_after) / LAMPORTS_PER_SOL, AMOUNT);
    assert_eq!((sol_after - sol_before) / LAMPORTS_PER_SOL, AMOUNT);

    Ok(())
}
