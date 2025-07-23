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
    base::helpers::{
        transfer_sol_from_user, transfer_token_from_program, transfer_token_from_user,
    },
};

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct AcceptSellForTokenTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    pub admin: SystemAccount<'info>,

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
        close = seller,
        seeds = [b"trade", seller.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub trade: Account<'info, Trade>,

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

impl<'info> AcceptSellForTokenTrade<'info> {
    pub fn accept_sell_for_token_trade(
        &mut self,
        _collection: Pubkey,
        _token_id: u16,
    ) -> Result<()> {
        let AcceptSellForTokenTrade {
            token_program,
            buyer,
            admin,
            marketplace,
            balances,
            trade,
            nft_mint,
            token_mint,
            seller_token_ata,
            buyer_token_ata,
            buyer_nft_ata,
            app_token_ata,
            app_nft_ata,
            ..
        } = self;

        if token_mint.key() != trade.price.asset || trade.price.asset == Pubkey::default() {
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

        // transfer to seller
        transfer_token_from_user(
            amount_to_seller,
            token_mint,
            buyer_token_ata,
            seller_token_ata,
            buyer,
            token_program,
        )?;

        // pay fee
        transfer_token_from_user(
            fee,
            token_mint,
            buyer_token_ata,
            app_token_ata,
            buyer,
            token_program,
        )?;

        // receive nft
        transfer_token_from_program(
            1,
            nft_mint,
            app_nft_ata,
            buyer_nft_ata,
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
pub struct AcceptSellForSolTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

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
        mut,
        seeds = [b"balances", admin.key().as_ref()],
        bump = marketplace.bump.balances
    )]
    pub balances: Account<'info, Balances>,

    #[account(
        mut,
        close = seller,
        seeds = [b"trade", seller.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub trade: Account<'info, Trade>,

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
    pub buyer_nft_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = marketplace
    )]
    pub app_nft_ata: Box<InterfaceAccount<'info, TokenAccount>>,
}

impl<'info> AcceptSellForSolTrade<'info> {
    pub fn accept_sell_for_sol_trade(&mut self, _collection: Pubkey, _token_id: u16) -> Result<()> {
        let AcceptSellForSolTrade {
            system_program,
            token_program,
            buyer,
            seller,
            admin,
            treasury,
            marketplace,
            balances,
            trade,
            nft_mint,
            buyer_nft_ata,
            app_nft_ata,
            ..
        } = self;

        if trade.price.asset != Pubkey::default() {
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

        // transfer to seller
        transfer_sol_from_user(amount_to_seller, buyer, seller, system_program)?;

        // pay fee
        transfer_sol_from_user(fee, buyer, &treasury.to_account_info(), system_program)?;

        // receive nft
        transfer_token_from_program(
            1,
            nft_mint,
            app_nft_ata,
            buyer_nft_ata,
            &[b"marketplace", admin.key().as_ref()],
            marketplace.bump.marketplace,
            marketplace,
            token_program,
        )?;

        Ok(())
    }
}
