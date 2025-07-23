use {
    crate::{
        error::CustomError,
        state::{Balances, Marketplace, Trade},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::{
        error::NftError,
        helpers::{
            deserialize_account, transfer_sol_from_program, transfer_token_from_program,
            transfer_token_from_user,
        },
    },
};

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct AcceptBuyWithTokenTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub admin: SystemAccount<'info>,
    pub buyer: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"balances", admin.key().as_ref()],
        bump = marketplace.bump.balances
    )]
    pub balances: Account<'info, Balances>,

    #[account(
        mut,
        close = buyer,
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
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = token_mint,
        associated_token::authority = seller
    )]
    pub seller_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller
    )]
    pub seller_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = marketplace
    )]
    pub app_token_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> AcceptBuyWithTokenTrade<'info> {
    pub fn accept_buy_with_token_trade(
        &mut self,
        _collection: Pubkey,
        _token_id: u16,
    ) -> Result<()> {
        let AcceptBuyWithTokenTrade {
            token_program,
            seller,
            admin,
            marketplace,
            balances,
            trade,
            token_account,
            nft_mint,
            token_mint,
            buyer_nft_ata,
            seller_token_ata,
            seller_nft_ata,
            app_token_ata,
            ..
        } = self;

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        if nft_mint.key() != nft_token.mint {
            Err(CustomError::AssetIsNotFound)?;
        }

        let fee = (trade.price.amount as u128 * marketplace.fee_bps as u128 / 10_000_u128) as u64;
        let amount_to_seller = trade.price.amount - fee;

        balances.value = balances
            .value
            .iter()
            .cloned()
            .map(|mut x| {
                if x.asset == trade.price.asset {
                    x.amount += fee;
                }

                x
            })
            .collect();

        // transfer nft to buyer
        transfer_token_from_user(
            1,
            nft_mint,
            seller_nft_ata,
            buyer_nft_ata,
            seller,
            token_program,
        )?;

        // transfer tokens to seller
        transfer_token_from_program(
            amount_to_seller,
            token_mint,
            app_token_ata,
            seller_token_ata,
            &[b"marketplace", admin.key().as_ref()],
            marketplace.bump.marketplace,
            marketplace,
            token_program,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct AcceptBuyWithSolTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub admin: SystemAccount<'info>,

    #[account(mut)]
    pub buyer: SystemAccount<'info>,

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
        mut,
        seeds = [b"balances", admin.key().as_ref()],
        bump = marketplace.bump.balances
    )]
    pub balances: Account<'info, Balances>,

    #[account(
        mut,
        close = buyer,
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
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller
    )]
    pub seller_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> AcceptBuyWithSolTrade<'info> {
    pub fn accept_buy_with_sol_trade(&mut self, _collection: Pubkey, _token_id: u16) -> Result<()> {
        let AcceptBuyWithSolTrade {
            system_program,
            token_program,
            seller,
            admin,
            treasury,
            marketplace,
            balances,
            trade,
            token_account,
            nft_mint,
            buyer_nft_ata,
            seller_nft_ata,
            ..
        } = self;

        let nft_token: crate::state::Token = deserialize_account(token_account)?;

        if !marketplace
            .collection_whitelist
            .contains(&nft_token.collection)
        {
            Err(NftError::CollectionIsNotFound)?;
        }

        if nft_mint.key() != nft_token.mint {
            Err(CustomError::AssetIsNotFound)?;
        }

        let fee = (trade.price.amount as u128 * marketplace.fee_bps as u128 / 10_000_u128) as u64;
        let amount_to_seller = trade.price.amount - fee;

        balances.value = balances
            .value
            .iter()
            .cloned()
            .map(|mut x| {
                if x.asset == trade.price.asset {
                    x.amount += fee;
                }

                x
            })
            .collect();

        // transfer nft to buyer
        transfer_token_from_user(
            1,
            nft_mint,
            seller_nft_ata,
            buyer_nft_ata,
            seller,
            token_program,
        )?;

        // transfer sol to seller
        transfer_sol_from_program(
            amount_to_seller,
            &treasury.to_account_info(),
            &seller,
            &[b"treasury", admin.key().as_ref()],
            marketplace.bump.treasury,
            system_program,
        )?;

        Ok(())
    }
}
