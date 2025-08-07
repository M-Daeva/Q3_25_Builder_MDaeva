use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx_with_ix},
            App, ProgramId,
        },
        types::{AppToken, AppUser},
    },
    anchor_lang::Result,
    base::helpers::sort_mints,
    dex_adapter::{accounts, instruction, state, types::RouteItem},
    litesvm::types::TransactionMetadata,
    solana_pubkey::Pubkey,
};

pub trait DexAdapterExtension {
    fn dex_adapter_try_init(
        &mut self,
        sender: AppUser,
        dex: Pubkey,
        registry: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        token_in_whitelist: Option<Vec<AppToken>>,
    ) -> Result<TransactionMetadata>;

    fn dex_adapter_try_swap_multihop(
        &mut self,
        sender: AppUser,
        token_in: AppToken,
        token_out: AppToken,
        amount_in: u64,
        amount_out_minimum: u64,
        route_config_indexes: &[u16],
    ) -> Result<TransactionMetadata>;

    fn dex_adapter_try_save_route(
        &mut self,
        sender: AppUser,
        route: &[RouteItem],
    ) -> Result<TransactionMetadata>;

    fn dex_adapter_query_config(&self) -> Result<state::Config>;

    fn dex_adapter_query_admin_rotation_state(&self) -> Result<state::RotationState>;

    fn dex_adapter_query_route(
        &self,
        mint_first: AppToken,
        mint_last: AppToken,
    ) -> Result<state::Route>;
}

impl DexAdapterExtension for App {
    fn dex_adapter_try_init(
        &mut self,
        sender: AppUser,
        dex: Pubkey,
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
            dex,
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

    fn dex_adapter_try_swap_multihop(
        &mut self,
        sender: AppUser,
        token_in: AppToken,
        token_out: AppToken,
        amount_in: u64,
        amount_out_minimum: u64,
        route_config_indexes: &[u16],
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            token_program_2022,
            token_program,
            associated_token_program,
            memo,
            dex_adapter: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mints
        let (input_token_mint, output_token_mint) = (token_in.pubkey(), token_out.pubkey());

        // pda
        let bump = self.pda.dex_adapter_bump();
        let config = self.pda.dex_adapter_config();

        // ata
        let input_token_sender_ata = self.get_or_create_ata(sender, &payer, &input_token_mint)?;
        let output_token_sender_ata = self.get_or_create_ata(sender, &payer, &output_token_mint)?;

        let input_token_app_ata = self.get_or_create_ata(sender, &config, &input_token_mint)?;
        let output_token_app_ata = self.get_or_create_ata(sender, &config, &output_token_mint)?;

        let accounts = accounts::SwapMultihop {
            system_program,
            token_program,
            associated_token_program,
            token_program_2022,
            memo_program: memo,
            sender: payer,
            bump,
            config,
            input_token_mint,
            output_token_mint,
            input_token_sender_ata,
            output_token_sender_ata,
            input_token_app_ata,
            output_token_app_ata,
        };

        let instruction_data = instruction::SwapMultihop {
            amount_in,
            amount_out_minimum,
            route_config_indexes: route_config_indexes.to_vec(),
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

    fn dex_adapter_try_save_route(
        &mut self,
        sender: AppUser,
        route: &[RouteItem],
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

        // mints
        let mint_first = route.first().map(|x| x.token_out).unwrap_or_default();
        let mint_last = route.last().map(|x| x.token_out).unwrap_or_default();
        let (mint_first, mint_last) = sort_mints(&mint_first, &mint_last);

        // pda
        let bump = self.pda.dex_adapter_bump();
        let config = self.pda.dex_adapter_config();
        let route_pda = self.pda.dex_adapter_route(mint_first, mint_last);

        let accounts = accounts::SaveRoute {
            system_program,
            sender: payer,
            bump,
            config,
            route: route_pda,
        };

        let instruction_data = instruction::SaveRoute {
            mint_first,
            mint_last,
            route: route.to_vec(),
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

    fn dex_adapter_query_route(
        &self,
        mint_first: AppToken,
        mint_last: AppToken,
    ) -> Result<state::Route> {
        get_data(
            &self.litesvm,
            &self
                .pda
                .dex_adapter_route(mint_first.pubkey(), mint_last.pubkey()),
        )
    }
}
