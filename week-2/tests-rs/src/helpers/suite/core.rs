use std::fs;
use std::str::FromStr;

use litesvm::LiteSVM;
use solana_kite::{create_associated_token_account, create_token_mint, mint_tokens_to_account};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use strum::IntoEnumIterator;

use crate::helpers::suite::types::{AppAsset, AppToken, AppUser, GetDecimals};

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
    //
    // package address

    // contract address
    amm: Pubkey,
    //
    // other
}

impl App {
    pub fn create_app_with_balances() -> Self {
        let (litesvm, token_registry) = Self::init_app_with_balances();

        Self {
            litesvm,
            token_registry,

            amm: Pubkey::default(),
        }
    }

    pub fn new() -> Self {
        // create app and distribute assets to accounts
        let mut app = Self::create_app_with_balances();

        // upload packages

        // upload app contracts

        app = Self {
            amm: app.upload_program("amm"),

            ..app
        };

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

    // package address getters

    // contract address getters

    pub fn get_program_amm(&self) -> Pubkey {
        self.amm
    }

    // utils

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

    fn upload_program(&mut self, program: &str) -> solana_pubkey::Pubkey {
        const CONFIG_PATH: &str = "../Anchor.toml";
        const PROGRAM_PATH: &str = "../target/deploy/";

        let program_id = read_program_id(CONFIG_PATH, program);
        solana_kite::deploy_program(
            &mut self.litesvm,
            &program_id,
            &format!("{}{}.so", PROGRAM_PATH, program),
        )
        .unwrap();

        program_id
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

pub fn read_program_id(file: &str, program: &str) -> solana_pubkey::Pubkey {
    let content = fs::read_to_string(file).expect("Failed to read file");

    let mut in_localnet_section = false;

    for line in content.lines() {
        let line = line.trim();

        // Check if we're entering the [programs.localnet] section
        if line == "[programs.localnet]" {
            in_localnet_section = true;
            continue;
        }

        // Check if we're leaving the section (entering a new section)
        if line.starts_with('[') && line != "[programs.localnet]" {
            in_localnet_section = false;
            continue;
        }

        // If we're in the localnet section, look for our program
        if in_localnet_section && !line.is_empty() && !line.starts_with('#') {
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');

                if key == program {
                    return solana_pubkey::Pubkey::from_str(value).expect("Invalid pubkey format");
                }
            }
        }
    }

    panic!(
        "Program '{}' not found in [programs.localnet] section",
        program
    );
}
