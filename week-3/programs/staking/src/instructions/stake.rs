use {
    crate::{
        math::get_updated_vault,
        state::{Config, Vault},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::{
        error::NftError,
        helpers::{deserialize_account, get_space, has_duplicates, transfer_to_program},
    },
};

#[derive(Accounts)]
#[instruction(token_id: u16)]
pub struct Stake<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

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

    /// CHECK: token_account
    #[account(
        seeds = [b"token", config.collection.as_ref(), token_id.to_le_bytes().as_ref()],
        seeds::program = nft_program.key(),
        bump
    )]
    pub token_account: AccountInfo<'info>,

    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = config
    )]
    pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, token_id: u16) -> Result<()> {
        let clock_time = Clock::get()?.unix_timestamp as u64;
        let Stake {
            token_program,
            user,
            user_vault,
            config,
            token_account,
            nft_mint,
            user_nft_ata,
            app_nft_ata,
            ..
        } = self;

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if nft_token.collection != config.collection {
            Err(NftError::CollectionIsNotFound)?;
        }

        if nft_token.mint != nft_mint.key() {
            Err(ProgramError::InvalidAccountData)?;
        }

        user_vault.set_inner(get_updated_vault(
            &user_vault,
            config.rewards_rate,
            clock_time,
        ));
        user_vault.tokens.push(token_id);

        if has_duplicates(&user_vault.tokens) {
            Err(NftError::NftDuplication)?;
        }

        if user_vault.rewards > config.max_stake {
            Err(NftError::ExceededTokenLimit)?;
        }

        transfer_to_program(
            1,
            nft_mint,
            &user_nft_ata,
            &app_nft_ata,
            &user,
            token_program,
        )?;

        Ok(())
    }
}
