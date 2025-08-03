use {
    crate::helpers::suite::{
        core::{extension::send_tx_with_ix, to_anchor_err, App, ProgramId},
        types::AppUser,
    },
    anchor_lang::{
        prelude::{AccountInfo, AccountLoader},
        Result,
    },
    litesvm::types::TransactionMetadata,
    raydium_amm_v3::{accounts, instruction, states},
};

pub trait ClmmExtension {
    fn clmm_try_create_operation_account(&mut self, sender: AppUser)
        -> Result<TransactionMetadata>;

    fn clmm_query_operation_account(&self) -> Result<states::OperationState>;
}

impl ClmmExtension for App {
    fn clmm_try_create_operation_account(
        &mut self,
        sender: AppUser,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            clmm: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let operation_state = self.pda.clmm_operation_account();

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

    // AccountLoader is required for Zero-Copy accounts
    fn clmm_query_operation_account(&self) -> Result<states::OperationState> {
        let pda = &self.pda.clmm_operation_account();

        match self.litesvm.get_account(pda) {
            Some(mut account) => {
                // create a mock account info
                let account_info = AccountInfo::new(
                    pda,
                    false,
                    false,
                    &mut account.lamports,
                    &mut account.data,
                    &account.owner,
                    account.executable,
                    account.rent_epoch,
                );

                let loader = AccountLoader::try_from(&account_info)?;
                let state = loader.load()?;

                Ok(*state)
            }
            None => Err(to_anchor_err("Account data is not found!")),
        }
    }
}
