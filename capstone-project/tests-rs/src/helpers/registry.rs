use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx_with_ix},
            token::WithTokenKeys,
            App, ProgramId,
        },
        types::{AppToken, AppUser},
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

    fn registry_try_update_common_config(
        &mut self,
        sender: AppUser,
        admin: Option<AppUser>,
        dex_adapter: Option<Pubkey>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
    ) -> Result<TransactionMetadata>;

    fn registry_try_update_account_config(
        &mut self,
        sender: AppUser,
        registration_fee: Option<AssetItem>,
        data_size_range: Option<Range>,
        lifetime_range: Option<Range>,
        lifetime_margin_bps: Option<u16>,
    ) -> Result<TransactionMetadata>;

    fn registry_try_confirm_admin_rotation(
        &mut self,
        sender: AppUser,
    ) -> Result<TransactionMetadata>;

    fn registry_try_withdraw_revenue(
        &mut self,
        sender: AppUser,
        amount: Option<u64>,
        recipient: Option<AppUser>,
        revenue_asset: Option<AppToken>,
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

    fn registry_try_update_common_config(
        &mut self,
        sender: AppUser,
        admin: Option<AppUser>,
        dex_adapter: Option<Pubkey>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let bump = self.pda.registry_bump();
        let common_config = self.pda.registry_common_config();
        let admin_rotation_state = self.pda.registry_admin_rotation_state();

        let accounts = accounts::UpdateCommonConfig {
            sender: payer,
            bump,
            common_config,
            admin_rotation_state,
        };

        let instruction_data = instruction::UpdateCommonConfig {
            admin: admin.map(|x| x.pubkey()),
            dex_adapter,
            is_paused,
            rotation_timeout,
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

    fn registry_try_update_account_config(
        &mut self,
        sender: AppUser,
        registration_fee: Option<AssetItem>,
        data_size_range: Option<Range>,
        lifetime_range: Option<Range>,
        lifetime_margin_bps: Option<u16>,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let bump = self.pda.registry_bump();
        let common_config = self.pda.registry_common_config();
        let account_config = self.pda.registry_account_config();

        let accounts = accounts::UpdateAccountConfig {
            sender: payer,
            bump,
            common_config,
            account_config,
        };

        let instruction_data = instruction::UpdateAccountConfig {
            registration_fee,
            data_size_range,
            lifetime_range,
            lifetime_margin_bps,
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

    fn registry_try_confirm_admin_rotation(
        &mut self,
        sender: AppUser,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let bump = self.pda.registry_bump();
        let common_config = self.pda.registry_common_config();
        let admin_rotation_state = self.pda.registry_admin_rotation_state();

        let accounts = accounts::ConfirmAdminRotation {
            sender: payer,
            bump,
            common_config,
            admin_rotation_state,
        };

        let instruction_data = instruction::ConfirmAdminRotation {};

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
        )
    }

    fn registry_try_withdraw_revenue(
        &mut self,
        sender: AppUser,
        amount: Option<u64>,
        recipient: Option<AppUser>,
        revenue_asset: Option<AppToken>, // to test asset guard
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            token_program,
            associated_token_program,
            registry: program_id,
            ..
        } = self.program_id;

        let recipient = recipient.unwrap_or(sender).pubkey();

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let revenue_mint = match revenue_asset {
            Some(x) => x.pubkey(&self),
            _ => self.registry_query_account_config()?.registration_fee.asset,
        };

        // pda
        let bump = self.pda.registry_bump();
        let common_config = self.pda.registry_common_config();
        let account_config = self.pda.registry_account_config();

        // ata
        let revenue_recipient_ata = App::get_ata(&recipient, &revenue_mint);
        let revenue_app_ata = App::get_ata(&common_config, &revenue_mint);

        let accounts = accounts::WithdrawRevenue {
            system_program,
            token_program,
            associated_token_program,
            sender: payer,
            recipient,
            bump,
            common_config,
            account_config,
            revenue_mint,
            revenue_recipient_ata,
            revenue_app_ata,
        };

        let instruction_data = instruction::WithdrawRevenue { amount };

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
