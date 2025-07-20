use {
    crate::{
        error::ProgError,
        helpers::{get_space, has_duplicates},
        math::get_updated_vault,
        state::{Config, Vault},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            mpl_token_metadata::instructions::{
                FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
            },
            MasterEditionAccount, Metadata, MetadataAccount,
        },
        token::{approve, Approve, Mint, Token, TokenAccount},
    },
};

#[derive(Accounts)]
pub struct Stake<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = get_space(Vault::INIT_SPACE),
        seeds = [b"user_vault", user.key().as_ref()],
        bump
    )]
    pub user_vault: Account<'info, Vault>,

    #[account(
        seeds = [b"config"],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,

    pub nft_mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,
    //
    // #[account(
    //     mut,
    //     associated_token::mint = nft_mint,
    //     associated_token::authority = user
    // )]
    // pub user_nft_ata: Account<'info, TokenAccount>,
    #[account(
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref()],
        seeds::program = metadata_program.key(), // TODO: do we need it?
        bump
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MetadataAccount>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, tokens: Vec<u16>) -> Result<()> {
        // let clock_time = Clock::get()?.unix_timestamp as u64;
        // let Stake {
        //     system_program,
        //     token_program,
        //     metadata_program,
        //     user,
        //     user_vault,
        //     config,
        //     nft_mint,
        //     collection_mint,
        //     user_nft_ata,
        //     metadata,
        //     edition,
        // } = self;

        // if tokens.is_empty() {
        //     Err(ProgError::EmptyTokenList)?;
        // }

        // user_vault.set_inner(get_updated_vault(
        //     &user_vault,
        //     config.rewards_rate,
        //     clock_time,
        // ));
        // user_vault.tokens.extend(tokens);

        // if has_duplicates(&user_vault.tokens) {
        //     Err(ProgError::NftDuplication)?;
        // }

        // if user_vault.rewards > config.max_stake {
        //     Err(ProgError::ExceededTokenLimit)?;
        // }

        // TODO: handle nft

        // approve
        //
        // let cpi_program = self.token_program.to_account_info();
        // let cpi_accounts = anchor_spl::token::Approve {
        //     to: config.to_account_info(), // The token account whose tokens are being delegated
        //     delegate: user_nft_ata.to_account_info(), // The account that will be given the authority to transfer tokens from the to account.
        //     authority: user.to_account_info(), // The account that currently has the authority over the to account and is granting the delegation.
        // };
        // anchor_spl::token::approve(CpiContext::new(cpi_program, cpi_accounts), 1)?;

        // // approve
        // //
        // let cpi_program = token_program.to_account_info();
        // let cpi_accounts = token_interface::ApproveChecked {
        //     mint: nft_mint.to_account_info(),
        //     to: config.to_account_info(), // TODO: will it work?
        //     delegate: user_nft_ata.to_account_info(),
        //     authority: user.to_account_info(),
        // };
        // token_interface::approve_checked(
        //     CpiContext::new(cpi_program, cpi_accounts),
        //     1,
        //     nft_mint.decimals,
        // )?;

        // // freeze
        // //
        // let cpi_program = metadata_program.to_account_info();
        // let cpi_accounts = token_interface::FreezeAccount {
        //     mint: nft_mint.to_account_info(),
        //     account: user_nft_ata.to_account_info(),
        //     authority: config.to_account_info(),
        // };
        // let seeds = &[b"config".as_ref(), &[config.config_bump]];

        // token_interface::freeze_account(CpiContext::new_with_signer(
        //     cpi_program,
        //     cpi_accounts,
        //     &[seeds],
        // ))?;

        Ok(())
    }
}
