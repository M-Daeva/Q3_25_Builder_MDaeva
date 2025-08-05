use {
    crate::state::{PoolConfig, SEED_POOL_CONFIG},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::{get_space, transfer_token_from_user},
};

pub fn sort_token_mints(mint_a: &Pubkey, mint_b: &Pubkey) -> (Pubkey, Pubkey) {
    if mint_a < mint_b {
        (*mint_a, *mint_b)
    } else {
        (*mint_b, *mint_a)
    }
}

#[derive(Accounts)]
pub struct CreatePoolNew<'info> {
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
        space = get_space(PoolConfig::INIT_SPACE),
        // mints must be sorted
        seeds = [SEED_POOL_CONFIG.as_bytes(), mint_a.key().to_bytes().as_ref(), mint_b.key().to_bytes().as_ref()],
        bump
    )]
    pub pool_config: Account<'info, PoolConfig>,

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
        associated_token::authority = pool_config,
    )]
    pub app_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_b,
        associated_token::authority = pool_config,
    )]
    pub app_b_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreatePoolNew<'info> {
    pub fn create_pool_new(
        &mut self,
        bumps: CreatePoolNewBumps,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        let CreatePoolNew {
            token_program,
            sender,
            pool_config,
            mint_a,
            mint_b,
            sender_a_ata,
            sender_b_ata,
            app_a_ata,
            app_b_ata,
            ..
        } = self;

        pool_config.set_inner(PoolConfig {
            bump: bumps.pool_config,
            mint_a: mint_a.key(),
            mint_b: mint_b.key(),
            amount_a,
            amount_b,
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
