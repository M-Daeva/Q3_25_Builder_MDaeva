use {
    crate::helpers::suite::{
        core::token::WithTokenKeys,
        types::{AppAsset, AppToken, AppUser, GetDecimals},
    },
    anchor_lang::{AnchorDeserialize, Id, InstructionData, Result, ToAccountMetas},
    anchor_spl::associated_token::AssociatedToken,
    dex_adapter,
    litesvm::{types::TransactionMetadata, LiteSVM},
    registry,
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
    pub registry: Pubkey,
    pub dex_adapter: Pubkey,
}

pub struct Pda {
    registry_program_id: Pubkey,
    dex_adapter_program_id: Pubkey,
}

impl Pda {
    pub fn registry_bump(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![registry::state::SEED_BUMP],
            &self.registry_program_id,
        )
        .0
    }

    pub fn registry_common_config(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![registry::state::SEED_COMMON_CONFIG],
            &self.registry_program_id,
        )
        .0
    }

    pub fn registry_account_config(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![registry::state::SEED_ACCOUNT_CONFIG],
            &self.registry_program_id,
        )
        .0
    }

    pub fn registry_user_counter(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![registry::state::SEED_USER_COUNTER],
            &self.registry_program_id,
        )
        .0
    }

    pub fn registry_admin_rotation_state(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![registry::state::SEED_ADMIN_ROTATION_STATE],
            &self.registry_program_id,
        )
        .0
    }

    pub fn dex_adapter_bump(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![dex_adapter::state::SEED_BUMP],
            &self.dex_adapter_program_id,
        )
        .0
    }

    pub fn dex_adapter_config(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![dex_adapter::state::SEED_CONFIG],
            &self.dex_adapter_program_id,
        )
        .0
    }

    pub fn dex_adapter_admin_rotation_state(&self) -> Pubkey {
        get_pda_and_bump(
            &seeds![dex_adapter::state::SEED_ADMIN_ROTATION_STATE],
            &self.dex_adapter_program_id,
        )
        .0
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
            registry: registry::ID,
            dex_adapter: dex_adapter::ID,
        };

        // specify PDA
        let pda = Pda {
            registry_program_id: program_id.registry,
            dex_adapter_program_id: program_id.dex_adapter,
        };

        // upload custom programs
        upload_program(&mut litesvm, "registry", &program_id.registry);
        upload_program(&mut litesvm, "dex_adapter", &program_id.dex_adapter);

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

        litesvm.send_transaction(transaction).map_err(|e| {
            let logs = e.meta.logs;
            let logs_str = format!("{:#?}", &logs);

            to_anchor_err(logs_str)
        })
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

pub fn assert_error(
    err: anchor_lang::error::Error,
    expected: impl ToString + Sized + std::fmt::Debug,
) {
    let expected_error_name = format!("{:#?}", expected);
    let expected_error_text = expected.to_string();

    let error = parse_anchor_err(&err);
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

/// Parses an Anchor error to extract the error code and message from program logs
/// Returns a formatted string like "Error Code: Unauthorized. Error Message: Sender doesn't have access permissions!"
fn parse_anchor_err(error: impl std::fmt::Debug) -> String {
    let error_msg = &format!("{:#?}", error);

    // Try to parse the error message as JSON (program logs)
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(error_msg) {
        if let serde_json::Value::Array(logs) = json_value {
            // Look for the AnchorError log entry
            for log in &logs {
                if let Some(log_str) = log.as_str() {
                    if log_str.contains("AnchorError occurred") {
                        return extract_error_details(log_str).unwrap_or_default();
                    }
                }
            }
        }
    }

    // If JSON parsing fails, try to find the error directly in the string
    if error_msg.contains("AnchorError occurred") {
        return extract_error_details(error_msg).unwrap_or_default();
    }

    String::default()
}

/// Extracts error code and message from an AnchorError log string
fn extract_error_details(log_str: &str) -> Option<String> {
    let error_code = extract_field(log_str, "Error Code: ", ".")?;
    let error_message = extract_field(log_str, "Error Message: ", ".")?;

    Some(format!("{} -> {}", error_code, error_message))
}

/// Helper function to extract a field between two delimiters
fn extract_field(text: &str, start_delimiter: &str, end_delimiter: &str) -> Option<String> {
    let start_pos = text.find(start_delimiter)? + start_delimiter.len();
    let remaining = &text[start_pos..];
    let end_pos = remaining.find(end_delimiter)?;
    Some(remaining[..end_pos].to_string())
}
