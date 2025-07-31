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
        state::{CommonConfig, ACCOUNT_REGISTRATION_FEE_AMOUNT, ROTATION_TIMEOUT},
        types::AssetItem,
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
        app.registry_query_common_config()?,
        CommonConfig {
            admin: AppUser::Admin.pubkey(),
            is_paused: false,
            rotation_timeout: ROTATION_TIMEOUT
        }
    );

    Ok(())
}

#[test]
fn transfer_admin() -> Result<()> {
    let mut app = init_app()?;

    let res = app
        .registry_try_update_common_config(AppUser::Alice, Some(AppUser::Alice), None, None)
        .unwrap_err();
    assert_error(res, AuthError::Unauthorized);

    let res = app
        .registry_try_confirm_admin_rotation(AppUser::Alice)
        .unwrap_err();
    assert_error(res, AuthError::NoNewAdmin);

    app.registry_try_update_common_config(AppUser::Admin, Some(AppUser::Alice), None, None)?;

    app.wait(ROTATION_TIMEOUT as u64);
    let res = app
        .registry_try_confirm_admin_rotation(AppUser::Alice)
        .unwrap_err();
    assert_error(res, AuthError::TransferAdminDeadline);

    app.registry_try_update_common_config(AppUser::Admin, Some(AppUser::Alice), None, None)?;

    let res = app
        .registry_try_confirm_admin_rotation(AppUser::Bob)
        .unwrap_err();
    assert_error(res, AuthError::Unauthorized);

    app.registry_try_confirm_admin_rotation(AppUser::Alice)?;
    assert_eq!(
        app.registry_query_common_config()?.admin,
        AppUser::Alice.pubkey()
    );

    Ok(())
}

#[test]
fn create_account_default() -> Result<()> {
    let mut app = init_app()?;

    app.registry_try_create_account(AppUser::Alice, 1_000)?;

    Ok(())
}
