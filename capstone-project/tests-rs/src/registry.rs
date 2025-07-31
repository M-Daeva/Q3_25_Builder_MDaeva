use {
    crate::helpers::{
        registry::RegistryExtension,
        suite::{
            core::{assert_error, token::WithTokenKeys, App},
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    base::error::AuthError,
    pretty_assertions::assert_eq,
    registry::{
        state::{
            Config, ACCOUNT_DATA_SIZE_MAX, ACCOUNT_DATA_SIZE_MIN, ACCOUNT_REGISTRATION_FEE_AMOUNT,
            ROTATION_TIMEOUT,
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
            asset: AppToken::USDC.pubkey(&app),
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
                asset: AppToken::USDC.pubkey(&app),
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
fn create_account_default() -> Result<()> {
    let mut app = init_app()?;

    app.registry_try_create_account(AppUser::Alice, 1_000)?;

    Ok(())
}
