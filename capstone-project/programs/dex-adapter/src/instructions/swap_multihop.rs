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
    pub input_token_mint: InterfaceAccount<'info, Mint>,
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

        // Load route from PDA
        let route_items = &self.route.value;
        if route_items.len() < 2 {
            Err(CustomError::InvalidRouteLength)?;
        }

        // Build route config indexes
        let mut route_config_indexes: Vec<u16> = vec![route_items[0].amm_index]; // First token's config
        for item in route_items.iter().skip(1) {
            route_config_indexes.push(item.amm_index);
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

        // execute multihop swap on clmm_mock
        self.execute_clmm_swap(
            remaining_accounts,
            amount_in,
            amount_out_minimum,
            route_config_indexes,
        )?;

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

        Ok(())
    }

    fn execute_clmm_swap(
        &self,
        remaining_accounts: &'info [AccountInfo<'info>],
        amount_in: u64,
        amount_out_minimum: u64,
        route_config_indexes: Vec<u16>,
    ) -> Result<()> {
        let accounts_per_hop = 7;
        let expected_remaining_accounts = (route_config_indexes.len() - 1) * accounts_per_hop;

        if remaining_accounts.len() != expected_remaining_accounts {
            Err(CustomError::InvalidRemainingAccounts)?;
        }

        // TODO
        msg!("Route config indexes: {:?}", route_config_indexes);
        msg!(
            "Expected remaining accounts: {}, actual: {}",
            expected_remaining_accounts,
            remaining_accounts.len()
        );

        // build accounts for CPI call to clmm_mock
        let mut accounts = vec![
            AccountMeta::new(self.config.key(), false), // payer (config as authority)
            AccountMeta::new(self.input_token_app_ata.key(), false), // input_token_account
            AccountMeta::new_readonly(self.input_token_mint.key(), false), // input_token_mint
            AccountMeta::new_readonly(self.token_program.key(), false), // token_program
            AccountMeta::new_readonly(self.token_program_2022.key(), false), // token_program_2022
            AccountMeta::new_readonly(self.memo_program.key(), false), // memo_program
        ];

        accounts.extend(remaining_accounts.iter().map(|acc| {
            if acc.is_writable {
                AccountMeta::new(acc.key(), acc.is_signer)
            } else {
                AccountMeta::new_readonly(acc.key(), acc.is_signer)
            }
        }));

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

        // TODO
        for (i, account) in account_infos.iter().enumerate() {
            msg!(
                "Account {}: {}, writable: {}, signer: {}",
                i,
                account.key(),
                account.is_writable,
                account.is_signer
            );
        }

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
