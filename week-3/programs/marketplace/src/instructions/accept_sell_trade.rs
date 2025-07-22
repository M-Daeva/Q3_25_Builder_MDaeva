use {
    crate::{
        error::CustomError,
        state::{Asset, Balances, Marketplace, Trade},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::{
        error::NftError,
        helpers::{deserialize_account, get_space, transfer_from_program, transfer_from_user},
    },
};

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct AcceptSellTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    pub admin: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"balances", admin.key().as_ref()],
        bump = marketplace.balances_bump
    )]
    pub balances: Account<'info, Balances>,

    #[account(
        mut,
        close = seller,
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
        associated_token::mint = token_mint,
        associated_token::authority = seller
    )]
    pub seller_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = marketplace
    )]
    pub app_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = marketplace
    )]
    pub app_nft_ata: Box<InterfaceAccount<'info, TokenAccount>>,
}

impl<'info> AcceptSellTrade<'info> {
    pub fn accept_sell_trade(&mut self, collection: Pubkey, token_id: u16) -> Result<()> {
        let AcceptSellTrade {
            system_program,
            token_program,
            associated_token_program,
            nft_program,
            buyer,
            seller,
            admin,
            marketplace,
            balances,
            trade,
            token_account,
            nft_mint,
            token_mint,
            seller_token_ata,
            buyer_token_ata,
            buyer_nft_ata,
            app_token_ata,
            app_nft_ata,
        } = self;

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if !marketplace.asset_whitelist.contains(&trade.price_asset) {
            Err(CustomError::AssetIsNotFound)?;
        }

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        let is_asset_correct = match trade.price_asset {
            Asset::Sol => false, // TODO
            Asset::Mint(token) => token == token_mint.key(),
        };

        if !is_asset_correct {
            Err(CustomError::AssetIsNotFound)?;
        }

        let fee = (trade.price_amount as u128 * marketplace.fee_bps as u128 / 10_000_u128) as u64;
        let amount_to_seller = trade.price_amount - fee;

        balances.value = balances
            .value
            .iter()
            .cloned()
            .map(|mut x| {
                if x.asset == trade.price_asset {
                    x.amount += fee;
                }

                x
            })
            .collect();

        // transfer to seller
        transfer_from_user(
            amount_to_seller,
            token_mint,
            buyer_token_ata,
            seller_token_ata,
            buyer,
            token_program,
        )?;

        // pay fee
        transfer_from_user(
            fee,
            token_mint,
            buyer_token_ata,
            app_token_ata,
            buyer,
            token_program,
        )?;

        // receive nft
        transfer_from_program(
            1,
            nft_mint,
            app_nft_ata,
            buyer_nft_ata,
            &[b"marketplace", admin.key().as_ref()],
            marketplace.marketplace_bump,
            marketplace,
            token_program,
        )?;

        Ok(())
    }
}
