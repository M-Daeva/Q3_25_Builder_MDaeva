use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub marketplace_bump: u8,
    pub balances_bump: u8,
    /// The wallet address of the marketplace administrator/authority
    pub admin: Pubkey,
    /// The marketplace fee percentage in basis points (e.g., 250 = 2.5%)
    pub fee_bps: u16,
    #[max_len(32)]
    pub collection_whitelist: Vec<Pubkey>,
    #[max_len(32)]
    pub asset_whitelist: Vec<Asset>,
    /// The name of the marketplace used for branding and identification
    #[max_len(32)]
    pub name: String,
}

#[account]
#[derive(InitSpace)]
pub struct Balances {
    #[max_len(32)]
    pub value: Vec<BalanceItem>,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct BalanceItem {
    pub amount: u64,
    pub asset: Asset,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Trade {
    pub bump: u8,
    pub is_sell_nft_trade: bool,
    pub creator: Pubkey,
    pub collection: Pubkey,
    pub token_id: u16,
    pub price_amount: u64,
    pub price_asset: Asset,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum Asset {
    Sol, // TODO: use Pubkey::default
    Mint(Pubkey),
}

// nft program
//
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Token {
    pub token_bump: u8,
    pub mint_bump: u8,
    pub id: u16,
    pub collection: Pubkey,
    pub mint: Pubkey,
    pub metadata: String,
}
