use {
    crate::helpers::suite::{
        core::token::WithTokenKeys,
        types::{AppAsset, AppToken, AppUser, GetDecimals},
    },
    anchor_lang::{AnchorDeserialize, Id, InstructionData, Result, ToAccountMetas},
    anchor_spl::associated_token::AssociatedToken,
    litesvm::{types::TransactionMetadata, LiteSVM},
    solana_instruction::Instruction,
    solana_keypair::Keypair,
    solana_kite::{
        create_associated_token_account, create_token_mint, deploy_program, get_pda_and_bump,
        get_token_account_balance, mint_tokens_to_account, seeds,
    },
    solana_program::{native_token::LAMPORTS_PER_SOL, system_program},
    solana_pubkey::Pubkey,
    solana_signer::{signers::Signers, Signer},
    solana_transaction::Transaction,
    spl_associated_token_account::get_associated_token_address,
    strum::IntoEnumIterator,
};

pub struct ProgramId {
    // standard
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,

    // custom
    pub amm: Pubkey,
}

pub struct Pda {
    amm_program_id: Pubkey,
}

impl Pda {
    pub fn amm_pool_config(&self, pool_id: u64) -> Pubkey {
        get_pda_and_bump(&seeds!["config", pool_id], &self.amm_program_id).0
    }

    pub fn amm_pool_balance(&self, pool_id: u64) -> Pubkey {
        get_pda_and_bump(&seeds!["balance", pool_id], &self.amm_program_id).0
    }

    pub fn amm_mint_lp(&self, pool_id: u64) -> Pubkey {
        get_pda_and_bump(&seeds!["lp", pool_id], &self.amm_program_id).0
    }
}

pub struct App {
    pub litesvm: LiteSVM,
    token_registry: Vec<(AppToken, Keypair)>,

    pub program_id: ProgramId,
    pub pda: Pda,
}

impl App {
    pub fn create_app_with_programs() -> Self {
        // prepare environment with balances
        let (mut litesvm, token_registry) = Self::init_env_with_balances();

        // specify programs
        let program_id = ProgramId {
            // standard
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: AssociatedToken::id(),

            // custom
            amm: amm::ID,
        };

        // specify PDA
        let pda = Pda {
            amm_program_id: program_id.amm,
        };

        // upload custom programs
        upload_program(&mut litesvm, "amm", &program_id.amm);

        Self {
            litesvm,
            token_registry,

            program_id,
            pda,
        }
    }

    pub fn new() -> Self {
        let app = Self::create_app_with_programs();

        // prepare programs
        // ...

        app
    }

    fn init_env_with_balances() -> (LiteSVM, Vec<(AppToken, Keypair)>) {
        let mut litesvm = LiteSVM::new();
        let mut token_registry: Vec<(AppToken, Keypair)> = vec![];

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

            token_registry.push((token, mint));
        }

        // mint tokens
        for user in AppUser::iter() {
            for (token, mint) in &token_registry {
                let ata = create_associated_token_account(
                    &mut litesvm,
                    &user.keypair(),
                    &mint.pubkey(),
                    &AppUser::Admin.keypair(),
                )
                .unwrap();

                mint_tokens_to_account(
                    &mut litesvm,
                    &mint.pubkey(),
                    &ata,
                    user.get_initial_asset_amount() * 10u64.pow(token.get_decimals() as u32),
                    &AppUser::Admin.keypair(),
                )
                .unwrap();
            }
        }

        (litesvm, token_registry)
    }

    // utils

    // pub fn get_clock_time(&self) -> u64 {
    //     self.litesvm.get_sysvar()
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

    pub fn get_balance(&self, user: AppUser, asset: impl Into<AppAsset>) -> Result<u64> {
        let address = &user.pubkey();

        match asset.into() {
            AppAsset::Coin(_) => self.get_coin_balance(address),
            AppAsset::Token(mint) => self.get_token_balance(address, &mint.pubkey(self)),
        }
    }

    pub fn get_coin_balance(&self, address: &Pubkey) -> Result<u64> {
        self.litesvm
            .get_balance(address)
            .ok_or(to_anchor_err("SOL balance error"))
    }

    pub fn get_token_balance(&self, address: &Pubkey, mint: &Pubkey) -> Result<u64> {
        get_token_account_balance(&self.litesvm, &Self::get_ata(address, mint))
            .map_err(to_anchor_err)
    }

    pub fn get_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
        get_associated_token_address(owner, mint)
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

fn upload_program(litesvm: &mut LiteSVM, program_name: &str, program_id: &Pubkey) {
    const PROGRAM_PATH: &str = "../target/deploy/";

    deploy_program(
        litesvm,
        program_id,
        &format!("{}{}.so", PROGRAM_PATH, program_name),
    )
    .unwrap();
}

pub mod token {
    use super::*;

    pub trait WithTokenKeys {
        fn keypair(&self, app: &App) -> Keypair;
        fn pubkey(&self, app: &App) -> Pubkey;
    }

    impl WithTokenKeys for AppToken {
        fn keypair(&self, app: &App) -> Keypair {
            let base58_string = app
                .token_registry
                .iter()
                .find(|(token, _)| token == self)
                .map(|(_, keypair)| keypair.to_base58_string())
                .unwrap();

            Keypair::from_base58_string(&base58_string)
        }

        fn pubkey(&self, app: &App) -> Pubkey {
            app.token_registry
                .iter()
                .find(|(token, _)| token == self)
                .map(|(_, keypair)| keypair.pubkey())
                .unwrap()
        }
    }
}

pub mod extension {

    use super::*;

    pub fn get_data<T>(litesvm: &LiteSVM, pda: &Pubkey) -> Result<T>
    where
        T: AnchorDeserialize,
    {
        const DISCRIMINATOR_SPACE: usize = 8;
        let data = &mut &litesvm.get_account(pda).unwrap().data[DISCRIMINATOR_SPACE..];

        Ok(T::deserialize(data)?)
    }

    pub fn send_tx<S>(
        litesvm: &mut LiteSVM,
        instructions: &[Instruction],
        payer: &Pubkey,
        signers: &S,
    ) -> Result<TransactionMetadata>
    where
        S: Signers + ?Sized,
    {
        let transaction = Transaction::new_signed_with_payer(
            instructions,
            Some(payer),
            signers,
            litesvm.latest_blockhash(),
        );

        litesvm
            .send_transaction(transaction)
            .map_err(|e| to_anchor_err(e.err))
    }

    pub fn send_tx_with_ix<A, D, S>(
        app: &mut App,
        program_id: &Pubkey,
        accounts: &A,
        instruction_data: &D,
        payer: &Pubkey,
        signers: &S,
    ) -> Result<TransactionMetadata>
    where
        A: ToAccountMetas,
        D: InstructionData,
        S: Signers + ?Sized,
    {
        let ix = Instruction {
            program_id: *program_id,
            accounts: accounts.to_account_metas(None),
            data: instruction_data.data(),
        };

        send_tx(&mut app.litesvm, &[ix], payer, signers)
    }
}
