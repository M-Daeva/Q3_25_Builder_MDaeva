use {
    crate::{
        error::ErrorCode,
        state::{PersonalPositionState, PoolState},
        util::transfer_from_user_to_pool_vault,
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::Metadata,
        token::Token,
        token_interface::{self, Mint, Token2022, TokenAccount},
    },
    raydium_clmm_cpi::states::{POSITION_SEED, TICK_ARRAY_SEED},
    std::cell::RefMut,
};

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPositionWithToken22Nft<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: UncheckedAccount<'info>,

    /// Unique token mint address, initialize in contract
    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// CHECK: ATA address where position NFT will be minted, initialize in contract
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK:  Account to store data for the position's lower tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_lower_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK: Account to store data for the position's upper tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_upper_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_upper: UncheckedAccount<'info>,

    /// personal position state
    #[account(
        init,
        seeds = [POSITION_SEED.as_bytes(), position_nft_mint.key().as_ref()],
        bump,
        payer = payer,
        space = PersonalPositionState::LEN
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// The token_0 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The token_1 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the position manager state account
    pub system_program: Program<'info, System>,

    /// Program to transfer for token account
    pub token_program: Program<'info, Token>,

    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program_2022: Program<'info, Token2022>,

    /// The mint of token vault 0
    #[account(
        address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,
}

pub fn open_position_with_token22_nft<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, OpenPositionWithToken22Nft<'info>>,
    liquidity: u128,
    amount_0_max: u64,
    amount_1_max: u64,
    tick_lower_index: i32,
    tick_upper_index: i32,
    tick_array_lower_start_index: i32,
    tick_array_upper_start_index: i32,
    with_metadata: bool,
    base_flag: Option<bool>,
) -> Result<()> {
    open_position(
        &ctx.accounts.payer,
        &ctx.accounts.position_nft_owner,
        &ctx.accounts.position_nft_mint,
        &ctx.accounts.position_nft_account,
        None,
        &ctx.accounts.pool_state,
        &ctx.accounts.tick_array_lower,
        &ctx.accounts.tick_array_upper,
        &mut ctx.accounts.personal_position,
        &ctx.accounts.token_account_0.to_account_info(),
        &ctx.accounts.token_account_1.to_account_info(),
        &ctx.accounts.token_vault_0.to_account_info(),
        &ctx.accounts.token_vault_1.to_account_info(),
        &ctx.accounts.rent,
        &ctx.accounts.system_program,
        &ctx.accounts.token_program,
        &ctx.accounts.associated_token_program,
        None,
        Some(&ctx.accounts.token_program_2022),
        Some(ctx.accounts.vault_0_mint.clone()),
        Some(ctx.accounts.vault_1_mint.clone()),
        &ctx.remaining_accounts,
        ctx.bumps.personal_position,
        liquidity,
        amount_0_max,
        amount_1_max,
        tick_lower_index,
        tick_upper_index,
        tick_array_lower_start_index,
        tick_array_upper_start_index,
        with_metadata,
        base_flag,
        true,
    )
}

pub fn open_position<'a, 'b, 'c: 'info, 'info>(
    payer: &'b Signer<'info>,
    _position_nft_owner: &'b UncheckedAccount<'info>,
    position_nft_mint: &'b AccountInfo<'info>,
    _position_nft_account: &'b AccountInfo<'info>,
    _metadata_account: Option<&'b UncheckedAccount<'info>>,
    pool_state_loader: &'b AccountLoader<'info, PoolState>,
    _tick_array_lower_loader: &'b UncheckedAccount<'info>,
    _tick_array_upper_loader: &'b UncheckedAccount<'info>,
    personal_position: &'b mut Box<Account<'info, PersonalPositionState>>,
    token_account_0: &'b AccountInfo<'info>,
    token_account_1: &'b AccountInfo<'info>,
    token_vault_0: &'b AccountInfo<'info>,
    token_vault_1: &'b AccountInfo<'info>,
    _rent: &'b Sysvar<'info, Rent>,
    _system_program: &'b Program<'info, System>,
    token_program: &'b Program<'info, Token>,
    _associated_token_program: &'b Program<'info, AssociatedToken>,
    _metadata_program: Option<&'b Program<'info, Metadata>>,
    token_program_2022: Option<&'b Program<'info, Token2022>>,
    vault_0_mint: Option<Box<InterfaceAccount<'info, token_interface::Mint>>>,
    vault_1_mint: Option<Box<InterfaceAccount<'info, token_interface::Mint>>>,

    _remaining_accounts: &'c [AccountInfo<'info>],
    personal_position_bump: u8,
    liquidity: u128,
    amount_0_max: u64,
    amount_1_max: u64,
    tick_lower_index: i32,
    tick_upper_index: i32,
    _tick_array_lower_start_index: i32,
    _tick_array_upper_start_index: i32,
    _with_metadata: bool,
    base_flag: Option<bool>,
    _use_metadata_extension: bool,
) -> Result<()> {
    let mut liquidity = liquidity;
    let pool_state = &mut pool_state_loader.load_mut()?;

    let LiquidityChangeResult {
        fee_growth_inside_0_x64,
        fee_growth_inside_1_x64,
        reward_growths_inside,
        ..
    } = add_liquidity(
        payer,
        token_account_0,
        token_account_1,
        token_vault_0,
        token_vault_1,
        // &tick_array_lower_loader,
        // &tick_array_upper_loader,
        token_program_2022,
        token_program,
        vault_0_mint,
        vault_1_mint,
        None,
        pool_state,
        &mut liquidity,
        amount_0_max,
        amount_1_max,
        tick_lower_index,
        tick_upper_index,
        base_flag,
    )?;

    personal_position.initialize(
        personal_position_bump,
        position_nft_mint.key(),
        pool_state_loader.key(),
        tick_lower_index,
        tick_upper_index,
        liquidity,
        fee_growth_inside_0_x64,
        fee_growth_inside_1_x64,
        reward_growths_inside,
        0,
    )?;

    Ok(())
}

#[derive(Default)]
pub struct LiquidityChangeResult {
    pub amount_0: u64,
    pub amount_1: u64,
    pub amount_0_transfer_fee: u64,
    pub amount_1_transfer_fee: u64,
    pub tick_lower_flipped: bool,
    pub tick_upper_flipped: bool,
    pub fee_growth_inside_0_x64: u128,
    pub fee_growth_inside_1_x64: u128,
    pub reward_growths_inside: [u128; 3],
}

/// Add liquidity to an initialized pool
pub fn add_liquidity<'b, 'c: 'info, 'info>(
    payer: &'b Signer<'info>,
    token_account_0: &'b AccountInfo<'info>,
    token_account_1: &'b AccountInfo<'info>,
    token_vault_0: &'b AccountInfo<'info>,
    token_vault_1: &'b AccountInfo<'info>,
    // tick_array_lower_loader: &'b AccountLoad<'info, TickArrayState>,
    // tick_array_upper_loader: &'b AccountLoad<'info, TickArrayState>,
    token_program_2022: Option<&Program<'info, Token2022>>,
    token_program: &'b Program<'info, Token>,
    vault_0_mint: Option<Box<InterfaceAccount<'info, token_interface::Mint>>>,
    vault_1_mint: Option<Box<InterfaceAccount<'info, token_interface::Mint>>>,
    _tick_array_bitmap_extension: Option<&'c AccountInfo<'info>>,
    _pool_state: &mut RefMut<PoolState>,
    liquidity: &mut u128,
    amount_0_max: u64,
    amount_1_max: u64,
    _tick_lower_index: i32,
    _tick_upper_index: i32,
    base_flag: Option<bool>,
) -> Result<LiquidityChangeResult> {
    if *liquidity == 0 {
        if base_flag.is_none() {
            // when establishing a new position , liquidity allows for further additions
            return Ok(LiquidityChangeResult::default());
        }
    }
    assert!(*liquidity > 0);

    let result = LiquidityChangeResult::default();

    let amount_0 = result.amount_0;
    let amount_1 = result.amount_1;

    let amount_0_transfer_fee = 0;
    let amount_1_transfer_fee = 0;

    require_gte!(
        amount_0_max,
        amount_0 + amount_0_transfer_fee,
        ErrorCode::PriceSlippageCheck
    );
    require_gte!(
        amount_1_max,
        amount_1 + amount_1_transfer_fee,
        ErrorCode::PriceSlippageCheck
    );
    let mut token_2022_program_opt: Option<AccountInfo> = None;
    if token_program_2022.is_some() {
        token_2022_program_opt = Some(token_program_2022.clone().unwrap().to_account_info());
    }

    transfer_from_user_to_pool_vault(
        payer,
        token_account_0,
        token_vault_0,
        vault_0_mint,
        &token_program,
        token_2022_program_opt.clone(),
        amount_0_max,
    )?;
    transfer_from_user_to_pool_vault(
        payer,
        token_account_1,
        token_vault_1,
        vault_1_mint,
        &token_program,
        token_2022_program_opt.clone(),
        amount_1_max,
    )?;

    Ok(result)
}
