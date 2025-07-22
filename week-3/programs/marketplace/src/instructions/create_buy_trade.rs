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
        helpers::{
            deserialize_account, get_space, transfer_sol_from_user, transfer_token_from_user,
        },
    },
};

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct CreateBuyWithTokenTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

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
        payer = buyer,
        space = get_space(Trade::INIT_SPACE),
        seeds = [b"trade", buyer.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
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
        associated_token::mint = token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = marketplace
    )]
    pub app_token_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreateBuyWithTokenTrade<'info> {
    pub fn create_buy_with_token_trade(
        &mut self,
        bump: u8,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        let CreateBuyWithTokenTrade {
            token_program,
            buyer,
            marketplace,
            trade,
            token_account,
            token_mint,
            buyer_token_ata,
            app_token_ata,
            ..
        } = self;

        if token_mint.key() != price.asset {
            Err(CustomError::AssetIsNotFound)?;
        }

        if !marketplace.asset_whitelist.contains(&price.asset) {
            Err(CustomError::AssetIsNotFound)?;
        }

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        transfer_token_from_user(
            price.amount,
            token_mint,
            buyer_token_ata,
            app_token_ata,
            buyer,
            token_program,
        )?;

        trade.set_inner(Trade {
            bump,
            is_sell_nft_trade: false,
            creator: buyer.key(),
            collection,
            token_id,
            price,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct CreateBuyWithSolTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

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
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = buyer,
        space = get_space(Trade::INIT_SPACE),
        seeds = [b"trade", buyer.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
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
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> CreateBuyWithSolTrade<'info> {
    pub fn create_buy_with_sol_trade(
        &mut self,
        bump: u8,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        let CreateBuyWithSolTrade {
            system_program,
            buyer,
            marketplace,
            trade,
            token_account,
            ..
        } = self;

        if price.asset != Pubkey::default() {
            Err(CustomError::AssetIsNotFound)?;
        }

        if !marketplace.asset_whitelist.contains(&price.asset) {
            Err(CustomError::AssetIsNotFound)?;
        }

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        transfer_sol_from_user(
            price.amount,
            buyer,
            &marketplace.to_account_info(),
            system_program,
        )?;

        trade.set_inner(Trade {
            bump,
            is_sell_nft_trade: false,
            creator: buyer.key(),
            collection,
            token_id,
            price,
        });

        Ok(())
    }
}
