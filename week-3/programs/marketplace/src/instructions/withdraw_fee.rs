use {
    crate::state::{Balances, Marketplace},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::{transfer_sol_from_program, transfer_token_from_program},
};

#[derive(Accounts)]
pub struct WithdrawTokenFee<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub admin: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"balances", admin.key().as_ref()],
        bump = marketplace.bump.balances
    )]
    pub balances: Account<'info, Balances>,

    // mint
    //
    pub token_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = token_mint,
        associated_token::authority = sender
    )]
    pub sender_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = marketplace
    )]
    pub app_token_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> WithdrawTokenFee<'info> {
    pub fn withdraw_token_fee(&mut self) -> Result<()> {
        let WithdrawTokenFee {
            token_program,
            admin,
            marketplace,
            balances,
            token_mint,
            sender_token_ata,
            app_token_ata,
            ..
        } = self;

        let mut amount = 0;
        balances.value = balances
            .value
            .iter()
            .cloned()
            .map(|mut x| {
                if x.asset == token_mint.key() && x.amount != 0 {
                    amount = x.amount;
                    x.amount = 0;
                }

                x
            })
            .collect();

        if amount != 0 {
            transfer_token_from_program(
                amount,
                token_mint,
                app_token_ata,
                sender_token_ata,
                &[b"marketplace", admin.key().as_ref()],
                marketplace.bump.marketplace,
                marketplace,
                token_program,
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawSolFee<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub admin: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"treasury", admin.key().as_ref()],
        bump = marketplace.bump.treasury
    )]
    pub treasury: SystemAccount<'info>,

    // data storage
    //
    #[account(
        mut,
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"balances", admin.key().as_ref()],
        bump = marketplace.bump.balances
    )]
    pub balances: Account<'info, Balances>,
}

impl<'info> WithdrawSolFee<'info> {
    pub fn withdraw_sol_fee(&mut self) -> Result<()> {
        let WithdrawSolFee {
            system_program,
            sender,
            admin,
            treasury,
            marketplace,
            balances,
        } = self;

        let mut amount = 0;
        balances.value = balances
            .value
            .iter()
            .cloned()
            .map(|mut x| {
                if x.asset == Pubkey::default() && x.amount != 0 {
                    amount = x.amount;
                    x.amount = 0;
                }

                x
            })
            .collect();

        if amount != 0 {
            transfer_sol_from_program(
                amount,
                &treasury.to_account_info(),
                sender,
                &[b"treasury", admin.key().as_ref()],
                marketplace.bump.treasury,
                system_program,
            )?;
        }

        Ok(())
    }
}
