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
    dex_adapter::{accounts, instruction},
    dex_adapter_cpi::{state, types::RouteItem},
    litesvm::types::TransactionMetadata,
    solana_instruction::AccountMeta,
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
    ) -> Result<TransactionMetadata>;

    fn dex_adapter_try_save_route(
        &mut self,
        sender: AppUser,
        token_first: AppToken,
        token_last: AppToken,
        route: &[RouteItem],
    ) -> Result<TransactionMetadata>;

    fn dex_adapter_query_config(&self) -> Result<state::Config>;

    fn dex_adapter_query_admin_rotation_state(&self) -> Result<state::RotationState>;

    fn dex_adapter_query_route(
        &self,
        mint_first: &Pubkey,
        mint_last: &Pubkey,
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
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            token_program_2022,
            token_program,
            associated_token_program,
            memo,
            dex_adapter: program_id,
            clmm_mock,
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
        let route = self
            .pda
            .dex_adapter_route(input_token_mint, output_token_mint);

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
            clmm_mock_program: clmm_mock,
            sender: payer,
            bump,
            config,
            route,
            input_token_mint,
            output_token_mint,
            input_token_sender_ata,
            output_token_sender_ata,
            input_token_app_ata,
            output_token_app_ata,
        };

        // build remaining accounts based on the route loaded from PDA
        let remaining_accounts = build_remaining_accounts_for_route(
            self,
            sender,
            &payer,
            input_token_mint,
            output_token_mint,
        )?;

        let instruction_data = instruction::SwapMultihop {
            amount_in,
            amount_out_minimum,
        };

        send_tx_with_ix(
            self,
            &program_id,
            &accounts,
            &instruction_data,
            &payer,
            &signers,
            &remaining_accounts,
        )
    }

    fn dex_adapter_try_save_route(
        &mut self,
        sender: AppUser,
        token_first: AppToken,
        token_last: AppToken,
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
        let (mint_first, mint_last) = (token_first.pubkey(), token_last.pubkey());

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
        mint_first: &Pubkey,
        mint_last: &Pubkey,
    ) -> Result<state::Route> {
        get_data(
            &self.litesvm,
            &self.pda.dex_adapter_route(*mint_first, *mint_last),
        )
    }
}

fn build_remaining_accounts_for_route(
    app: &mut App,
    sender: AppUser,
    payer: &Pubkey,
    mint_in: Pubkey,
    mint_out: Pubkey,
) -> Result<Vec<AccountMeta>> {
    let config = app.pda.dex_adapter_config();
    let route_items = app.dex_adapter_query_route(&mint_in, &mint_out)?.value;

    // build token sequence correctly
    let token_sequence = route_items.iter().fold(vec![mint_in], |mut acc, cur| {
        acc.push(cur.token_out);
        acc
    });

    let mut remaining_accounts = vec![];

    // build accounts for each hop in the route
    for i in 0..route_items.len() {
        let token_a = token_sequence[i]; // input token for this hop
        let token_b = token_sequence[i + 1]; // output token for this hop

        // use the AMM config index from the current route item
        let amm_config_index = route_items[i].amm_index;

        // the pool was created with sorted tokens in prepare_dex, so we need to use sorted tokens to find it
        let (token_0_mint, token_1_mint) = sort_mints(&token_a, &token_b);

        let amm_config = app.pda.clmm_mock_amm_config(amm_config_index);
        let pool_state = app
            .pda
            .clmm_mock_pool_state(amm_config, token_0_mint, token_1_mint);
        let observation_state = app.pda.clmm_mock_observation_state(pool_state);

        // determine which vault corresponds to our input/output based on the original token order
        let (input_vault, output_vault, output_mint_for_accounts) = if token_a == token_0_mint {
            (
                app.pda.clmm_mock_token_vault_0(pool_state, token_0_mint),
                app.pda.clmm_mock_token_vault_1(pool_state, token_1_mint),
                token_1_mint,
            )
        } else {
            (
                app.pda.clmm_mock_token_vault_1(pool_state, token_1_mint),
                app.pda.clmm_mock_token_vault_0(pool_state, token_0_mint),
                token_0_mint,
            )
        };

        // for output token account:
        // - last hop goes to user
        // - intermediate hops go to app (config)
        let output_token_account = if i == route_items.len() - 1 {
            // final hop: output goes to user
            app.get_or_create_ata(sender, payer, &token_b)?
        } else {
            // intermediate hop: output goes to app for next hop input
            app.get_or_create_ata(sender, &config, &token_b)?
        };

        // match the exact account order and writability from clmm_mock
        remaining_accounts.extend([
            AccountMeta::new_readonly(amm_config, false), // amm_config (readonly)
            AccountMeta::new(pool_state, false),          // pool_state (writable)
            AccountMeta::new(output_token_account, false), // output_token_account (writable)
            AccountMeta::new(input_vault, false),         // input_vault (writable)
            AccountMeta::new(output_vault, false),        // output_vault (writable)
            AccountMeta::new_readonly(output_mint_for_accounts, false), // output_mint (readonly)
            AccountMeta::new(observation_state, false),   // observation_state (writable)
        ]);
    }

    Ok(remaining_accounts)
}
