use cosmwasm_schema::serde::Serialize;
use cosmwasm_std::{
    coin, coins, testing::MockStorage, to_json_binary, Addr, Coin, Empty, StdError, StdResult,
    Timestamp, Uint128,
};
use cw_gopniks::assets::{Currency, Token, TokenUnverified};
use cw_multi_test::{
    App, AppBuilder, AppResponse, BankKeeper, BankSudo, DistributionKeeper, Executor,
    FailingModule, GovFailingModule, IbcFailingModule, MockApiBech32, StakeKeeper, StargateFailing,
    SudoMsg, WasmKeeper,
};

use strum::IntoEnumIterator;

use coral_base::{
    error::parse_err,
    hub::types::{Fee, Range},
};

use crate::helpers::{
    hub::HubExtension,
    suite::{
        codes::WithCodes,
        types::{
            GetDecimals, ProjectAccount, ProjectAsset, ProjectCoin, ProjectToken, WrappedResponse,
        },
    },
};

pub type CustomApp = App<
    BankKeeper,
    MockApiBech32,
    MockStorage,
    FailingModule<Empty, Empty, Empty>,
    WasmKeeper<Empty, Empty>,
    StakeKeeper,
    DistributionKeeper,
    IbcFailingModule,
    GovFailingModule,
    StargateFailing,
>;

pub struct Project {
    pub app: CustomApp,
    pub logs: WrappedResponse,
    contract_counter: u16,

    // package code id
    cw20_base_code_id: u64,

    // contract code id
    hub_code_id: u64,
    offer_code_id: u64,
    profile_code_id: u64,
    trade_code_id: u64,
    hash_generator_code_id: u64,
    chat_code_id: u64,
    justice_code_id: u64,

    // package address

    // contract address
    hub_address: Addr,
    offer_address: Addr,
    profile_address: Addr,
    trade_address: Addr,
    hash_generator_address: Addr,
    chat_address: Addr,
    justice_address: Addr,
    //
    // other
}

impl Project {
    pub fn create_project_with_balances() -> Self {
        Self {
            app: Self::create_app_with_balances(),
            logs: WrappedResponse::Execute(Ok(AppResponse::default())),
            contract_counter: 0,

            cw20_base_code_id: 0,

            hub_code_id: 0,
            offer_code_id: 0,
            profile_code_id: 0,
            trade_code_id: 0,
            hash_generator_code_id: 0,
            chat_code_id: 0,
            justice_code_id: 0,

            hub_address: Addr::unchecked(""),
            offer_address: Addr::unchecked(""),
            profile_address: Addr::unchecked(""),
            trade_address: Addr::unchecked(""),
            hash_generator_address: Addr::unchecked(""),
            chat_address: Addr::unchecked(""),
            justice_address: Addr::unchecked(""),
        }
    }

    pub fn new() -> Self {
        // create app and distribute coins to accounts
        let mut project = Self::create_project_with_balances();

        // register contracts code
        // packages
        let cw20_base_code_id = project.store_cw20_base_code();

        // contracts
        let hub_code_id = project.store_hub_code();
        let offer_code_id = project.store_offer_code();
        let profile_code_id = project.store_profile_code();
        let trade_code_id = project.store_trade_code();
        let hash_generator_code_id = project.store_hash_henerator_code();
        let chat_code_id = project.store_chat_code();
        let justice_code_id = project.store_justice_code();

        // instantiate packages

        // DON'T CHANGE TOKEN INIT ORDER AS ITS ADDRESSES ARE HARDCODED IN ProjectToken ENUM
        for project_token in ProjectToken::iter() {
            project.instantiate_cw20_base_token(cw20_base_code_id, project_token);
        }

        // instantiate coral contracts

        let hub_address = project.instantiate_hub(
            hub_code_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let offer_address = project.instantiate_offer(offer_code_id);
        let profile_address = project.instantiate_profile(profile_code_id);
        let trade_address = project.instantiate_trade(trade_code_id);
        let hash_generator_address = project.instantiate_hash_generator(hash_generator_code_id);
        let chat_address = project.instantiate_chat(chat_code_id);
        let justice_address = project.instantiate_justice(justice_code_id);

        project = Self {
            cw20_base_code_id,

            hub_code_id,
            offer_code_id,
            profile_code_id,
            trade_code_id,
            hash_generator_code_id,
            chat_code_id,
            justice_code_id,

            hub_address,
            offer_address,
            profile_address,
            trade_address,
            hash_generator_address,
            chat_address,
            justice_address,

            ..project
        };

        // prepare contracts

        // MUST BE EXECUTED BEFORE UpdateAddressConfig
        project
            .hub_try_update_common_config(
                ProjectAccount::Admin,
                Some(21_600),
                Some(9_000),
                Some(3_000),
                Some(4),
                Some(20),
                Some(Currency::new(
                    &TokenUnverified::new_native(&ProjectCoin::Usdc.to_string()),
                    ProjectCoin::Usdc.get_decimals(),
                )),
                Some(Fee::new(0, "0.01")),
                Some(Fee::new(0, "0.04")),
                Some(Range::new(1_u128, 1_000_000_000_000_u128)),
                None,
                None,
                None,
                None,
            )
            .unwrap();

        project
            .hub_try_update_address_config(
                ProjectAccount::Admin,
                None,
                Some(&project.get_offer_address()),
                Some(&project.get_trade_address()),
                Some(&project.get_profile_address()),
                Some(&project.get_hash_generator_address()),
                Some(&project.get_chat_address()),
                Some(&project.get_justice_address()),
                Some(&ProjectAccount::Treasury.into()),
                Some(&[ProjectAccount::ArbitratorA]),
            )
            .unwrap();

        project
            .hub_try_update_common_config(
                ProjectAccount::Admin,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(1),
            )
            .unwrap();

        project
    }

    // code id getters
    pub fn get_cw20_base_code_id(&self) -> u64 {
        self.cw20_base_code_id
    }

    pub fn get_hub_code_id(&self) -> u64 {
        self.hub_code_id
    }

    pub fn get_offer_code_id(&self) -> u64 {
        self.offer_code_id
    }

    pub fn get_profile_code_id(&self) -> u64 {
        self.profile_code_id
    }

    pub fn get_trade_code_id(&self) -> u64 {
        self.trade_code_id
    }

    pub fn get_hash_generator_code_id(&self) -> u64 {
        self.hash_generator_code_id
    }

    pub fn get_chat_code_id(&self) -> u64 {
        self.chat_code_id
    }

    pub fn get_justice_code_id(&self) -> u64 {
        self.justice_code_id
    }

    // package address getters

    // contract address getters

    pub fn get_hub_address(&self) -> Addr {
        self.hub_address.clone()
    }

    pub fn get_offer_address(&self) -> Addr {
        self.offer_address.clone()
    }

    pub fn get_profile_address(&self) -> Addr {
        self.profile_address.clone()
    }

    pub fn get_trade_address(&self) -> Addr {
        self.trade_address.clone()
    }

    pub fn get_hash_generator_address(&self) -> Addr {
        self.hash_generator_address.clone()
    }

    pub fn get_chat_address(&self) -> Addr {
        self.chat_address.clone()
    }

    pub fn get_justice_address(&self) -> Addr {
        self.justice_address.clone()
    }

    // utils
    pub fn increase_contract_counter(&mut self, step: u16) {
        self.contract_counter += step;
    }

    pub fn get_last_contract_address(&self) -> String {
        format!("contract{}", self.contract_counter)
    }

    pub fn get_block_time(&self) -> u64 {
        self.app.block_info().time.seconds()
    }

    pub fn reset_time(&mut self) {
        self.app.update_block(|block| {
            block.time = Timestamp::default().plus_seconds(1_000);
            block.height = 200;
        });
    }

    pub fn wait(&mut self, delay_s: u64) {
        self.app.update_block(|block| {
            block.time = block.time.plus_seconds(delay_s);
            block.height += delay_s / 5;
        });
    }

    pub fn set_chain_id(&mut self, chain_id: &str) {
        self.app.update_block(|block| {
            block.chain_id = chain_id.to_string();
        });
    }

    pub fn mint_native(&mut self, recipient: impl ToString, amount: u128, asset: ProjectCoin) {
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: recipient.to_string(),
                amount: coins(amount, asset.to_string()),
            }))
            .unwrap();
    }

    pub fn increase_allowances(
        &mut self,
        owner: ProjectAccount,
        spender: impl ToString,
        assets: &[(impl Into<Uint128> + Clone, ProjectToken)],
    ) {
        for (asset_amount, token) in assets {
            self.app
                .execute_contract(
                    owner.into(),
                    token.to_owned().into(),
                    &cw20_base::msg::ExecuteMsg::IncreaseAllowance {
                        spender: spender.to_string(),
                        amount: asset_amount.to_owned().into(),
                        expires: None,
                    },
                    &[],
                )
                .unwrap();
        }
    }

    pub fn query_balance(
        &self,
        account: impl ToString,
        token: &(impl Into<Token> + Clone),
    ) -> StdResult<u128> {
        let token: Token = token.to_owned().into();

        match token {
            Token::Native { denom } => Ok(self
                .app
                .wrap()
                .query_balance(account.to_string(), denom)?
                .amount
                .u128()),
            Token::Cw20 { address } => {
                let cw20::BalanceResponse { balance } = self.app.wrap().query_wasm_smart(
                    address.to_string(),
                    &cw20::Cw20QueryMsg::Balance {
                        address: account.to_string(),
                    },
                )?;

                Ok(balance.u128())
            }
        }
    }

    pub fn instantiate_contract(
        &mut self,
        code_id: u64,
        label: &str,
        init_msg: &impl Serialize,
    ) -> Addr {
        self.increase_contract_counter(1);

        self.app
            .instantiate_contract(
                code_id,
                ProjectAccount::Admin.into(),
                init_msg,
                &[],
                label,
                Some(ProjectAccount::Admin.to_string()),
            )
            .unwrap()
    }

    fn create_app_with_balances() -> CustomApp {
        AppBuilder::new_custom()
            .with_api(MockApiBech32::new("wasm"))
            .build(|router, _api, storage| {
                for project_account in ProjectAccount::iter() {
                    let funds: Vec<Coin> = ProjectCoin::iter()
                        .map(|project_coin| {
                            let amount = project_account.get_initial_funds_amount()
                                * 10u128.pow(project_coin.get_decimals() as u32);

                            coin(amount, project_coin.to_string())
                        })
                        .collect();

                    router
                        .bank
                        .init_balance(storage, &project_account.into(), funds)
                        .unwrap();
                }
            })
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

#[track_caller]
pub fn assert_error(err: StdError, expected: impl ToString + Sized + std::fmt::Debug) {
    let expected_error_name = format!("{:#?}", expected);
    let expected_error_text = expected.to_string();

    let error = format!("{}", err);
    let contains_name = error.contains(&expected_error_name);
    let contains_text = error.contains(&expected_error_text);

    pretty_assertions::assert_eq!(
        "",
        if contains_name || contains_text {
            ""
        } else {
            " "
        },
        "\n\n✅ Expected error:\n{} -> {}\n\n❌ Received error:\n{}",
        expected_error_name,
        expected_error_text,
        error
    );
}

pub fn add_funds_to_exec_msg<T: Serialize + std::fmt::Debug>(
    project: &mut Project,
    sender: ProjectAccount,
    contract_address: &Addr,
    msg: &T,
    amount: impl Into<Uint128>,
    asset: impl Into<ProjectAsset>,
) -> StdResult<AppResponse> {
    let asset: ProjectAsset = asset.into();

    match asset {
        ProjectAsset::Coin(denom) => project
            .app
            .execute_contract(
                sender.into(),
                contract_address.to_owned(),
                msg,
                &[coin(
                    Into::<Uint128>::into(amount).u128(),
                    denom.to_string(),
                )],
            )
            .map_err(parse_err),
        ProjectAsset::Token(address) => {
            let wasm_msg = cw20::Cw20ExecuteMsg::Send {
                contract: contract_address.to_string(),
                amount: Into::<Uint128>::into(amount),
                msg: to_json_binary(msg).unwrap(),
            };

            project
                .app
                .execute_contract(sender.into(), address.into(), &wasm_msg, &[])
                .map_err(parse_err)
        }
    }
}

pub fn add_token_to_exec_msg<T: Serialize + std::fmt::Debug>(
    project: &mut Project,
    sender: ProjectAccount,
    contract_address: &Addr,
    msg: &T,
    amount: impl Into<Uint128>,
    asset: &Token,
) -> StdResult<AppResponse> {
    match asset {
        Token::Native { denom } => project
            .app
            .execute_contract(
                sender.into(),
                contract_address.to_owned(),
                msg,
                &[coin(
                    Into::<Uint128>::into(amount).u128(),
                    denom.to_string(),
                )],
            )
            .map_err(parse_err),
        Token::Cw20 { address } => {
            let wasm_msg = cw20::Cw20ExecuteMsg::Send {
                contract: contract_address.to_string(),
                amount: Into::<Uint128>::into(amount),
                msg: to_json_binary(msg).unwrap(),
            };

            project
                .app
                .execute_contract(sender.into(), address.to_owned(), &wasm_msg, &[])
                .map_err(parse_err)
        }
    }
}

pub fn to_string_vec(str_vec: &[&str]) -> Vec<String> {
    str_vec.iter().map(|x| x.to_string()).collect()
}
