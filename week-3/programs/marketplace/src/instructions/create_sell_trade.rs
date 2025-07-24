use {
    crate::{
        error::CustomError,
        state::{AssetItem, Marketplace, Trade},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::{
        error::NftError,
        helpers::{deserialize_account, get_space, transfer_token_from_user},
    },
};

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct CreateSellForTokenTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub admin: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = seller,
        space = get_space(Trade::INIT_SPACE),
        seeds = [b"trade", seller.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
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
    pub token_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller
    )]
    pub seller_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = token_mint,
        associated_token::authority = seller
    )]
    pub seller_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = nft_mint,
        associated_token::authority = marketplace
    )]
    pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreateSellForTokenTrade<'info> {
    pub fn create_sell_for_token_trade(
        &mut self,
        bump: u8,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        let CreateSellForTokenTrade {
            token_program,
            seller,
            marketplace,
            trade,
            token_account,
            nft_mint,
            seller_nft_ata,
            app_nft_ata,
            ..
        } = self;

        if !marketplace.asset_whitelist.contains(&price.asset) {
            Err(CustomError::AssetIsNotFound)?;
        }

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if nft_mint.key() != nft_token.mint {
            Err(CustomError::AssetIsNotFound)?;
        }

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        trade.set_inner(Trade {
            bump,
            is_sell_nft_trade: true,
            creator: seller.key(),
            collection,
            token_id,
            price,
        });

        transfer_token_from_user(
            1,
            nft_mint,
            seller_nft_ata,
            app_nft_ata,
            seller,
            token_program,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct CreateSellForSolTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub admin: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = seller,
        space = get_space(Trade::INIT_SPACE),
        seeds = [b"trade", seller.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
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
        associated_token::authority = seller
    )]
    pub seller_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = nft_mint,
        associated_token::authority = marketplace
    )]
    pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreateSellForSolTrade<'info> {
    pub fn create_sell_for_sol_trade(
        &mut self,
        bump: u8,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        let CreateSellForSolTrade {
            token_program,
            seller,
            marketplace,
            trade,
            token_account,
            nft_mint,
            seller_nft_ata,
            app_nft_ata,
            ..
        } = self;

        if !marketplace.asset_whitelist.contains(&price.asset) {
            Err(CustomError::AssetIsNotFound)?;
        }

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if nft_mint.key() != nft_token.mint {
            Err(CustomError::AssetIsNotFound)?;
        }

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        trade.set_inner(Trade {
            bump,
            is_sell_nft_trade: true,
            creator: seller.key(),
            collection,
            token_id,
            price,
        });

        transfer_token_from_user(
            1,
            nft_mint,
            seller_nft_ata,
            app_nft_ata,
            seller,
            token_program,
        )?;

        Ok(())
    }
}
