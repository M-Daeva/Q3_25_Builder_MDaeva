use {
    crate::state::{PoolState, SEED_POOL_STATE},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::{get_space, transfer_token_from_user},
};

#[derive(Accounts)]
#[instruction(id: u8)]
pub struct CreatePool<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        init,
        payer = sender,
        space = get_space(PoolState::INIT_SPACE),
        seeds = [SEED_POOL_STATE.as_bytes(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool_state: Account<'info, PoolState>,

    // mint
    //
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = sender,
    )]
    pub sender_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = sender,
    )]
    pub sender_b_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_a,
        associated_token::authority = pool_state,
    )]
    pub app_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_b,
        associated_token::authority = pool_state,
    )]
    pub app_b_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreatePool<'info> {
    pub fn create_pool(&mut self, id: u8, amount_a: u64, amount_b: u64) -> Result<()> {
        let CreatePool {
            token_program,
            sender,
            pool_state,
            mint_a,
            mint_b,
            sender_a_ata,
            sender_b_ata,
            app_a_ata,
            app_b_ata,
            ..
        } = self;

        // price_ratio = amount_b / amount_a (scaled by 1e9 for precision)
        let price_ratio = if amount_a > 0 {
            (amount_b as u128 * 1_000_000_000) / amount_a as u128
        } else {
            0
        };

        pool_state.set_inner(PoolState {
            id,
            mint_a: mint_a.key(),
            mint_b: mint_b.key(),
            price_ratio,
        });

        for (amount, mint, from, to) in [
            (amount_a, mint_a, sender_a_ata, app_a_ata),
            (amount_b, mint_b, sender_b_ata, app_b_ata),
        ] {
            transfer_token_from_user(amount, mint, from, to, sender, token_program)?;
        }

        Ok(())
    }
}
