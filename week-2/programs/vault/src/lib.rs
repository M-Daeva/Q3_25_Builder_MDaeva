#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("Fy8V7mvvD4W3sAcCK725XxTi2XKZ4oDWBxaZV1hvDWom");

fn get_space(struct_space: usize) -> usize {
    const DISCRIMINATOR_SPACE: usize = 8;

    DISCRIMINATOR_SPACE + struct_space
}

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        init,
        space = get_space(VaultState::INIT_SPACE),
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let Initialize {
            user,
            system_program,
            vault_state,
            vault,
        } = self;

        vault_state.set_inner(VaultState {
            vault_bump: bumps.vault,
            state_bump: bumps.vault_state,
        });

        let rent_exempt = Rent::get()?.minimum_balance(vault.to_account_info().data_len());
        let cpi_program = system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: user.to_account_info(),
            to: vault.to_account_info(),
        };

        transfer(CpiContext::new(cpi_program, cpi_accounts), rent_exempt)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let Payment {
            user,
            system_program,
            vault,
            ..
        } = self;

        if amount == 0 {
            Err(ProgError::ZeroAmount)?;
        }

        let cpi_program = system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: user.to_account_info(),
            to: vault.to_account_info(),
        };

        transfer(CpiContext::new(cpi_program, cpi_accounts), amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let Payment {
            user,
            system_program,
            vault_state,
            vault,
        } = self;

        if amount == 0 {
            Err(ProgError::ZeroAmount)?;
        }

        let min_balance = Rent::get()?.minimum_balance(vault.to_account_info().data_len());
        let current_balance = vault.to_account_info().lamports();

        if current_balance < amount + min_balance {
            Err(ProgError::InsufficientFunds)?;
        }

        let cpi_program = system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: user.to_account_info(),
        };

        let seeds: &[&[u8]] = &[
            b"vault",
            vault_state.to_account_info().key.as_ref(),
            &[vault_state.vault_bump],
        ];

        transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, &[seeds]),
            amount,
        )
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let Close {
            user,
            system_program,
            vault_state,
            vault,
        } = self;

        let vault_balance = vault.to_account_info().lamports();

        if vault_balance == 0 {
            return Ok(());
        }

        let cpi_program = system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: user.to_account_info(),
        };

        let seeds: &[&[u8]] = &[
            b"vault",
            vault_state.to_account_info().key.as_ref(),
            &[vault_state.vault_bump],
        ];

        transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, &[seeds]),
            vault_balance,
        )
    }
}

#[error_code]
pub enum ProgError {
    #[msg("Zero amount of tokens can't be added or withdrawn")]
    ZeroAmount,

    #[msg("Insufficient funds in vault for withdrawal")]
    InsufficientFunds,
}
