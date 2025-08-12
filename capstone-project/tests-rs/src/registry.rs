use {
    crate::helpers::{
        extensions::registry::RegistryExtension,
        suite::{
            core::{assert_error, App},
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    base::error::AuthError,
    pretty_assertions::assert_eq,
    registry_cpi::{
        state::{
            Config, UserAccount, ACCOUNT_DATA_SIZE_MAX, ACCOUNT_DATA_SIZE_MIN,
            ACCOUNT_REGISTRATION_FEE_AMOUNT, CLOCK_TIME_MIN, ROTATION_TIMEOUT,
        },
        types::{AssetItem, Range},
    },
};

fn init_app() -> Result<App> {
    let mut app = App::new();

    app.registry_try_init(
        AppUser::Admin,
        None,
        Some(AssetItem {
            amount: ACCOUNT_REGISTRATION_FEE_AMOUNT,
            asset: AppToken::USDC.pubkey(),
        }),
        None,
    )?;

    Ok(app)
}

#[test]
fn init_default() -> Result<()> {
    let app = init_app()?;

    assert_eq!(
        app.registry_query_config()?,
        Config {
            admin: AppUser::Admin.pubkey(),
            is_paused: false,
            rotation_timeout: ROTATION_TIMEOUT,
            registration_fee: AssetItem {
                amount: ACCOUNT_REGISTRATION_FEE_AMOUNT,
                asset: AppToken::USDC.pubkey(),
            },
            data_size_range: Range {
                min: ACCOUNT_DATA_SIZE_MIN,
                max: ACCOUNT_DATA_SIZE_MAX,
            }
        }
    );

    Ok(())
}

#[test]
fn init_admin_guard() -> Result<()> {
    let mut app = App::new();
    app.wait(CLOCK_TIME_MIN + 1);

    let res = app
        .registry_try_init(
            AppUser::Admin,
            None,
            Some(AssetItem {
                amount: ACCOUNT_REGISTRATION_FEE_AMOUNT,
                asset: AppToken::USDC.pubkey(),
            }),
            None,
        )
        .unwrap_err();
    assert_error(res, AuthError::Unauthorized);

    Ok(())
}

#[test]
fn transfer_admin() -> Result<()> {
    let mut app = init_app()?;

    let res = app
        .registry_try_update_config(AppUser::Alice, Some(AppUser::Alice), None, None, None, None)
        .unwrap_err();
    assert_error(res, AuthError::Unauthorized);

    let res = app
        .registry_try_confirm_admin_rotation(AppUser::Alice)
        .unwrap_err();
    assert_error(res, AuthError::NoNewOwner);

    app.registry_try_update_config(AppUser::Admin, Some(AppUser::Alice), None, None, None, None)?;

    app.wait(ROTATION_TIMEOUT as u64);
    let res = app
        .registry_try_confirm_admin_rotation(AppUser::Alice)
        .unwrap_err();
    assert_error(res, AuthError::TransferOwnerDeadline);

    app.registry_try_update_config(AppUser::Admin, Some(AppUser::Alice), None, None, None, None)?;

    let res = app
        .registry_try_confirm_admin_rotation(AppUser::Bob)
        .unwrap_err();
    assert_error(res, AuthError::Unauthorized);

    app.registry_try_confirm_admin_rotation(AppUser::Alice)?;
    assert_eq!(app.registry_query_config()?.admin, AppUser::Alice.pubkey());

    Ok(())
}

#[test]
fn create_and_activate_account_default() -> Result<()> {
    const MAX_DATA_SIZE: u32 = 1_000;

    let mut app = init_app()?;

    app.registry_try_create_account(AppUser::Alice, MAX_DATA_SIZE)?;

    let user_id = app.registry_query_user_id(AppUser::Alice)?;
    assert_eq!(user_id.id, 1);
    assert_eq!(user_id.is_open, true);
    assert_eq!(user_id.is_activated, false);

    assert_eq!(
        app.registry_query_user_account(user_id.id)?,
        UserAccount {
            data: String::default(),
            nonce: 0,
            max_size: MAX_DATA_SIZE
        }
    );

    let alice_usdc_before = app.get_balance(AppUser::Alice, AppToken::USDC);

    app.registry_try_activate_account(AppUser::Alice, None, None)?;

    let alice_usdc_after = app.get_balance(AppUser::Alice, AppToken::USDC);
    assert_eq!(
        alice_usdc_before - alice_usdc_after,
        ACCOUNT_REGISTRATION_FEE_AMOUNT
    );

    let user_id = app.registry_query_user_id(AppUser::Alice)?;
    assert_eq!(user_id.id, 1);
    assert_eq!(user_id.is_open, true);
    assert_eq!(user_id.is_activated, true);

    Ok(())
}

#[test]
fn withdraw_revenue_default() -> Result<()> {
    const MAX_DATA_SIZE: u32 = 1_000;

    let mut app = init_app()?;

    app.registry_try_create_account(AppUser::Alice, MAX_DATA_SIZE)?;
    app.registry_try_activate_account(AppUser::Alice, None, None)?;

    let admin_usdc_before = app.get_balance(AppUser::Admin, AppToken::USDC);

    app.registry_try_withdraw_revenue(AppUser::Admin, None, None, None)?;

    let admin_usdc_after = app.get_balance(AppUser::Admin, AppToken::USDC);
    assert_eq!(
        admin_usdc_after - admin_usdc_before,
        ACCOUNT_REGISTRATION_FEE_AMOUNT
    );

    Ok(())
}

#[test]
fn reopen_account_default() -> Result<()> {
    const MAX_DATA_SIZE_0: u32 = 1_000;
    const MAX_DATA_SIZE_1: u32 = 1_000;

    let mut app = init_app()?;

    app.registry_try_create_account(AppUser::Alice, MAX_DATA_SIZE_0)?;
    app.registry_try_activate_account(AppUser::Alice, None, None)?;
    app.registry_try_close_account(AppUser::Alice)?;

    let user_id = app.registry_query_user_id(AppUser::Alice)?;
    assert_eq!(user_id.id, 1);
    assert_eq!(user_id.is_open, false);
    assert_eq!(user_id.is_activated, true);

    app.registry_try_reopen_account(AppUser::Alice, MAX_DATA_SIZE_1)?;

    let user_id = app.registry_query_user_id(AppUser::Alice)?;
    assert_eq!(user_id.id, 1);
    assert_eq!(user_id.is_open, true);
    assert_eq!(user_id.is_activated, true);

    assert_eq!(
        app.registry_query_user_account(user_id.id)?.max_size,
        MAX_DATA_SIZE_1
    );

    Ok(())
}

#[test]
fn write_data_default() -> Result<()> {
    const MAX_DATA_SIZE: u32 = 1_000;
    const DATA_0: &str = "encrypted_secrets_0";
    const DATA_1: &str = "encrypted_secrets_1";
    const NONCE_0: u64 = 1;
    const NONCE_1: u64 = 2;

    let mut app = init_app()?;

    app.registry_try_create_account(AppUser::Alice, MAX_DATA_SIZE)?;
    app.registry_try_activate_account(AppUser::Alice, None, None)?;

    for (data, nonce) in [(DATA_0, NONCE_0), (DATA_1, NONCE_1)] {
        app.registry_try_write_data(AppUser::Alice, data, nonce)?;

        assert_eq!(
            app.registry_query_user_account(app.registry_query_user_id(AppUser::Alice)?.id)?,
            UserAccount {
                data: data.to_string(),
                nonce,
                max_size: MAX_DATA_SIZE
            }
        );
    }

    Ok(())
}

#[test]
fn rotate_account_default() -> Result<()> {
    const MAX_DATA_SIZE: u32 = 1_000;
    const DATA_0: &str = "encrypted_secrets_0";
    const NONCE_0: u64 = 1;

    let mut app = init_app()?;

    app.registry_try_create_account(AppUser::Alice, MAX_DATA_SIZE)?;
    app.registry_try_activate_account(AppUser::Alice, None, None)?;
    app.registry_try_write_data(AppUser::Alice, DATA_0, NONCE_0)?;

    app.registry_try_request_account_rotation(AppUser::Alice, AppUser::Bob)?;
    app.registry_try_confirm_account_rotation(AppUser::Bob, AppUser::Alice)?;

    app.registry_query_user_id(AppUser::Alice).unwrap_err();

    let bob_user_id = app.registry_query_user_id(AppUser::Bob)?;
    assert_eq!(
        app.registry_query_user_account(bob_user_id.id)?,
        UserAccount {
            data: DATA_0.to_string(),
            nonce: NONCE_0,
            max_size: MAX_DATA_SIZE
        }
    );

    Ok(())
}
