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

        let instruction_data = instruction::SwapMultihop {
            amount_in,
            amount_out_minimum,
        };

        // We still need remaining accounts for the multihop swap
        // Build remaining accounts based on the route loaded from PDA
        let remaining_accounts = build_remaining_accounts_for_route(
            self,
            sender,
            &payer,
            input_token_mint,
            output_token_mint,
        )?;

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

// fn build_remaining_accounts_for_route(
//     app: &mut App,
//     sender: AppUser,
//     payer: &Pubkey,
//     mint_in: Pubkey,
//     mint_out: Pubkey,
// ) -> Result<Vec<AccountMeta>> {
//     // Load route data
//     let route_data = app.dex_adapter_query_route(&mint_in, &mint_out)?;
//     let route_items = &route_data.value;

//     // build token sequence
//     let mut token_sequence: Vec<Pubkey> = vec![mint_in];
//     for item in route_items.iter().skip(1) {
//         token_sequence.push(item.token_out);
//     }

//     let mut remaining_accounts = vec![];

//     // Build accounts for each hop in the route
//     for i in 0..token_sequence.len() - 1 {
//         let token_a = token_sequence[i];
//         let token_b = token_sequence[i + 1];

//         // Get the appropriate config index for this hop
//         let amm_config_index = route_items[i + 1].amm_index;

//         let (token_0_mint, token_1_mint) = sort_mints(&token_a, &token_b);

//         let amm_config = app.pda.clmm_mock_amm_config(amm_config_index);
//         let pool_state = app
//             .pda
//             .clmm_mock_pool_state(amm_config, token_0_mint, token_1_mint);
//         let input_vault = app.pda.clmm_mock_token_vault_0(pool_state, token_0_mint);
//         let output_vault = app.pda.clmm_mock_token_vault_1(pool_state, token_1_mint);
//         let observation_state = app.pda.clmm_mock_observation_state(pool_state);
//         let output_token_account = app.get_or_create_ata(sender, payer, &token_b)?;

//         remaining_accounts.extend(vec![
//             AccountMeta::new_readonly(amm_config, false),
//             AccountMeta::new(pool_state, false),
//             AccountMeta::new(output_token_account, false),
//             AccountMeta::new(input_vault, false),
//             AccountMeta::new(output_vault, false),
//             AccountMeta::new_readonly(token_b, false),
//             AccountMeta::new(observation_state, false),
//         ]);
//     }

//     Ok(remaining_accounts)
// }

fn build_remaining_accounts_for_route(
    app: &mut App,
    sender: AppUser,
    payer: &Pubkey,
    mint_in: Pubkey,
    mint_out: Pubkey,
) -> Result<Vec<AccountMeta>> {
    // Load route data
    let route_data = app.dex_adapter_query_route(&mint_in, &mint_out)?;
    let route_items = &route_data.value;

    // Build token sequence correctly - start with mint_in, then add each token_out from route
    let mut token_sequence: Vec<Pubkey> = vec![mint_in];
    for item in route_items.iter() {
        token_sequence.push(item.token_out);
    }

    let mut remaining_accounts = vec![];

    // Build accounts for each hop in the route
    for i in 0..token_sequence.len() - 1 {
        let token_a = token_sequence[i];
        let token_b = token_sequence[i + 1];

        // Use the correct AMM config index for this hop
        let amm_config_index = route_items[i].amm_index;

        let (token_0_mint, token_1_mint) = sort_mints(&token_a, &token_b);

        let amm_config = app.pda.clmm_mock_amm_config(amm_config_index);
        let pool_state = app
            .pda
            .clmm_mock_pool_state(amm_config, token_0_mint, token_1_mint);

        // Make sure we're using the correct vault assignment based on token order
        let (input_vault, output_vault) = if token_a == token_0_mint {
            (
                app.pda.clmm_mock_token_vault_0(pool_state, token_0_mint),
                app.pda.clmm_mock_token_vault_1(pool_state, token_1_mint),
            )
        } else {
            (
                app.pda.clmm_mock_token_vault_1(pool_state, token_1_mint),
                app.pda.clmm_mock_token_vault_0(pool_state, token_0_mint),
            )
        };

        let observation_state = app.pda.clmm_mock_observation_state(pool_state);
        let output_token_account = app.get_or_create_ata(sender, payer, &token_b)?;

        remaining_accounts.extend(vec![
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(output_token_account, false),
            AccountMeta::new(input_vault, false),
            AccountMeta::new(output_vault, false),
            AccountMeta::new_readonly(token_b, false),
            AccountMeta::new(observation_state, false),
        ]);
    }

    // TODO
    println!("Route items: {:?}", route_data.value);
    println!("Token sequence: {:?}", token_sequence);
    println!("Remaining accounts count: {}\n", remaining_accounts.len());

    Ok(remaining_accounts)
}
