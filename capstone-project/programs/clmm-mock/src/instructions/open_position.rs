use {
    crate::{
        error::ErrorCode,
        state::{PersonalPositionState, PoolState},
        util::transfer_from_user_to_pool_vault,
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken, metadata::Metadata, token::Token, token_2022::Token2022,
        token_interface,
    },
    std::cell::RefMut,
};

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
