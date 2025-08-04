use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, get_data_zero_copy, send_tx_with_ix},
            App, ProgramId,
        },
        decimal::{u128_to_dec, Decimal},
        types::AppUser,
    },
    anchor_lang::Result,
    litesvm::types::TransactionMetadata,
    raydium_amm_v3::{accounts, instruction, states},
    solana_pubkey::Pubkey,
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

    fn clmm_try_create_pool(
        &mut self,
        sender: AppUser,
        sqrt_price_x64: u128,
        open_time: u64,
        amm_config_index: u16,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<TransactionMetadata>;

    fn clmm_query_operation_account(&self) -> Result<states::OperationState>;

    fn clmm_query_amm_config(&self, index: u16) -> Result<states::AmmConfig>;

    fn clmm_query_pool_state(
        &self,
        amm_config: &Pubkey,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<states::PoolState>;
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

    fn clmm_try_create_pool(
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
            clmm: program_id,
            ..
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let (token_mint_0, token_mint_1) = (*token_mint_0, *token_mint_1);

        // pda
        let amm_config = self.pda.clmm_amm_config(amm_config_index);
        let pool_state = self
            .pda
            .clmm_pool_state(amm_config, token_mint_0, token_mint_1);
        let token_vault_0 = self.pda.clmm_token_vault_0(pool_state, token_mint_0);
        let token_vault_1 = self.pda.clmm_token_vault_1(pool_state, token_mint_1);
        let observation_state = self.pda.clmm_observation_state(pool_state);
        let tick_array_bitmap = self.pda.clmm_tick_array_bitmap(pool_state);

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

    fn clmm_query_operation_account(&self) -> Result<states::OperationState> {
        get_data_zero_copy(&self.litesvm, &self.pda.clmm_operation_account())
    }

    fn clmm_query_amm_config(&self, index: u16) -> Result<states::AmmConfig> {
        get_data(&self.litesvm, &self.pda.clmm_amm_config(index))
    }

    fn clmm_query_pool_state(
        &self,
        amm_config: &Pubkey,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> Result<states::PoolState> {
        get_data_zero_copy(
            &self.litesvm,
            &self
                .pda
                .clmm_pool_state(*amm_config, *token_mint_0, *token_mint_1),
        )
    }
}

/// returns (token_mint_0, token_mint_1, sqrt_price_x64)
pub fn get_token_info_for_pool_creation(
    token_info_list: &[(Pubkey, u8, Decimal)], // (mint, decimals, price)
) -> (Pubkey, Pubkey, u128) {
    let (token_mint_0, token_mint_1) =
        &sort_token_mints(&token_info_list[0].0, &token_info_list[1].0);

    let (_, token_decimals_0, token_price_0) = *token_info_list
        .iter()
        .find(|x| &x.0 == token_mint_0)
        .unwrap();
    let (_, token_decimals_1, token_price_1) = *token_info_list
        .iter()
        .find(|x| &x.0 == token_mint_1)
        .unwrap();

    let sqrt_price_x64 = calculate_sqrt_price_x64(
        token_price_0,
        token_decimals_0,
        token_price_1,
        token_decimals_1,
    );

    (*token_mint_0, *token_mint_1, sqrt_price_x64)
}

fn sort_token_mints(mint_a: &Pubkey, mint_b: &Pubkey) -> (Pubkey, Pubkey) {
    if mint_a < mint_b {
        (*mint_a, *mint_b)
    } else {
        (*mint_b, *mint_a)
    }
}

/// Calculate sqrt_price_x64 for AMM pools using Decimal for precision
///
/// Formula: sqrt_price_x64 = sqrt(price_ratio) * 2^64
/// where price_ratio = (price_token1 / price_token0) * (10^decimals0 / 10^decimals1)
fn calculate_sqrt_price_x64(
    price_token0_usd: Decimal,
    decimals_token0: u8,
    price_token1_usd: Decimal,
    decimals_token1: u8,
) -> u128 {
    // Step 1: Calculate the price ratio (token1/token0)
    let price_ratio = price_token1_usd / price_token0_usd;

    // Step 2: Adjust for decimal differences
    let decimal_diff = decimals_token0 as i8 - decimals_token1 as i8;
    let decimal_adjustment = if decimal_diff >= 0 {
        u128_to_dec(10u128.pow(decimal_diff as u32))
    } else {
        Decimal::from_ratio(1, 10u128.pow((-decimal_diff) as u32))
    };

    let adjusted_price_ratio = price_ratio * decimal_adjustment;

    // Step 3: Take square root using integer square root
    // Since we don't have a built-in sqrt for Decimal, we'll use integer square root
    let sqrt_price = integer_sqrt(adjusted_price_ratio.atomics());

    // Step 4: Convert to Q64.64 format
    // We need to adjust because we took sqrt of the raw atomics
    // sqrt(atomics) = sqrt(actual_value * 10^18) = sqrt(actual_value) * sqrt(10^18)
    // So we need to divide by sqrt(10^18) = 10^9, then multiply by 2^64

    let sqrt_decimal_fractional = 1_000_000_000u128; // sqrt(10^18) = 10^9
    let q64_64_factor = 1u128 << 64; // 2^64

    (sqrt_price * q64_64_factor) / sqrt_decimal_fractional
}

/// Integer square root using Newton's method
fn integer_sqrt(value: u128) -> u128 {
    if value == 0 {
        return 0;
    }

    let mut x = value;
    let mut y = (x + 1) / 2;

    while y < x {
        x = y;
        y = (x + value / x) / 2;
    }

    x
}
