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
    clmm_mock::{accounts, instruction, state},
    litesvm::types::TransactionMetadata,
};

pub trait ClmmMockExtension {
    fn clmm_mock_try_create_pool(
        &mut self,
        sender: AppUser,
        id: u8,
        amount_and_token_a: (u64, AppToken),
        amount_and_token_b: (u64, AppToken),
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_query_pool_state(&self, id: u8) -> Result<state::PoolState>;
}

impl ClmmMockExtension for App {
    fn clmm_mock_try_create_pool(
        &mut self,
        sender: AppUser,
        id: u8,
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

        // pda
        let pool_state = self.pda.clmm_mock_pool_state(id);

        // ata
        let sender_a_ata = self.get_or_create_ata(sender, &payer, &mint_a)?;
        let sender_b_ata = self.get_or_create_ata(sender, &payer, &mint_b)?;
        let app_a_ata = self.get_or_create_ata(sender, &pool_state, &mint_a)?;
        let app_b_ata = self.get_or_create_ata(sender, &pool_state, &mint_b)?;

        let accounts = accounts::CreatePool {
            system_program,
            token_program,
            associated_token_program,
            sender: payer,
            pool_state,
            mint_a,
            mint_b,
            sender_a_ata,
            sender_b_ata,
            app_a_ata,
            app_b_ata,
        };

        let instruction_data = instruction::CreatePool {
            id,
            amount_a,
            amount_b,
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

    fn clmm_mock_query_pool_state(&self, id: u8) -> Result<state::PoolState> {
        get_data(&self.litesvm, &&self.pda.clmm_mock_pool_state(id))
    }
}
