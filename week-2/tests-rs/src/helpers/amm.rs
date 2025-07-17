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

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<state::PoolConfig>;
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
        let signing_keypairs = [sender.keypair()];

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
            &signing_keypairs,
        )
    }

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<state::PoolConfig> {
        get_data(&self.litesvm, &self.pda.amm_pool_config(pool_id))
    }
}
