use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, get_data_zero_copy, send_tx_with_ix},
            token::WithTokenKeys,
            App, ProgramId,
        },
        types::{AppToken, AppUser},
    },
    anchor_lang::Result,
    clmm_mock::{accounts, instruction, instructions::sort_token_mints, state},
    litesvm::types::TransactionMetadata,
    solana_pubkey::Pubkey,
};

pub trait ClmmMockExtension {
    fn clmm_mock_try_create_operation_account(
        &mut self,
        sender: AppUser,
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_try_create_pool(
        &mut self,
        sender: AppUser,
        amount_and_token_a: (u64, AppToken),
        amount_and_token_b: (u64, AppToken),
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_query_operation_account(&self) -> Result<state::OperationState>;

    fn clmm_mock_query_pool_state(
        &self,
        mint_a: &Pubkey,
        mint_b: &Pubkey,
    ) -> Result<state::PoolConfig>;
}

impl ClmmMockExtension for App {
    fn clmm_mock_try_create_operation_account(
        &mut self,
        sender: AppUser,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            clmm_mock: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let operation_state = self.pda.clmm_mock_operation_account();

        let accounts = accounts::CreateOperationAccount {
            owner: payer,
            operation_state,
            system_program,
        };

        let instruction_data = instruction::CreateOperationAccount {};

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
        )
    }

    fn clmm_mock_try_create_pool(
        &mut self,
        sender: AppUser,
        amount_and_token_a: (u64, AppToken),
        amount_and_token_b: (u64, AppToken),
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            token_program,
            associated_token_program,
            clmm_mock: program_id,
            ..
        } = self.program_id;

        let (amount_a, token_a) = amount_and_token_a;
        let (amount_b, token_b) = amount_and_token_b;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let mint_a = token_a.pubkey(self);
        let mint_b = token_b.pubkey(self);

        // make sure tokens are sorted initially
        let (mint_a_sorted, _) = sort_token_mints(&mint_a, &mint_b);
        if mint_a_sorted != mint_a {
            panic!("Tokens aren't sorted!");
        }

        // pda
        let pool_config = self.pda.clmm_mock_pool_config(&mint_a, &mint_b);

        // ata
        let sender_a_ata = self.get_or_create_ata(sender, &payer, &mint_a)?;
        let sender_b_ata = self.get_or_create_ata(sender, &payer, &mint_b)?;
        let app_a_ata = self.get_or_create_ata(sender, &pool_config, &mint_a)?;
        let app_b_ata = self.get_or_create_ata(sender, &pool_config, &mint_b)?;

        let accounts = accounts::CreatePool {
            system_program,
            token_program,
            associated_token_program,
            sender: payer,
            pool_config,
            mint_a,
            mint_b,
            sender_a_ata,
            sender_b_ata,
            app_a_ata,
            app_b_ata,
        };

        let instruction_data = instruction::CreatePool { amount_a, amount_b };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
        )
    }

    fn clmm_mock_query_operation_account(&self) -> Result<state::OperationState> {
        get_data_zero_copy(&self.litesvm, &self.pda.clmm_mock_operation_account())
    }

    fn clmm_mock_query_pool_state(
        &self,
        mint_a: &Pubkey,
        mint_b: &Pubkey,
    ) -> Result<state::PoolConfig> {
        get_data(
            &self.litesvm,
            &&self.pda.clmm_mock_pool_config(&mint_a, &mint_b),
        )
    }
}
