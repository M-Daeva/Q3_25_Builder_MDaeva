use {
    crate::{
        error::CustomError,
        state::{Bump, Config, Route, SEED_BUMP, SEED_CONFIG, SEED_ROUTE},
        types::SwapRouterBaseInData,
    },
    anchor_lang::{prelude::*, solana_program},
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::{transfer_token_from_program, transfer_token_from_user},
};

#[derive(Accounts)]
pub struct SwapMultihop<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: token_program_2022
    pub token_program_2022: UncheckedAccount<'info>,
    /// CHECK: memo_program
    pub memo_program: UncheckedAccount<'info>,
    /// CHECK: clmm_mock_program
    pub clmm_mock_program: UncheckedAccount<'info>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        mut,
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [SEED_ROUTE.as_bytes(), &input_token_mint.key().to_bytes(), &output_token_mint.key().to_bytes()],
        bump
    )]
    pub route: Account<'info, Route>,

    // mint
    //
    #[account(mut)]
    pub input_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub output_token_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        mut,
        associated_token::mint = input_token_mint,
        associated_token::authority = sender
    )]
    pub input_token_sender_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = output_token_mint,
        associated_token::authority = sender
    )]
    pub output_token_sender_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = input_token_mint,
        associated_token::authority = config
    )]
    pub input_token_app_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = output_token_mint,
        associated_token::authority = config
    )]
    pub output_token_app_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> SwapMultihop<'info> {
    pub fn swap_multihop(
        &mut self,
        remaining_accounts: &'info [AccountInfo<'info>],
        amount_in: u64,
        amount_out_minimum: u64,
    ) -> Result<()> {
        if amount_in == 0 {
            Err(CustomError::InvalidAmount)?;
        }

        // transfer input tokens from sender to app ATA
        transfer_token_from_user(
            amount_in,
            &self.input_token_mint,
            &self.input_token_sender_ata,
            &self.input_token_app_ata,
            &self.sender,
            &self.token_program,
        )?;
        msg!("✅ completed transfer_token_from_user");

        // execute multihop swap on clmm_mock
        self.execute_clmm_swap(remaining_accounts, amount_in, amount_out_minimum)?;
        msg!("✅ completed execute_clmm_swap");

        // transfer output tokens from app ATA to sender
        let output_balance = self.output_token_app_ata.amount;
        if output_balance == 0 {
            Err(CustomError::NoOutputTokens)?;
        }

        transfer_token_from_program(
            output_balance,
            &self.output_token_mint,
            &self.output_token_app_ata,
            &self.output_token_sender_ata,
            &[SEED_CONFIG.as_bytes()],
            self.bump.config,
            &self.config,
            &self.token_program,
        )?;
        msg!("✅ completed transfer_token_from_program");

        Ok(())
    }

    fn execute_clmm_swap(
        &self,
        remaining_accounts: &'info [AccountInfo<'info>],
        amount_in: u64,
        amount_out_minimum: u64,
    ) -> Result<()> {
        // Validate that remaining accounts length is correct (multiple of 7)
        if remaining_accounts.len() % 7 != 0 {
            Err(CustomError::InvalidRemainingAccounts)?;
        }

        // build accounts for CPI call to clmm_mock - match exact structure from clmm-mock
        let mut accounts = vec![
            AccountMeta::new(self.config.key(), true), // payer (signer)
            AccountMeta::new(self.input_token_app_ata.key(), false), // input_token_account (writable)
            AccountMeta::new(self.input_token_mint.key(), false), // input_token_mint (writable) ← FIXED
            AccountMeta::new_readonly(self.token_program.key(), false), // token_program
            AccountMeta::new_readonly(self.token_program_2022.key(), false), // token_program_2022
            AccountMeta::new_readonly(self.memo_program.key(), false), // memo_program
        ];

        // Process remaining accounts in groups of 7
        for chunk in remaining_accounts.chunks_exact(7) {
            accounts.extend(vec![
                AccountMeta::new_readonly(chunk[0].key(), false), // amm_config (readonly)
                AccountMeta::new(chunk[1].key(), false),          // pool_state (writable)
                AccountMeta::new(chunk[2].key(), false),          // output_token_account (writable)
                AccountMeta::new(chunk[3].key(), false),          // input_vault (writable)
                AccountMeta::new(chunk[4].key(), false),          // output_vault (writable)
                AccountMeta::new_readonly(chunk[5].key(), false), // output_mint (readonly)
                AccountMeta::new(chunk[6].key(), false),          // observation_state (writable)
            ]);
        }

        // prepare instruction data
        let instruction_data = SwapRouterBaseInData {
            discriminator: get_discriminator("swap_router_base_in"),
            amount_in,
            amount_out_minimum,
        };

        let instruction = solana_program::instruction::Instruction {
            program_id: self.config.dex,
            accounts,
            data: instruction_data.try_to_vec()?,
        };
        msg!("✅ completed instruction_data");

        // create signer seeds for config PDA
        let config_seeds = &[SEED_CONFIG.as_bytes(), &[self.bump.config]];
        let signer_seeds = &[&config_seeds[..]];

        let account_infos = [
            &[
                self.config.to_account_info(),
                self.input_token_app_ata.to_account_info(),
                self.input_token_mint.to_account_info(),
                self.token_program.to_account_info(),
                self.token_program_2022.to_account_info(),
                self.memo_program.to_account_info(),
            ],
            remaining_accounts,
        ]
        .concat();

        msg!("✅ input_token_mint: {:#?}", &self.input_token_mint.key());

        // execute CPI call with config as signer
        anchor_lang::solana_program::program::invoke_signed(
            &instruction,
            &account_infos,
            signer_seeds,
        )?;

        Ok(())
    }
}

fn get_discriminator(instruction_name: &str) -> [u8; 8] {
    let mut discriminator = [0u8; 8];
    let hash = solana_program::hash::hash(format!("global:{}", instruction_name).as_bytes());
    discriminator.copy_from_slice(&hash.to_bytes()[..8]);
    discriminator
}
