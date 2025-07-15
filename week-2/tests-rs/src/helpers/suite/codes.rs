use std::{
    error::Error,
    fmt::{Debug, Display},
};

use cosmwasm_schema::{
    schemars::JsonSchema,
    serde::{de::DeserializeOwned, Serialize},
};
use cosmwasm_std::{
    Addr, Binary, CustomMsg, CustomQuery, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response,
    StdResult, Uint128,
};
use cw_gopniks::assets::{Currency, TokenUnverified};
use cw_multi_test::{AppResponse, Contract, ContractWrapper, Executor};

use strum::IntoEnumIterator;

use coral_base::{
    error::parse_err,
    hub::types::{Fee, Range},
};

use crate::helpers::suite::{
    core::Project,
    types::{GetDecimals, ProjectAccount, ProjectToken},
};

pub trait WithCodes {
    // store packages
    fn store_cw20_base_code(&mut self) -> u64;

    // store contracts
    fn store_hub_code(&mut self) -> u64;
    fn store_offer_code(&mut self) -> u64;
    fn store_profile_code(&mut self) -> u64;
    fn store_trade_code(&mut self) -> u64;
    fn store_hash_henerator_code(&mut self) -> u64;
    fn store_chat_code(&mut self) -> u64;
    fn store_justice_code(&mut self) -> u64;

    // instantiate packages
    fn instantiate_cw20_base_token(&mut self, code_id: u64, project_token: ProjectToken) -> Addr;

    // instantiate contracts
    #[allow(clippy::too_many_arguments)]
    fn instantiate_hub(
        &mut self,
        code_id: u64,
        admin: Option<ProjectAccount>,
        offer: Option<&Addr>,
        trade: Option<&Addr>,
        profile: Option<&Addr>,
        hash_generator: Option<&Addr>,
        chat: Option<&Addr>,
        justice: Option<&Addr>,
        treasury: Option<&Addr>,
        judge_list: Option<&[ProjectAccount]>,

        trade_expiration_timer: Option<u64>,
        trade_dispute_timer: Option<u64>,
        join_dispute_timer: Option<u64>,
        active_offers_limit: Option<u8>,
        active_trades_limit: Option<u8>,
        stablecoin: Option<Currency<TokenUnverified>>,
        trade_fee: Option<Fee>,
        dispute_fee: Option<Fee>,
        trade_limit: Option<Range>,
        chat_max_length: Option<u8>,
        chat_max_symbols_per_msg: Option<u16>,
        chat_max_msg_sequence: Option<u8>,
        dispute_quorum: Option<u8>,
    ) -> Addr;
    fn instantiate_offer(&mut self, code_id: u64) -> Addr;
    fn instantiate_profile(&mut self, code_id: u64) -> Addr;
    fn instantiate_trade(&mut self, code_id: u64) -> Addr;
    fn instantiate_hash_generator(&mut self, code_id: u64) -> Addr;
    fn instantiate_chat(&mut self, code_id: u64) -> Addr;
    fn instantiate_justice(&mut self, code_id: u64) -> Addr;

    fn migrate_contract(
        &mut self,
        sender: ProjectAccount,
        contract_address: Addr,
        contract_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse>;
}

impl WithCodes for Project {
    // store packages
    fn store_cw20_base_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        ))
    }

    fn store_hub_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            hub::contract::execute,
            hub::contract::instantiate,
            hub::contract::query,
        ))
    }

    fn store_offer_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            offer::contract::execute,
            offer::contract::instantiate,
            offer::contract::query,
        ))
    }

    fn store_profile_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            profile::contract::execute,
            profile::contract::instantiate,
            profile::contract::query,
        ))
    }

    fn store_trade_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            trade::contract::execute,
            trade::contract::instantiate,
            trade::contract::query,
        ))
    }

    fn store_hash_henerator_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            hash_generator::contract::execute,
            hash_generator::contract::instantiate,
            hash_generator::contract::query,
        ))
    }

    fn store_chat_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            chat::contract::execute,
            chat::contract::instantiate,
            chat::contract::query,
        ))
    }

    fn store_justice_code(&mut self) -> u64 {
        self.app.store_code(box_contract(
            justice::contract::execute,
            justice::contract::instantiate,
            justice::contract::query,
        ))
    }

    // instantiate packages
    fn instantiate_cw20_base_token(&mut self, code_id: u64, project_token: ProjectToken) -> Addr {
        let symbol = "TOKEN".to_string(); // max 10 tokens
        let initial_balances: Vec<cw20::Cw20Coin> = ProjectAccount::iter()
            .map(|project_account| {
                let amount = project_account.get_initial_funds_amount()
                    * 10u128.pow(project_token.get_decimals() as u32);

                cw20::Cw20Coin {
                    address: project_account.to_string(),
                    amount: Uint128::from(amount),
                }
            })
            .collect();

        self.instantiate_contract(
            code_id,
            "token",
            &cw20_base::msg::InstantiateMsg {
                name: format!("cw20-base token {}", symbol),
                symbol,
                decimals: project_token.get_decimals(),
                initial_balances,
                mint: None,
                marketing: None,
            },
        )
    }

    // instantiate contracts

    fn instantiate_hub(
        &mut self,
        code_id: u64,
        admin: Option<ProjectAccount>,
        offer: Option<&Addr>,
        trade: Option<&Addr>,
        profile: Option<&Addr>,
        hash_generator: Option<&Addr>,
        chat: Option<&Addr>,
        justice: Option<&Addr>,
        treasury: Option<&Addr>,
        judge_list: Option<&[ProjectAccount]>,

        trade_expiration_timer: Option<u64>,
        trade_dispute_timer: Option<u64>,
        join_dispute_timer: Option<u64>,
        active_offers_limit: Option<u8>,
        active_trades_limit: Option<u8>,
        stablecoin: Option<Currency<TokenUnverified>>,
        trade_fee: Option<Fee>,
        dispute_fee: Option<Fee>,
        trade_limit: Option<Range>,
        chat_max_length: Option<u8>,
        chat_max_symbols_per_msg: Option<u16>,
        chat_max_msg_sequence: Option<u8>,
        dispute_quorum: Option<u8>,
    ) -> Addr {
        self.instantiate_contract(
            code_id,
            "hub",
            &coral_base::hub::msg::InstantiateMsg {
                admin: admin.map(|x| x.to_string()),
                offer: offer.map(|x| x.to_string()),
                trade: trade.map(|x| x.to_string()),
                profile: profile.map(|x| x.to_string()),
                hash_generator: hash_generator.map(|x| x.to_string()),
                chat: chat.map(|x| x.to_string()),
                justice: justice.map(|x| x.to_string()),
                treasury: treasury.map(|x| x.to_string()),
                judge_list: judge_list.map(|x| x.iter().map(|y| y.to_string()).collect()),

                trade_expiration_timer,
                trade_dispute_timer,
                join_dispute_timer,
                active_offers_limit,
                active_trades_limit,
                stablecoin,
                trade_fee,
                dispute_fee,
                trade_limit,
                chat_max_length,
                chat_max_symbols_per_msg,
                chat_max_msg_sequence,
                dispute_quorum,
            },
        )
    }

    fn instantiate_offer(&mut self, code_id: u64) -> Addr {
        self.instantiate_contract(code_id, "offer", &coral_base::offer::msg::InstantiateMsg {})
    }

    fn instantiate_profile(&mut self, code_id: u64) -> Addr {
        self.instantiate_contract(
            code_id,
            "profile",
            &coral_base::profile::msg::InstantiateMsg {},
        )
    }

    fn instantiate_trade(&mut self, code_id: u64) -> Addr {
        self.instantiate_contract(code_id, "trade", &coral_base::trade::msg::InstantiateMsg {})
    }

    fn instantiate_hash_generator(&mut self, code_id: u64) -> Addr {
        self.instantiate_contract(
            code_id,
            "hash_generator",
            &coral_base::hash_generator::msg::InstantiateMsg {},
        )
    }

    fn instantiate_chat(&mut self, code_id: u64) -> Addr {
        self.instantiate_contract(code_id, "chat", &coral_base::chat::msg::InstantiateMsg {})
    }

    fn instantiate_justice(&mut self, code_id: u64) -> Addr {
        self.instantiate_contract(
            code_id,
            "justice",
            &coral_base::justice::msg::InstantiateMsg {},
        )
    }

    fn migrate_contract(
        &mut self,
        sender: ProjectAccount,
        contract_address: Addr,
        contract_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse> {
        self.app
            .migrate_contract(
                sender.into(),
                contract_address,
                &migrate_msg,
                contract_new_code_id,
            )
            .map_err(parse_err)
    }
}
