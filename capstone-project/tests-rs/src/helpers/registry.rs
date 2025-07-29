use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx_with_ix},
            App, ProgramId,
        },
        types::AppUser,
    },
    anchor_lang::Result,
    litesvm::types::TransactionMetadata,
    registry::{
        accounts, instruction, state,
        types::{AssetItem, Range},
    },
    solana_pubkey::Pubkey,
};

pub trait RegistryExtension {
    fn registry_try_init(
        &mut self,
        sender: AppUser,
        dex_adapter: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
        account_lifetime_range: Option<Range>,
        account_lifetime_margin_bps: Option<u16>,
    ) -> Result<TransactionMetadata>;

    fn registry_query_common_config(&self) -> Result<state::CommonConfig>;

    fn registry_query_account_config(&self) -> Result<state::AccountConfig>;

    fn registry_query_user_counter(&self) -> Result<state::UserCounter>;

    fn registry_query_admin_rotation_state(&self) -> Result<state::RotationState>;
}

impl RegistryExtension for App {
    fn registry_try_init(
        &mut self,
        sender: AppUser,
        dex_adapter: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
        account_lifetime_range: Option<Range>,
        account_lifetime_margin_bps: Option<u16>,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            token_program,
            associated_token_program,
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let revenue_mint = account_registration_fee
            .as_ref()
            .map(|x| x.asset.clone())
            .unwrap();

        // pda
        let bump = self.pda.registry_bump();
        let common_config = self.pda.registry_common_config();
        let account_config = self.pda.registry_account_config();
        let user_counter = self.pda.registry_user_counter();
        let admin_rotation_state = self.pda.registry_admin_rotation_state();

        // ata
        let revenue_app_ata = App::get_ata(&bump, &revenue_mint);

        let accounts = accounts::Init {
            system_program,
            token_program,
            associated_token_program,
            sender: payer,
            bump,
            common_config,
            account_config,
            user_counter,
            admin_rotation_state,
            revenue_mint,
            revenue_app_ata,
        };

        let instruction_data = instruction::Init {
            dex_adapter,
            rotation_timeout,
            account_registration_fee,
            account_data_size_range,
            account_lifetime_range,
            account_lifetime_margin_bps,
        };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
        )
    }

    fn registry_query_common_config(&self) -> Result<state::CommonConfig> {
        get_data(&self.litesvm, &self.pda.registry_common_config())
    }

    fn registry_query_account_config(&self) -> Result<state::AccountConfig> {
        get_data(&self.litesvm, &&self.pda.registry_account_config())
    }

    fn registry_query_user_counter(&self) -> Result<state::UserCounter> {
        get_data(&self.litesvm, &self.pda.registry_user_counter())
    }

    fn registry_query_admin_rotation_state(&self) -> Result<state::RotationState> {
        get_data(&self.litesvm, &self.pda.registry_admin_rotation_state())
    }
}
