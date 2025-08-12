use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx_with_ix},
            App, ProgramId,
        },
        types::{AppToken, AppUser},
    },
    anchor_lang::Result,
    litesvm::types::TransactionMetadata,
    registry::{accounts, instruction},
    registry_cpi::{
        state::{self, ACCOUNT_REGISTRATION_FEE_ASSET},
        types::{AssetItem, Range},
    },
};

pub trait RegistryExtension {
    fn registry_try_init(
        &mut self,
        sender: AppUser,
        rotation_timeout: Option<u32>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
    ) -> Result<TransactionMetadata>;

    fn registry_try_update_config(
        &mut self,
        sender: AppUser,
        admin: Option<AppUser>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
        registration_fee_amount: Option<u64>,
        data_size_range: Option<Range>,
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

    fn registry_try_create_account(
        &mut self,
        sender: AppUser,
        max_data_size: u32,
    ) -> Result<TransactionMetadata>;

    fn registry_try_close_account(&mut self, sender: AppUser) -> Result<TransactionMetadata>;

    fn registry_try_reopen_account(
        &mut self,
        sender: AppUser,
        max_data_size: u32,
    ) -> Result<TransactionMetadata>;

    fn registry_try_activate_account(
        &mut self,
        sender: AppUser,
        user: Option<AppUser>,
        revenue_asset: Option<AppToken>, // to test asset guard
    ) -> Result<TransactionMetadata>;

    fn registry_try_write_data(
        &mut self,
        sender: AppUser,
        data: &str,
        nonce: u64,
    ) -> Result<TransactionMetadata>;

    fn registry_try_request_account_rotation(
        &mut self,
        sender: AppUser,
        new_owner: AppUser,
    ) -> Result<TransactionMetadata>;

    fn registry_try_confirm_account_rotation(
        &mut self,
        sender: AppUser,
        prev_owner: AppUser,
    ) -> Result<TransactionMetadata>;

    fn registry_query_config(&self) -> Result<state::Config>;

    fn registry_query_user_counter(&self) -> Result<state::UserCounter>;

    fn registry_query_admin_rotation_state(&self) -> Result<state::RotationState>;

    fn registry_query_user_id(&self, user: AppUser) -> Result<state::UserId>;

    fn registry_query_user_account(&self, user_id: u32) -> Result<state::UserAccount>;

    fn registry_query_user_rotation_state(&self, user_id: u32) -> Result<state::RotationState>;
}

impl RegistryExtension for App {
    fn registry_try_init(
        &mut self,
        sender: AppUser,
        rotation_timeout: Option<u32>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
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
            .map(|x| x.asset)
            .unwrap_or(ACCOUNT_REGISTRATION_FEE_ASSET);

        // pda
        let bump = self.pda.registry_bump();
        let config = self.pda.registry_config();
        let user_counter = self.pda.registry_user_counter();
        let admin_rotation_state = self.pda.registry_admin_rotation_state();

        // ata
        let revenue_app_ata = App::get_ata(&config, &revenue_mint);

        let accounts = accounts::Init {
            system_program,
            token_program,
            associated_token_program,
            sender: payer,
            bump,
            config,
            user_counter,
            admin_rotation_state,
            revenue_mint,
            revenue_app_ata,
        };

        let instruction_data = instruction::Init {
            rotation_timeout,
            account_registration_fee,
            account_data_size_range,
        };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_try_update_config(
        &mut self,
        sender: AppUser,
        admin: Option<AppUser>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
        registration_fee_amount: Option<u64>,
        data_size_range: Option<Range>,
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
        let config = self.pda.registry_config();
        let admin_rotation_state = self.pda.registry_admin_rotation_state();

        let accounts = accounts::UpdateConfig {
            sender: payer,
            bump,
            config,
            admin_rotation_state,
        };

        let instruction_data = instruction::UpdateConfig {
            admin: admin.map(|x| x.pubkey()),
            is_paused,
            rotation_timeout,
            registration_fee_amount,
            data_size_range,
        };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
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
        let config = self.pda.registry_config();
        let admin_rotation_state = self.pda.registry_admin_rotation_state();

        let accounts = accounts::ConfirmAdminRotation {
            sender: payer,
            bump,
            config,
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
            &[],
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
            Some(x) => x.pubkey(),
            _ => self.registry_query_config()?.registration_fee.asset,
        };

        // pda
        let bump = self.pda.registry_bump();
        let config = self.pda.registry_config();

        // ata
        let revenue_recipient_ata = App::get_ata(&recipient, &revenue_mint);
        let revenue_app_ata = App::get_ata(&config, &revenue_mint);

        let accounts = accounts::WithdrawRevenue {
            system_program,
            token_program,
            associated_token_program,
            sender: payer,
            recipient,
            bump,
            config,
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
            &[],
        )
    }

    fn registry_try_create_account(
        &mut self,
        sender: AppUser,
        max_data_size: u32,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let bump = self.pda.registry_bump();
        let config = self.pda.registry_config();
        let user_counter = self.pda.registry_user_counter();

        let user_id = self.pda.registry_user_id(payer);
        let expected_user_id = self.registry_query_user_counter()?.last_user_id + 1;
        let user_account = self.pda.registry_user_account(expected_user_id);
        let user_rotation_state = self.pda.registry_user_rotation_state(expected_user_id);

        let accounts = accounts::CreateAccount {
            system_program,
            sender: payer,
            bump,
            config,
            user_counter,
            user_id,
            user_account,
            user_rotation_state,
        };

        let instruction_data = instruction::CreateAccount {
            max_data_size,
            expected_user_id,
        };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_try_close_account(&mut self, sender: AppUser) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let user_id = self.pda.registry_user_id(payer);
        let id = self.registry_query_user_id(sender)?.id;
        let user_account = self.pda.registry_user_account(id);
        let user_rotation_state = self.pda.registry_user_rotation_state(id);

        let accounts = accounts::CloseAccount {
            system_program,
            sender: payer,
            user_id,
            user_account,
            user_rotation_state,
        };

        let instruction_data = instruction::CloseAccount {};

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_try_reopen_account(
        &mut self,
        sender: AppUser,
        max_data_size: u32,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let bump = self.pda.registry_bump();
        let config = self.pda.registry_config();

        let user_id = self.pda.registry_user_id(payer);
        let id = self.registry_query_user_id(sender)?.id;
        let user_account = self.pda.registry_user_account(id);
        let user_rotation_state = self.pda.registry_user_rotation_state(id);

        let accounts = accounts::ReopenAccount {
            system_program,
            sender: payer,
            bump,
            config,
            user_id,
            user_account,
            user_rotation_state,
        };

        let instruction_data = instruction::ReopenAccount { max_data_size };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_try_activate_account(
        &mut self,
        sender: AppUser,
        user: Option<AppUser>,
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

        let user = user.unwrap_or(sender).pubkey();

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let revenue_mint = match revenue_asset {
            Some(x) => x.pubkey(),
            _ => self.registry_query_config()?.registration_fee.asset,
        };

        // pda
        let bump = self.pda.registry_bump();
        let config = self.pda.registry_config();

        let user_id = self.pda.registry_user_id(payer);

        // ata
        let revenue_sender_ata = App::get_ata(&payer, &revenue_mint);
        let revenue_app_ata = App::get_ata(&config, &revenue_mint);

        let accounts = accounts::ActivateAccount {
            system_program,
            token_program,
            associated_token_program,
            sender: payer,
            bump,
            config,
            user_id,
            revenue_mint,
            revenue_sender_ata,
            revenue_app_ata,
        };

        let instruction_data = instruction::ActivateAccount { user };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_try_write_data(
        &mut self,
        sender: AppUser,
        data: &str,
        nonce: u64,
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
        let user_id = self.pda.registry_user_id(payer);
        let id = self.registry_query_user_id(sender)?.id;
        let user_account = self.pda.registry_user_account(id);

        let accounts = accounts::WriteData {
            sender: payer,
            user_id,
            user_account,
        };

        let instruction_data = instruction::WriteData {
            data: data.to_string(),
            nonce,
        };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_try_request_account_rotation(
        &mut self,
        sender: AppUser,
        new_owner: AppUser,
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
        let config = self.pda.registry_config();

        let user_id = self.pda.registry_user_id(payer);
        let id = self.registry_query_user_id(sender)?.id;
        let user_rotation_state = self.pda.registry_user_rotation_state(id);

        let accounts = accounts::RequestAccountRotation {
            sender: payer,
            bump,
            config,
            user_id,
            user_rotation_state,
        };

        let instruction_data = instruction::RequestAccountRotation {
            new_owner: new_owner.pubkey(),
        };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_try_confirm_account_rotation(
        &mut self,
        sender: AppUser,
        prev_owner: AppUser,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            registry: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let user_id_pre = self.pda.registry_user_id(prev_owner.pubkey());
        let user_id = self.pda.registry_user_id(payer);
        let user_id_value_pre = self.registry_query_user_id(prev_owner)?.id;
        let user_rotation_state = self.pda.registry_user_rotation_state(user_id_value_pre);

        let accounts = accounts::ConfirmAccountRotation {
            system_program,
            sender: payer,
            user_id_pre,
            user_id,
            user_rotation_state,
        };

        let instruction_data = instruction::ConfirmAccountRotation {};

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &[],
        )
    }

    fn registry_query_config(&self) -> Result<state::Config> {
        get_data(&self.litesvm, &self.pda.registry_config())
    }

    fn registry_query_user_counter(&self) -> Result<state::UserCounter> {
        get_data(&self.litesvm, &self.pda.registry_user_counter())
    }

    fn registry_query_admin_rotation_state(&self) -> Result<state::RotationState> {
        get_data(&self.litesvm, &self.pda.registry_admin_rotation_state())
    }

    fn registry_query_user_id(&self, user: AppUser) -> Result<state::UserId> {
        get_data(&self.litesvm, &self.pda.registry_user_id(user.pubkey()))
    }

    fn registry_query_user_account(&self, user_id: u32) -> Result<state::UserAccount> {
        get_data(&self.litesvm, &self.pda.registry_user_account(user_id))
    }

    fn registry_query_user_rotation_state(&self, user_id: u32) -> Result<state::RotationState> {
        get_data(
            &self.litesvm,
            &self.pda.registry_user_rotation_state(user_id),
        )
    }
}
