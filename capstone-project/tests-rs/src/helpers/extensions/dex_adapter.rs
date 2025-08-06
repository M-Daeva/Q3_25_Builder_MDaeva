use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx_with_ix},
            App, ProgramId,
        },
        types::{AppToken, AppUser},
    },
    anchor_lang::Result,
    dex_adapter::{accounts, instruction, state},
    litesvm::types::TransactionMetadata,
    solana_pubkey::Pubkey,
};

pub trait DexAdapterExtension {
    fn dex_adapter_try_init(
        &mut self,
        sender: AppUser,
        registry: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        token_in_whitelist: Option<Vec<AppToken>>,
    ) -> Result<TransactionMetadata>;

    fn dex_adapter_query_config(&self) -> Result<state::Config>;

    fn dex_adapter_query_admin_rotation_state(&self) -> Result<state::RotationState>;
}

impl DexAdapterExtension for App {
    fn dex_adapter_try_init(
        &mut self,
        sender: AppUser,
        registry: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        token_in_whitelist: Option<Vec<AppToken>>,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            dex_adapter: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let bump = self.pda.dex_adapter_bump();
        let config = self.pda.dex_adapter_config();
        let admin_rotation_state = self.pda.dex_adapter_admin_rotation_state();

        let accounts = accounts::Init {
            system_program,
            sender: payer,
            bump,
            config,
            admin_rotation_state,
        };

        let instruction_data = instruction::Init {
            registry,
            rotation_timeout,
            token_in_whitelist: token_in_whitelist.map(|x| x.iter().map(|y| y.pubkey()).collect()),
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

    fn dex_adapter_query_config(&self) -> Result<state::Config> {
        get_data(&self.litesvm, &self.pda.dex_adapter_config())
    }

    fn dex_adapter_query_admin_rotation_state(&self) -> Result<state::RotationState> {
        get_data(&self.litesvm, &self.pda.dex_adapter_admin_rotation_state())
    }
}
