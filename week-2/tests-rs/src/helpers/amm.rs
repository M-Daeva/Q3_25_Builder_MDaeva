use anchor_lang::{
    error::{AnchorError, Error, ProgramErrorWithOrigin},
    prelude::ProgramError,
    system_program, AnchorDeserialize, Id, Result,
};
use anchor_spl::associated_token::AssociatedToken;
use litesvm::types::TransactionMetadata;
use solana_pubkey::Pubkey;

use amm::state::PoolConfig;

use crate::helpers::suite::{
    core::App,
    types::{AppToken, AppUser},
};

const DISCRIMINATOR_SPACE: usize = 8;

pub trait AmmExtension {
    fn amm_try_create_pool(&mut self) -> Result<TransactionMetadata>;

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<PoolConfig>;
}

impl AmmExtension for App {
    fn amm_try_create_pool(&mut self) -> Result<TransactionMetadata> {
        let program_id = self.program_id.amm;

        let pool_creator_keypair = AppUser::Admin.keypair();
        let mint_x_keypair = AppToken::USDC.keypair(&app);
        let mint_y_keypair = AppToken::PYTH.keypair(&app);

        let pool_creator = pool_creator_keypair.pubkey();
        let mint_x = mint_x_keypair.pubkey();
        let mint_y = mint_y_keypair.pubkey();

        // Pool parameters
        let id: u64 = 1; // pool_id
        let fee_bps: u16 = 300; // 3%

        // Derive PDAs
        let pool_config = app.pda.amm_pool_config(id);
        let pool_balance = app.pda.amm_pool_balance(id);
        let mint_lp = app.pda.amm_mint_lp(id);

        // Derive ATAs
        let liquidity_pool_mint_lp_ata = App::get_ata(&pool_config, &mint_lp);
        let liquidity_pool_mint_x_ata = App::get_ata(&pool_config, &mint_x);
        let liquidity_pool_mint_y_ata = App::get_ata(&pool_config, &mint_y);

        // Create instruction data
        let instruction_data = amm::instruction::CreatePool {
            id,
            mint_x,
            mint_y,
            fee_bps,
        };

        // Build accounts for the instruction
        let accounts = amm::accounts::CreatePool {
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: AssociatedToken::id(),
            pool_creator,
            pool_config,
            pool_balance,
            mint_lp,
            mint_x,
            mint_y,
            liquidity_pool_mint_lp_ata,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
        };

        // Create the instruction
        let ix = Instruction {
            program_id,
            accounts: accounts.to_account_metas(None),
            data: instruction_data.data(),
        };

        // Create and send transaction
        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&pool_creator),
            &[&pool_creator_keypair],
            app.litesvm.latest_blockhash(),
        );

        // Execute the transaction
        let res = app.litesvm.send_transaction(transaction).unwrap();

        unimplemented!()
    }

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<PoolConfig> {
        get_data(self, &self.pda.amm_pool_config(pool_id))
    }
}

fn get_data<T>(app: &App, pda: &Pubkey) -> Result<T>
where
    T: AnchorDeserialize,
{
    let data = &mut &app.litesvm.get_account(pda).unwrap().data[DISCRIMINATOR_SPACE..];

    Ok(T::deserialize(data)?)
}
