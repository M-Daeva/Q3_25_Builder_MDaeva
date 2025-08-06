use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, get_data_zero_copy, send_tx_with_ix},
            App, ProgramId,
        },
        decimal::Decimal,
        types::AppUser,
    },
    anchor_lang::Result,
    clmm_mock::{accounts, instruction, state},
    litesvm::types::TransactionMetadata,
    raydium_clmm_cpi,
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    spl_associated_token_account::get_associated_token_address,
};

pub trait ClmmMockExtension {
    fn clmm_mock_try_create_operation_account(
        &mut self,
        sender: AppUser,
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_try_create_amm_config(
        &mut self,
        sender: AppUser,
        index: u16,
        tick_spacing: u16,
        trade_fee_rate: u32,
        protocol_fee_rate: u32,
        fund_fee_rate: u32,
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_try_create_pool(
        &mut self,
        sender: AppUser,
        sqrt_price_x64: u128,
        open_time: u64,
        amm_config_index: u16,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_try_open_position(
        &mut self,
        sender: AppUser,
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        with_metadata: bool,
        base_flag: Option<bool>,
        amm_config_index: u16,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_try_swap(
        &mut self,
        sender: AppUser,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
        amm_config_index: u16,
        input_vault_mint: &Pubkey,
        output_vault_mint: &Pubkey,
    ) -> Result<TransactionMetadata>;

    fn clmm_mock_query_operation_account(&self) -> Result<state::OperationState>;

    fn clmm_mock_query_amm_config(&self, index: u16)
        -> Result<raydium_clmm_cpi::states::AmmConfig>;

    fn clmm_mock_query_pool_state(
        &self,
        amm_config: &Pubkey,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<raydium_clmm_cpi::states::PoolState>;
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

    fn clmm_mock_try_create_amm_config(
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
            clmm_mock: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // pda
        let amm_config = self.pda.clmm_mock_amm_config(index);

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

    fn clmm_mock_try_create_pool(
        &mut self,
        sender: AppUser,
        sqrt_price_x64: u128,
        open_time: u64,
        amm_config_index: u16,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            token_program,
            rent,
            clmm_mock: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let (token_mint_0, token_mint_1) = (*token_mint_0, *token_mint_1);

        // pda
        let amm_config = self.pda.clmm_mock_amm_config(amm_config_index);
        let pool_state = self
            .pda
            .clmm_mock_pool_state(amm_config, token_mint_0, token_mint_1);
        let token_vault_0 = self.pda.clmm_mock_token_vault_0(pool_state, token_mint_0);
        let token_vault_1 = self.pda.clmm_mock_token_vault_1(pool_state, token_mint_1);
        let observation_state = self.pda.clmm_mock_observation_state(pool_state);
        let tick_array_bitmap = self.pda.clmm_mock_tick_array_bitmap(pool_state);

        let accounts = accounts::CreatePool {
            pool_creator: payer,
            amm_config,
            pool_state,
            token_mint_0,
            token_mint_1,
            token_vault_0,
            token_vault_1,
            observation_state,
            tick_array_bitmap,
            token_program_0: token_program,
            token_program_1: token_program,
            system_program,
            rent,
        };

        let instruction_data = instruction::CreatePool {
            sqrt_price_x64,
            open_time,
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

    fn clmm_mock_try_open_position(
        &mut self,
        sender: AppUser,
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        with_metadata: bool,
        base_flag: Option<bool>,
        amm_config_index: u16,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            system_program,
            token_program_2022,
            token_program,
            associated_token_program,
            rent,
            clmm_mock: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();

        // generate new keypair for position NFT mint
        let position_nft_mint_keypair = Keypair::new();
        let position_nft_mint = position_nft_mint_keypair.pubkey();

        // include position_nft_mint in signers
        let signers = [sender.keypair(), position_nft_mint_keypair];

        // mint addresses
        let (token_mint_0, token_mint_1) = (*token_mint_0, *token_mint_1);

        // pda
        let amm_config = self.pda.clmm_mock_amm_config(amm_config_index);
        let pool_state = self
            .pda
            .clmm_mock_pool_state(amm_config, token_mint_0, token_mint_1);
        let token_vault_0 = self.pda.clmm_mock_token_vault_0(pool_state, token_mint_0);
        let token_vault_1 = self.pda.clmm_mock_token_vault_1(pool_state, token_mint_1);

        let tick_array_lower = self
            .pda
            .clmm_mock_tick_array_lower(pool_state, tick_array_lower_start_index);
        let tick_array_upper = self
            .pda
            .clmm_mock_tick_array_upper(pool_state, tick_array_upper_start_index);

        let personal_position = self.pda.clmm_mock_personal_position(position_nft_mint);

        // ata
        // position_nft_account will be created during instruction execution
        let position_nft_account = get_associated_token_address(&payer, &position_nft_mint);
        let token_account_0 = self.get_or_create_ata(sender, &payer, &token_mint_0)?;
        let token_account_1 = self.get_or_create_ata(sender, &payer, &token_mint_1)?;

        let accounts = accounts::OpenPositionWithToken22Nft {
            payer,
            position_nft_owner: payer,
            position_nft_mint,
            position_nft_account,
            pool_state,
            protocol_position: Pubkey::default(), // deprecated field
            tick_array_lower,
            tick_array_upper,
            personal_position,
            token_account_0,
            token_account_1,
            token_vault_0,
            token_vault_1,
            rent,
            system_program,
            token_program,
            associated_token_program,
            token_program_2022,
            vault_0_mint: token_mint_0,
            vault_1_mint: token_mint_1,
        };

        let instruction_data = instruction::OpenPositionWithToken22Nft {
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
            with_metadata,
            base_flag,
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

    fn clmm_mock_try_swap(
        &mut self,
        sender: AppUser,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
        amm_config_index: u16,
        input_vault_mint: &Pubkey,
        output_vault_mint: &Pubkey,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            token_program_2022,
            token_program,
            memo,
            clmm_mock: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();

        // include position_nft_mint in signers
        let signers = [sender.keypair()];

        // mint addresses
        let (input_vault_mint, output_vault_mint) = (*input_vault_mint, *output_vault_mint);

        // pda
        let amm_config = self.pda.clmm_mock_amm_config(amm_config_index);
        let pool_state =
            self.pda
                .clmm_mock_pool_state(amm_config, input_vault_mint, output_vault_mint);
        let input_vault = self
            .pda
            .clmm_mock_token_vault_0(pool_state, input_vault_mint);
        let output_vault = self
            .pda
            .clmm_mock_token_vault_1(pool_state, output_vault_mint);
        let observation_state = self.pda.clmm_mock_observation_state(pool_state);

        // ata
        let input_token_account = self.get_or_create_ata(sender, &payer, &input_vault_mint)?;
        let output_token_account = self.get_or_create_ata(sender, &payer, &output_vault_mint)?;

        let accounts = accounts::SwapSingleV2 {
            payer,
            amm_config,
            pool_state,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            observation_state,
            token_program,
            token_program_2022,
            memo_program: memo,
            input_vault_mint,
            output_vault_mint,
        };

        let instruction_data = instruction::SwapV2 {
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
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

    fn clmm_mock_query_operation_account(&self) -> Result<state::OperationState> {
        get_data_zero_copy(&self.litesvm, &self.pda.clmm_mock_operation_account())
    }

    fn clmm_mock_query_amm_config(
        &self,
        index: u16,
    ) -> Result<raydium_clmm_cpi::states::AmmConfig> {
        get_data(&self.litesvm, &self.pda.clmm_mock_amm_config(index))
    }

    fn clmm_mock_query_pool_state(
        &self,
        amm_config: &Pubkey,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<raydium_clmm_cpi::states::PoolState> {
        get_data_zero_copy(
            &self.litesvm,
            &self
                .pda
                .clmm_mock_pool_state(*amm_config, *token_mint_0, *token_mint_1),
        )
    }
}

/// returns src data sorted by mint
pub fn get_token_info_for_pool_creation(
    token_info_list: &[(Pubkey, u8, Decimal)], // (mint, decimals, price)
) -> Vec<(Pubkey, u8, Decimal)> {
    let mut mint_list: Vec<_> = token_info_list.iter().map(|(x, _, _)| *x).collect();
    mint_list.sort_unstable();

    mint_list
        .iter()
        .map(|mint| {
            let (_, decimals, price) = token_info_list.iter().find(|(x, _, _)| x == mint).unwrap();

            (*mint, *decimals, *price)
        })
        .collect()
}

pub fn sort_token_mints(mint_a: &Pubkey, mint_b: &Pubkey) -> (Pubkey, Pubkey) {
    if mint_a < mint_b {
        (*mint_a, *mint_b)
    } else {
        (*mint_b, *mint_a)
    }
}

pub fn calc_token_amount_for_pool(token_decimals: u8, token_price: Decimal) -> u64 {
    const BASE_AMOUNT: u128 = 1_000_000; // $

    let price_atomics = token_price.atomics();
    let dec_multiplier = 10_u128.pow(token_decimals as u32);

    (dec_multiplier * (BASE_AMOUNT * Decimal::DECIMAL_FRACTIONAL / price_atomics)) as u64
}
