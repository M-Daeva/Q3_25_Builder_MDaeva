use {
    crate::helpers::{
        registry::RegistryExtension,
        suite::{
            core::{token::WithTokenKeys, App},
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    pretty_assertions::assert_eq,
    registry::{
        state::{CommonConfig, ACCOUNT_REGISTRATION_FEE_AMOUNT, ROTATION_TIMEOUT},
        types::AssetItem,
    },
};

#[test]
fn init_default() -> Result<()> {
    let mut app = App::new();

    app.registry_try_init(
        AppUser::Admin,
        None,
        None,
        Some(AssetItem {
            amount: ACCOUNT_REGISTRATION_FEE_AMOUNT,
            asset: AppToken::USDC.pubkey(&app),
        }),
        None,
        None,
        None,
    )?;

    assert_eq!(
        app.registry_query_common_config()?,
        CommonConfig {
            admin: AppUser::Admin.pubkey(),
            dex_adapter: None,
            is_paused: false,
            rotation_timeout: ROTATION_TIMEOUT
        }
    );

    Ok(())
}
