use {
    crate::{
        error::DiceError,
        state::{Bet, HOUSE_EDGE},
    },
    anchor_instruction_sysvar::Ed25519InstructionSignatures,
    anchor_lang::{
        prelude::*,
        solana_program::{
            self, ed25519_program,
            hash::hash,
            sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
        },
    },
    base::helpers::transfer_sol_from_program,
};

#[derive(Accounts)]
#[instruction(id: u128)]
pub struct ResolveBet<'info> {
    pub system_program: Program<'info, System>,

    /// CHECK: this is safe
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    pub instruction_sysvar: AccountInfo<'info>,

    #[account(mut)]
    pub house: Signer<'info>,

    /// CHECK: this is safe
    #[account(mut)]
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        close = player,
        has_one = player,
        seeds = [b"bet", vault.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ResolveBet {
            instruction_sysvar,
            house,
            bet,
            ..
        } = self;

        // get the current instruction index first
        let current_index = load_current_index_checked(&instruction_sysvar.to_account_info())?;

        // Ed25519 instruction should be before this instruction
        let ed25519_index = current_index
            .checked_sub(1)
            .ok_or(DiceError::Ed25519Program)? as usize;

        let ix = load_instruction_at_checked(ed25519_index, &instruction_sysvar.to_account_info())?;

        if ix.program_id != ed25519_program::ID {
            Err(DiceError::Ed25519Program)?;
        }

        if !ix.accounts.is_empty() {
            Err(DiceError::Ed25519Accounts)?;
        }

        let Ed25519InstructionSignatures(signatures) =
            Ed25519InstructionSignatures::unpack(&ix.data)?;

        if signatures.len() != 1 {
            Err(DiceError::Ed25519Length)?;
        }

        let signature = signatures.first().unwrap();

        if !signature.is_verifiable {
            Err(DiceError::Ed25519Header)?;
        }

        if signature.public_key != Some(house.key()) {
            Err(DiceError::Ed25519Pubkey)?;
        }

        if signature.signature.ok_or(DiceError::Ed25519Signature)? != sig
            || signature.message != Some(bet.to_slice())
        {
            Err(DiceError::Ed25519Signature)?;
        }

        Ok(())
    }

    pub fn resolve_bet(&mut self, bumps: &ResolveBetBumps, _id: u128, sig: &[u8]) -> Result<()> {
        let ResolveBet {
            system_program,
            house,
            player,
            bet,
            vault,
            ..
        } = self;

        let hash = hash(sig).to_bytes();
        let mut hash_16 = [0; 16];

        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if bet.roll > roll {
            let payout = (bet.amount as u128)
                .checked_mul(10_000 - HOUSE_EDGE as u128)
                .ok_or(DiceError::Overflow)?
                .checked_div(bet.roll as u128 - 1)
                .ok_or(DiceError::Overflow)?
                .checked_div(100)
                .ok_or(DiceError::Overflow)? as u64;

            transfer_sol_from_program(
                payout,
                vault,
                player,
                &[b"vault", house.key().as_ref()],
                bumps.vault,
                system_program,
            )?;
        }

        Ok(())
    }
}
