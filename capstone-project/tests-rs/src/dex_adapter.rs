use {
    crate::helpers::{
        dex_adapter::DexAdapterExtension,
        suite::{
            core::{token::WithTokenKeys, App},
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    dex_adapter::state::{Config, ROTATION_TIMEOUT},
    pretty_assertions::assert_eq,
};

#[test]
fn init_default() -> Result<()> {
    let mut app = App::new();

    app.dex_adapter_try_init(
        AppUser::Admin,
        None,
        None,
        Some(vec![AppToken::USDC, AppToken::WBTC]),
    )?;

    assert_eq!(
        app.dex_adapter_query_config()?,
        Config {
            admin: AppUser::Admin.pubkey(),
            registry: None,
            is_paused: false,
            rotation_timeout: ROTATION_TIMEOUT,
            token_in_whitelist: [AppToken::USDC, AppToken::WBTC]
                .iter()
                .map(|y| y.pubkey(&app))
                .collect()
        }
    );

    Ok(())
}
