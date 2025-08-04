use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, get_data_zero_copy, send_tx_with_ix},
            App, ProgramId,
        },
        types::AppUser,
    },
    anchor_lang::Result,
    litesvm::types::TransactionMetadata,
    raydium_amm_v3::{accounts, instruction, states},
};

pub trait ClmmExtension {
    fn clmm_try_create_operation_account(&mut self, sender: AppUser)
        -> Result<TransactionMetadata>;

    fn clmm_try_create_amm_config(
        &mut self,
        sender: AppUser,
        index: u16,
        tick_spacing: u16,
        trade_fee_rate: u32,
        protocol_fee_rate: u32,
        fund_fee_rate: u32,
    ) -> Result<TransactionMetadata>;

    fn clmm_query_operation_account(&self) -> Result<states::OperationState>;

    fn clmm_query_amm_config(&self, index: u16) -> Result<states::AmmConfig>;
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

    fn clmm_try_create_amm_config(
        &mut self,
        sender: AppUser,
        index: u16,
        tick_spacing: u16,
        trade_fee_rate: u32,
        protocol_fee_rate: u32,
        fund_fee_rate: u32,
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
        let amm_config = self.pda.clmm_amm_config(index);

        let accounts = accounts::CreateAmmConfig {
            owner: payer,
            amm_config,
            system_program,
        };

        let instruction_data = instruction::CreateAmmConfig {
            index,
            tick_spacing,
            trade_fee_rate,
            protocol_fee_rate,
            fund_fee_rate,
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

    fn clmm_query_operation_account(&self) -> Result<states::OperationState> {
        get_data_zero_copy(&self.litesvm, &self.pda.clmm_operation_account())
    }

    fn clmm_query_amm_config(&self, index: u16) -> Result<states::AmmConfig> {
        get_data(&self.litesvm, &self.pda.clmm_amm_config(index))
    }
}
