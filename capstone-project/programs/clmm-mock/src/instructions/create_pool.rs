use {
    crate::{
        state::{ObservationState, PoolState, TickArrayBitmapExtension},
        util::create_token_vault_account,
    },
    anchor_lang::{prelude::*, solana_program},
    anchor_spl::token_interface::{Mint, TokenInterface},
    raydium_clmm_cpi::states::{
        AmmConfig, OBSERVATION_SEED, POOL_SEED, POOL_TICK_ARRAY_BITMAP_SEED, POOL_VAULT_SEED,
    },
};

#[derive(Accounts)]
pub struct CreatePool<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub pool_creator: Signer<'info>,

    /// Which config the pool belongs to.
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// Initialize an account to store the pool state
    #[account(
        init,
        seeds = [
            POOL_SEED.as_bytes(),
            amm_config.key().as_ref(),
            token_mint_0.key().as_ref(),
            token_mint_1.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        space = PoolState::LEN
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Token_0 mint, the key must be smaller then token_1 mint.
    #[account(
        constraint = token_mint_0.key() < token_mint_1.key(),
        mint::token_program = token_program_0
    )]
    pub token_mint_0: Box<InterfaceAccount<'info, Mint>>,

    /// Token_1 mint
    #[account(
        mint::token_program = token_program_1
    )]
    pub token_mint_1: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Token_0 vault for the pool, initialized in contract
    #[account(
        mut,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_mint_0.key().as_ref(),
        ],
        bump,
    )]
    pub token_vault_0: UncheckedAccount<'info>,

    /// CHECK: Token_1 vault for the pool, initialized in contract
    #[account(
        mut,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_mint_1.key().as_ref(),
        ],
        bump,
    )]
    pub token_vault_1: UncheckedAccount<'info>,

    /// Initialize an account to store oracle observations
    #[account(
        init,
        seeds = [
            OBSERVATION_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        space = ObservationState::LEN
    )]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// Initialize an account to store if a tick array is initialized.
    #[account(
        init,
        seeds = [
            POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        space = TickArrayBitmapExtension::LEN
    )]
    pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,

    /// Spl token program or token program 2022
    pub token_program_0: Interface<'info, TokenInterface>,
    /// Spl token program or token program 2022
    pub token_program_1: Interface<'info, TokenInterface>,
    /// To create a new program account
    pub system_program: Program<'info, System>,
    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_pool(ctx: Context<CreatePool>, sqrt_price_x64: u128, open_time: u64) -> Result<()> {
    let block_timestamp = solana_program::clock::Clock::get()?.unix_timestamp as u64;
    require_gt!(block_timestamp, open_time);
    let pool_id = ctx.accounts.pool_state.key();
    let mut pool_state = ctx.accounts.pool_state.load_init()?;

    // init token vault accounts
    create_token_vault_account(
        &ctx.accounts.pool_creator,
        &ctx.accounts.pool_state.to_account_info(),
        &ctx.accounts.token_vault_0,
        &ctx.accounts.token_mint_0,
        &ctx.accounts.system_program,
        &ctx.accounts.token_program_0,
        &[
            POOL_VAULT_SEED.as_bytes(),
            ctx.accounts.pool_state.key().as_ref(),
            ctx.accounts.token_mint_0.key().as_ref(),
            &[ctx.bumps.token_vault_0][..],
        ],
    )?;

    create_token_vault_account(
        &ctx.accounts.pool_creator,
        &ctx.accounts.pool_state.to_account_info(),
        &ctx.accounts.token_vault_1,
        &ctx.accounts.token_mint_1,
        &ctx.accounts.system_program,
        &ctx.accounts.token_program_1,
        &[
            POOL_VAULT_SEED.as_bytes(),
            ctx.accounts.pool_state.key().as_ref(),
            ctx.accounts.token_mint_1.key().as_ref(),
            &[ctx.bumps.token_vault_1][..],
        ],
    )?;

    // init observation
    ctx.accounts
        .observation_state
        .load_init()?
        .initialize(pool_id)?;

    let bump = ctx.bumps.pool_state;
    pool_state.initialize(
        bump,
        sqrt_price_x64,
        0,
        0,
        ctx.accounts.pool_creator.key(),
        ctx.accounts.token_vault_0.key(),
        ctx.accounts.token_vault_1.key(),
        ctx.accounts.amm_config.as_ref(),
        ctx.accounts.token_mint_0.as_ref(),
        ctx.accounts.token_mint_1.as_ref(),
        ctx.accounts.observation_state.key(),
    )?;

    ctx.accounts
        .tick_array_bitmap
        .load_init()?
        .initialize(pool_id);

    Ok(())
}
