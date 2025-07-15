use litesvm::LiteSVM;
use solana_kite::{create_associated_token_account, create_token_mint, mint_tokens_to_account};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use strum::IntoEnumIterator;

use crate::helpers::suite::types::{AppAsset, AppCoin, AppToken, AppUser, GetDecimals};

pub trait WithTokenPubkey {
    fn pubkey(&self, app: &App) -> Pubkey;
}

impl WithTokenPubkey for AppToken {
    fn pubkey(&self, app: &App) -> Pubkey {
        app.token_registry
            .iter()
            .find(|(token, _)| token == self)
            .map(|(_, pubkey)| *pubkey)
            .unwrap()
    }
}

pub struct App {
    pub litesvm: LiteSVM,
    token_registry: Vec<(AppToken, Pubkey)>,
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
    // hub_address: Addr,
    // offer_address: Addr,
    // profile_address: Addr,
    // trade_address: Addr,
    // hash_generator_address: Addr,
    // chat_address: Addr,
    // justice_address: Addr,
    //
    // other
}

impl App {
    pub fn create_app_with_balances() -> Self {
        let (litesvm, token_registry) = Self::init_app_with_balances();

        Self {
            litesvm,
            token_registry,
            contract_counter: 0,

            cw20_base_code_id: 0,

            hub_code_id: 0,
            offer_code_id: 0,
            profile_code_id: 0,
            trade_code_id: 0,
            hash_generator_code_id: 0,
            chat_code_id: 0,
            justice_code_id: 0,
            //
            // hub_address: Addr::unchecked(""),
            // offer_address: Addr::unchecked(""),
            // profile_address: Addr::unchecked(""),
            // trade_address: Addr::unchecked(""),
            // hash_generator_address: Addr::unchecked(""),
            // chat_address: Addr::unchecked(""),
            // justice_address: Addr::unchecked(""),
        }
    }

    pub fn new() -> Self {
        // create app and distribute coins to accounts
        let mut app = Self::create_app_with_balances();

        // // register contracts code
        // // packages
        // let cw20_base_code_id = app.store_cw20_base_code();

        // // contracts
        // let justice_code_id = app.store_justice_code();

        // // instantiate packages

        // // instantiate app contracts

        // let offer_address = app.instantiate_offer(offer_code_id);

        // app = Self {
        //     cw20_base_code_id,

        //     hub_code_id,
        //     offer_code_id,
        //     profile_code_id,
        //     trade_code_id,
        //     hash_generator_code_id,
        //     chat_code_id,
        //     justice_code_id,

        //     hub_address,
        //     offer_address,
        //     profile_address,
        //     trade_address,
        //     hash_generator_address,
        //     chat_address,
        //     justice_address,

        //     ..app
        // };

        // // prepare contracts

        // // MUST BE EXECUTED BEFORE UpdateAddressConfig
        // app
        //     .hub_try_update_common_config(
        //         AppAccount::Admin,
        //         Some(21_600),
        //         Some(9_000),
        //         Some(3_000),
        //         Some(4),
        //         Some(20),
        //         Some(Currency::new(
        //             &TokenUnverified::new_native(&AppCoin::Usdc.to_string()),
        //             AppCoin::Usdc.get_decimals(),
        //         )),
        //         Some(Fee::new(0, "0.01")),
        //         Some(Fee::new(0, "0.04")),
        //         Some(Range::new(1_u128, 1_000_000_000_000_u128)),
        //         None,
        //         None,
        //         None,
        //         None,
        //     )
        //     .unwrap();

        // app
        //     .hub_try_update_address_config(
        //         AppAccount::Admin,
        //         None,
        //         Some(&app.get_offer_address()),
        //         Some(&app.get_trade_address()),
        //         Some(&app.get_profile_address()),
        //         Some(&app.get_hash_generator_address()),
        //         Some(&app.get_chat_address()),
        //         Some(&app.get_justice_address()),
        //         Some(&AppAccount::Treasury.into()),
        //         Some(&[AppAccount::ArbitratorA]),
        //     )
        //     .unwrap();

        // app
        //     .hub_try_update_common_config(
        //         AppAccount::Admin,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         Some(1),
        //     )
        //     .unwrap();

        app
    }

    // code id getters
    pub fn get_hub_code_id(&self) -> u64 {
        self.hub_code_id
    }

    // package address getters

    // contract address getters

    // pub fn get_hub_address(&self) -> Addr {
    //     self.hub_address.clone()
    // }

    // utils
    pub fn increase_contract_counter(&mut self, step: u16) {
        self.contract_counter += step;
    }

    pub fn get_last_contract_address(&self) -> String {
        format!("contract{}", self.contract_counter)
    }

    // pub fn get_block_time(&self) -> u64 {
    //     self.app.block_info().time.seconds()
    // }

    // pub fn reset_time(&mut self) {
    //     self.app.update_block(|block| {
    //         block.time = Timestamp::default().plus_seconds(1_000);
    //         block.height = 200;
    //     });
    // }

    // pub fn wait(&mut self, delay_s: u64) {
    //     self.app.update_block(|block| {
    //         block.time = block.time.plus_seconds(delay_s);
    //         block.height += delay_s / 5;
    //     });
    // }

    // pub fn set_chain_id(&mut self, chain_id: &str) {
    //     self.app.update_block(|block| {
    //         block.chain_id = chain_id.to_string();
    //     });
    // }

    // pub fn mint_native(&mut self, recipient: impl ToString, amount: u128, asset: AppCoin) {
    //     self.app
    //         .sudo(SudoMsg::Bank(BankSudo::Mint {
    //             to_address: recipient.to_string(),
    //             amount: coins(amount, asset.to_string()),
    //         }))
    //         .unwrap();
    // }

    // pub fn increase_allowances(
    //     &mut self,
    //     owner: AppAccount,
    //     spender: impl ToString,
    //     assets: &[(impl Into<Uint128> + Clone, AppToken)],
    // ) {
    //     for (asset_amount, token) in assets {
    //         self.app
    //             .execute_contract(
    //                 owner.into(),
    //                 token.to_owned().into(),
    //                 &cw20_base::msg::ExecuteMsg::IncreaseAllowance {
    //                     spender: spender.to_string(),
    //                     amount: asset_amount.to_owned().into(),
    //                     expires: None,
    //                 },
    //                 &[],
    //             )
    //             .unwrap();
    //     }
    // }

    pub fn get_balance(
        &self,
        user: AppUser,
        asset: impl Into<AppAsset>,
    ) -> Result<u64, anchor_lang::error::Error> {
        let address = &user.pubkey();

        match asset.into() {
            AppAsset::Coin(_) => self.get_coin_balance(address),
            AppAsset::Token(mint) => self.get_token_balance(address, &mint.pubkey(self)),
        }
    }

    pub fn get_coin_balance(&self, address: &Pubkey) -> Result<u64, anchor_lang::error::Error> {
        self.litesvm
            .get_balance(address)
            .ok_or(to_anchor_err("SOL balance error"))
    }

    pub fn get_token_balance(
        &self,
        address: &Pubkey,
        mint: &Pubkey,
    ) -> Result<u64, anchor_lang::error::Error> {
        let ata = spl_associated_token_account::get_associated_token_address(address, mint);
        solana_kite::get_token_account_balance(&self.litesvm, &ata).map_err(to_anchor_err)
    }

    // pub fn instantiate_contract(
    //     &mut self,
    //     code_id: u64,
    //     label: &str,
    //     init_msg: &impl Serialize,
    // ) -> Addr {
    //     self.increase_contract_counter(1);

    //     self.app
    //         .instantiate_contract(
    //             code_id,
    //             AppAccount::Admin.into(),
    //             init_msg,
    //             &[],
    //             label,
    //             Some(AppAccount::Admin.to_string()),
    //         )
    //         .unwrap()
    // }

    fn init_app_with_balances() -> (LiteSVM, Vec<(AppToken, Pubkey)>) {
        let mut litesvm = LiteSVM::new();
        let mut token_registry: Vec<(AppToken, Pubkey)> = vec![];

        // airdrop SOL
        for user in AppUser::iter() {
            litesvm
                .airdrop(
                    &user.pubkey(),
                    user.get_initial_asset_amount() * LAMPORTS_PER_SOL,
                )
                .unwrap();
        }

        // create tokens
        for token in AppToken::iter() {
            let mint = create_token_mint(
                &mut litesvm,
                &AppUser::Admin.keypair(),
                token.get_decimals(),
            )
            .unwrap();

            token_registry.push((token, mint.pubkey()));
        }

        // mint tokens
        for user in AppUser::iter() {
            for (token, mint) in &token_registry {
                let ata = create_associated_token_account(
                    &mut litesvm,
                    &user.keypair(),
                    mint,
                    &AppUser::Admin.keypair(),
                )
                .unwrap();

                mint_tokens_to_account(
                    &mut litesvm,
                    mint,
                    &ata,
                    user.get_initial_asset_amount() * 10u64.pow(token.get_decimals() as u32),
                    &AppUser::Admin.keypair(),
                )
                .unwrap();
            }
        }

        (litesvm, token_registry)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// #[track_caller]
// pub fn assert_error(err: StdError, expected: impl ToString + Sized + std::fmt::Debug) {
//     let expected_error_name = format!("{:#?}", expected);
//     let expected_error_text = expected.to_string();

//     let error = format!("{}", err);
//     let contains_name = error.contains(&expected_error_name);
//     let contains_text = error.contains(&expected_error_text);

//     pretty_assertions::assert_eq!(
//         "",
//         if contains_name || contains_text {
//             ""
//         } else {
//             " "
//         },
//         "\n\n✅ Expected error:\n{} -> {}\n\n❌ Received error:\n{}",
//         expected_error_name,
//         expected_error_text,
//         error
//     );
// }

// pub fn add_funds_to_exec_msg<T: Serialize + std::fmt::Debug>(
//     app: &mut App,
//     sender: AppAccount,
//     contract_address: &Addr,
//     msg: &T,
//     amount: impl Into<Uint128>,
//     asset: impl Into<AppAsset>,
// ) -> StdResult<AppResponse> {
//     let asset: AppAsset = asset.into();

//     match asset {
//         AppAsset::Coin(denom) => app
//             .app
//             .execute_contract(
//                 sender.into(),
//                 contract_address.to_owned(),
//                 msg,
//                 &[coin(
//                     Into::<Uint128>::into(amount).u128(),
//                     denom.to_string(),
//                 )],
//             )
//             .map_err(parse_err),
//         AppAsset::Token(address) => {
//             let wasm_msg = cw20::Cw20ExecuteMsg::Send {
//                 contract: contract_address.to_string(),
//                 amount: Into::<Uint128>::into(amount),
//                 msg: to_json_binary(msg).unwrap(),
//             };

//             app
//                 .app
//                 .execute_contract(sender.into(), address.into(), &wasm_msg, &[])
//                 .map_err(parse_err)
//         }
//     }
// }

// pub fn add_token_to_exec_msg<T: Serialize + std::fmt::Debug>(
//     app: &mut App,
//     sender: AppAccount,
//     contract_address: &Addr,
//     msg: &T,
//     amount: impl Into<Uint128>,
//     asset: &Token,
// ) -> StdResult<AppResponse> {
//     match asset {
//         Token::Native { denom } => app
//             .app
//             .execute_contract(
//                 sender.into(),
//                 contract_address.to_owned(),
//                 msg,
//                 &[coin(
//                     Into::<Uint128>::into(amount).u128(),
//                     denom.to_string(),
//                 )],
//             )
//             .map_err(parse_err),
//         Token::Cw20 { address } => {
//             let wasm_msg = cw20::Cw20ExecuteMsg::Send {
//                 contract: contract_address.to_string(),
//                 amount: Into::<Uint128>::into(amount),
//                 msg: to_json_binary(msg).unwrap(),
//             };

//             app
//                 .app
//                 .execute_contract(sender.into(), address.to_owned(), &wasm_msg, &[])
//                 .map_err(parse_err)
//         }
//     }
// }

pub fn to_string_vec(str_vec: &[&str]) -> Vec<String> {
    str_vec.iter().map(|x| x.to_string()).collect()
}

// get_token_account_balance(&app.litesvm, &bob_pyth_ata).map_err(|_| anchor_lang::error!(CustomError::TokenBalanceError))?;
pub fn to_anchor_err(message: impl ToString) -> anchor_lang::error::Error {
    anchor_lang::error::Error::AnchorError(Box::new(anchor_lang::error::AnchorError {
        error_name: "CustomError".to_string(),
        error_code_number: 9_999,
        error_msg: message.to_string(),
        error_origin: None,
        compared_values: None,
    }))
}
