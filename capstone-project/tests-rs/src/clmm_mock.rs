use {
    crate::helpers::{
        extensions::clmm_mock::ClmmMockExtension,
        suite::{
            core::{extension::get_data, token::WithTokenKeys, App},
            types::{AppToken, AppUser},
        },
    },
    anchor_lang::Result,
    // pretty_assertions::assert_eq,
};

#[test]
fn swap_default() -> Result<()> {
    const AMM_CONFIG_INDEX: u16 = 0;
    // const DECIMALS_MULTIPLIER: u64 = 1_000_000;
    // const AMOUNT_A: u64 = 10 * DECIMALS_MULTIPLIER;
    // const AMOUNT_B: u64 = 10 * AMOUNT_A;

    let mut app = App::new();
    app.wait(1_000);

    app.clmm_mock_try_create_operation_account(AppUser::Admin)?;
    let _operation_account = app.clmm_mock_query_operation_account()?;

    // let amm_conf: raydium_clmm_cpi::states::AmmConfig = get_data(
    //     &app.litesvm,
    //     &app.pda.clmm_mock_amm_config(AMM_CONFIG_INDEX),
    // )?;

    // https://explorer.solana.com/address/9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x/anchor-account
    app.clmm_mock_try_create_amm_config(AppUser::Admin, AMM_CONFIG_INDEX, 1, 100, 120_000, 40_000)?;
    //  let _amm_config = app.clmm_mock_query_amm_config(AMM_CONFIG_INDEX)?;

    // let token_info_list = sort_token_info_list(
    //     &app,
    //     &[(AMOUNT_A, AppToken::USDC), (AMOUNT_B, AppToken::PYTH)],
    // );
    // let (amount_a, token_a) = token_info_list[0];
    // let (amount_b, token_b) = token_info_list[1];
    // let mint_a = &token_a.pubkey(&app);
    // let mint_b = &token_b.pubkey(&app);

    // app.clmm_mock_try_create_pool(AppUser::Admin, (amount_a, token_a), (amount_b, token_b))?;

    // let pool_state_pda = &app.pda.clmm_mock_pool_config(mint_a, mint_b);
    // assert_eq!(app.get_token_balance(pool_state_pda, mint_a), amount_a);
    // assert_eq!(app.get_token_balance(pool_state_pda, mint_b), amount_b);

    Ok(())
}

pub fn sort_token_info_list(app: &App, list: &[(u64, AppToken)]) -> Vec<(u64, AppToken)> {
    let mut mint_list: Vec<_> = list.iter().map(|(_, x)| x.pubkey(&app)).collect();
    mint_list.sort_unstable();

    mint_list
        .iter()
        .map(|mint| {
            let (amount, token) = list.iter().find(|(_, x)| mint == &x.pubkey(app)).unwrap();
            (*amount, *token)
        })
        .collect()
}
