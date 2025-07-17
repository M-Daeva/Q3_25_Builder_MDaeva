use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx_with_ix},
            token::WithTokenKeys,
            App, ProgramId,
        },
        types::{AppToken, AppUser},
    },
    amm::{accounts, instruction, state},
    anchor_lang::Result,
    litesvm::types::TransactionMetadata,
};

pub trait AmmExtension {
    fn amm_try_create_pool(
        &mut self,
        sender: AppUser,
        id: u64,
        mint_x: AppToken,
        mint_y: AppToken,
        fee_bps: u16,
    ) -> Result<TransactionMetadata>;

    fn amm_try_provide_liquidity(
        &mut self,
        sender: AppUser,
        id: u64,
        mint_x_amount: u64,
        mint_y_amount: u64,
    ) -> Result<TransactionMetadata>;

    fn amm_try_withdraw_liquidity(
        &mut self,
        sender: AppUser,
        id: u64,
        mint_lp_amount: u64,
    ) -> Result<TransactionMetadata>;

    fn amm_try_swap(
        &mut self,
        sender: AppUser,
        id: u64,
        amount_in: u64,
        mint_in: AppToken,
    ) -> Result<TransactionMetadata>;

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<state::PoolConfig>;

    fn amm_query_pool_config_list(&self) -> Result<Vec<state::PoolConfig>>;

    fn amm_query_pool_balance(&self, pool_id: u64) -> Result<state::PoolBalance>;
}

impl AmmExtension for App {
    fn amm_try_create_pool(
        &mut self,
        sender: AppUser,
        id: u64,
        mint_x: AppToken,
        mint_y: AppToken,
        fee_bps: u16,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            amm: program_id,
            system_program,
            token_program,
            associated_token_program,
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let mint_x = mint_x.pubkey(&self);
        let mint_y = mint_y.pubkey(&self);

        // pda
        let pool_config = self.pda.amm_pool_config(id);
        let pool_balance = self.pda.amm_pool_balance(id);
        let mint_lp = self.pda.amm_mint_lp(id);

        // ata
        let liquidity_pool_mint_lp_ata = App::get_ata(&pool_config, &mint_lp);
        let liquidity_pool_mint_x_ata = App::get_ata(&pool_config, &mint_x);
        let liquidity_pool_mint_y_ata = App::get_ata(&pool_config, &mint_y);

        let accounts = accounts::CreatePool {
            system_program,
            token_program,
            associated_token_program,
            pool_creator: payer,
            pool_config,
            pool_balance,
            mint_lp,
            mint_x,
            mint_y,
            liquidity_pool_mint_lp_ata,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
        };

        let instruction_data = instruction::CreatePool {
            id,
            mint_x,
            mint_y,
            fee_bps,
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

    fn amm_try_provide_liquidity(
        &mut self,
        sender: AppUser,
        id: u64,
        mint_x_amount: u64,
        mint_y_amount: u64,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            amm: program_id,
            system_program,
            token_program,
            associated_token_program,
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let state::PoolConfig { mint_x, mint_y, .. } = self.amm_query_pool_config(id)?;

        // pda
        let pool_config = self.pda.amm_pool_config(id);
        let pool_balance = self.pda.amm_pool_balance(id);
        let mint_lp = self.pda.amm_mint_lp(id);

        // ata
        let liquidity_pool_mint_lp_ata = App::get_ata(&pool_config, &mint_lp);
        let liquidity_pool_mint_x_ata = App::get_ata(&pool_config, &mint_x);
        let liquidity_pool_mint_y_ata = App::get_ata(&pool_config, &mint_y);

        let liquidity_provider_mint_lp_ata = App::get_ata(&payer, &mint_lp);
        let liquidity_provider_mint_x_ata = App::get_ata(&payer, &mint_x);
        let liquidity_provider_mint_y_ata = App::get_ata(&payer, &mint_y);

        let accounts = accounts::Liquidity {
            system_program,
            token_program,
            associated_token_program,
            liquidity_provider: payer,
            pool_config,
            pool_balance,
            mint_lp,
            mint_x,
            mint_y,
            liquidity_pool_mint_lp_ata,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
            liquidity_provider_mint_lp_ata,
            liquidity_provider_mint_x_ata,
            liquidity_provider_mint_y_ata,
        };

        let instruction_data = instruction::ProvideLiquidity {
            _id: id,
            mint_x_amount,
            mint_y_amount,
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

    fn amm_try_withdraw_liquidity(
        &mut self,
        sender: AppUser,
        id: u64,
        mint_lp_amount: u64,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            amm: program_id,
            system_program,
            token_program,
            associated_token_program,
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let state::PoolConfig { mint_x, mint_y, .. } = self.amm_query_pool_config(id)?;

        // pda
        let pool_config = self.pda.amm_pool_config(id);
        let pool_balance = self.pda.amm_pool_balance(id);
        let mint_lp = self.pda.amm_mint_lp(id);

        // ata
        let liquidity_pool_mint_lp_ata = App::get_ata(&pool_config, &mint_lp);
        let liquidity_pool_mint_x_ata = App::get_ata(&pool_config, &mint_x);
        let liquidity_pool_mint_y_ata = App::get_ata(&pool_config, &mint_y);

        let liquidity_provider_mint_lp_ata = App::get_ata(&payer, &mint_lp);
        let liquidity_provider_mint_x_ata = App::get_ata(&payer, &mint_x);
        let liquidity_provider_mint_y_ata = App::get_ata(&payer, &mint_y);

        let accounts = accounts::Liquidity {
            system_program,
            token_program,
            associated_token_program,
            liquidity_provider: payer,
            pool_config,
            pool_balance,
            mint_lp,
            mint_x,
            mint_y,
            liquidity_pool_mint_lp_ata,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
            liquidity_provider_mint_lp_ata,
            liquidity_provider_mint_x_ata,
            liquidity_provider_mint_y_ata,
        };

        let instruction_data = instruction::WithdrawLiquidity {
            _id: id,
            mint_lp_amount,
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

    fn amm_try_swap(
        &mut self,
        sender: AppUser,
        id: u64,
        amount_in: u64,
        mint_in: AppToken,
    ) -> Result<TransactionMetadata> {
        // programs
        let ProgramId {
            amm: program_id,
            system_program,
            token_program,
            associated_token_program,
        } = self.program_id;

        // signers
        let payer = sender.pubkey();
        let signers = [sender.keypair()];

        // mint
        let mint_in = mint_in.pubkey(&self);
        let state::PoolConfig { mint_x, mint_y, .. } = self.amm_query_pool_config(id)?;

        // pda
        let pool_config = self.pda.amm_pool_config(id);
        let pool_balance = self.pda.amm_pool_balance(id);

        // ata
        let liquidity_pool_mint_x_ata = App::get_ata(&pool_config, &mint_x);
        let liquidity_pool_mint_y_ata = App::get_ata(&pool_config, &mint_y);

        let trader_mint_x_ata = App::get_ata(&payer, &mint_x);
        let trader_mint_y_ata = App::get_ata(&payer, &mint_y);

        let accounts = accounts::Swap {
            system_program,
            token_program,
            associated_token_program,
            trader: payer,
            pool_config,
            pool_balance,
            mint_x,
            mint_y,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
            trader_mint_x_ata,
            trader_mint_y_ata,
        };

        let instruction_data = instruction::Swap {
            _id: id,
            amount_in,
            mint_in,
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

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<state::PoolConfig> {
        get_data(&self.litesvm, &self.pda.amm_pool_config(pool_id))
    }

    fn amm_query_pool_config_list(&self) -> Result<Vec<state::PoolConfig>> {
        unimplemented!()
    }

    fn amm_query_pool_balance(&self, pool_id: u64) -> Result<state::PoolBalance> {
        get_data(&self.litesvm, &self.pda.amm_pool_balance(pool_id))
    }
}
