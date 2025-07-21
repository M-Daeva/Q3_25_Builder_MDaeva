use {
    crate::{
        error::CustomError,
        state::{Asset, Marketplace, Trade},
    },
    anchor_lang::prelude::*,
    anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface},
    base::{
        error::NftError,
        helpers::{deserialize_account, get_space, transfer_to_program},
    },
};

#[derive(Accounts)]
#[instruction(is_sell_nft_trade: bool, collection: Pubkey, token_id: u16)]
pub struct CreateTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub admin: SystemAccount<'info>,

    // data storages
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = user,
        space = get_space(Trade::INIT_SPACE),
        seeds = [b"trade", user.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub trade: Account<'info, Trade>,

    /// CHECK: token_account
    #[account(
        seeds = [b"token", collection.as_ref(), token_id.to_le_bytes().as_ref()],
        seeds::program = nft_program.key(),
        bump
    )]
    pub token_account: AccountInfo<'info>,

    // mint
    //
    pub nft_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = marketplace
    )]
    pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreateTrade<'info> {
    pub fn create_trade(
        &mut self,
        bump: u8,
        is_sell_nft_trade: bool,
        collection: Pubkey,
        token_id: u16,
        price_amount: u64,
        price_asset: Asset,
    ) -> Result<()> {
        let CreateTrade {
            system_program,
            token_program,
            nft_program,
            user,
            admin,
            marketplace,
            trade,
            token_account,
            nft_mint,
            user_nft_ata,
            app_nft_ata,
        } = self;

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        if nft_token.mint != nft_mint.key() {
            Err(ProgramError::InvalidAccountData)?;
        }

        if !marketplace.asset_whitelist.contains(&price_asset) {
            Err(CustomError::AssetIsNotFound)?;
        }

        trade.set_inner(Trade {
            bump,
            is_sell_nft_trade,
            creator: user.key(),
            collection,
            token_id,
            price_amount,
            price_asset,
        });

        if is_sell_nft_trade {
            transfer_to_program(
                1,
                nft_mint,
                &user_nft_ata,
                &app_nft_ata,
                &user,
                token_program,
            )?;
        }

        // TODO: add else branch

        Ok(())
    }
}
