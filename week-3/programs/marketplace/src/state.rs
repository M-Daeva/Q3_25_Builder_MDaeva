use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub bump: Bump,
    /// The wallet address of the marketplace administrator/authority
    pub admin: Pubkey,
    /// The marketplace fee percentage in basis points (e.g., 250 = 2.5%)
    pub fee_bps: u16,
    #[max_len(32)]
    pub collection_whitelist: Vec<Pubkey>,
    #[max_len(32)]
    pub asset_whitelist: Vec<Pubkey>,
    /// The name of the marketplace used for branding and identification
    #[max_len(32)]
    pub name: String,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Bump {
    pub marketplace: u8,
    pub balances: u8,
    pub treasury: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Balances {
    #[max_len(32)]
    pub value: Vec<AssetItem>,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct AssetItem {
    pub amount: u64,
    /// Sol is Pubkey::default()
    pub asset: Pubkey,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Trade {
    pub bump: u8,
    pub is_sell_nft_trade: bool,
    pub creator: Pubkey,
    pub collection: Pubkey,
    pub token_id: u16,
    pub price: AssetItem,
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
