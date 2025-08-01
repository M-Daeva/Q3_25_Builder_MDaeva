use {
    crate::helpers::suite::decimal::{str_to_dec, Decimal},
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    strum_macros::{Display, EnumIter, IntoStaticStr},
};

const ASSET_AMOUNT_DEFAULT: u64 = 1_000;
const ASSET_AMOUNT_INCREASED: u64 = 100_000;

const DECIMALS_COIN_SOL: u8 = 9;
const DECIMALS_TOKEN_DEFAULT: u8 = 6;
const DECIMALS_TOKEN_WBTC: u8 = 8;

const PRICE_COIN_SOL: &str = "160";
const PRICE_TOKEN_USDC: &str = "1";
const PRICE_TOKEN_PYTH: &str = "0.1";
const PRICE_TOKEN_WBTC: &str = "120000";

const KEYPAIR_ADMIN: &str =
    "3SKiuW2cbAJH8KDAuhB5cdJnAGU8Y9a95gRWMFB6zPy8XH45HTNebRALhL1EqPv2QkBytb8iTu577TcmLutkzC9g";
const PUBKEY_ADMIN: &str = "Fk5wpZL2pV8AYkMKnEo5TAJ1p88FmUxBbKsZLwpiqWqQ";

const KEYPAIR_ALICE: &str =
    "4TwYiTAG6eHLznaSGZinmQGSFxKxxmx7DHwKcbs5WkasMmLPP5fv1BYKJjsfmR47KFzmA2gs5DHtsZnR8YvMCinB";
const PUBKEY_ALICE: &str = "68ZZmGRDn5971SDrj5Ldj6MUJTeRUdSV1NQUuzsaQ4N3";

const KEYPAIR_BOB: &str =
    "zsbe2oRXt1K3gRNCurjZFTVzQtYqJjhyPAQMk4VsLWe3QoU3pMGZDVVRvmgZXgLtXvAsC9kGi4ShpYpjrQbtaf5";
const PUBKEY_BOB: &str = "FPS369ZvUkQTdsU8pzmypafNnNghiDHi8G6gDCvux5SB";

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter, PartialEq)]
pub enum AppUser {
    Admin,
    Alice,
    Bob,
}

impl AppUser {
    pub fn pubkey(&self) -> Pubkey {
        let str_const = match self {
            Self::Admin => PUBKEY_ADMIN,
            Self::Alice => PUBKEY_ALICE,
            Self::Bob => PUBKEY_BOB,
        };

        Pubkey::from_str_const(str_const)
    }

    pub fn keypair(&self) -> Keypair {
        let base58_string = match self {
            Self::Admin => KEYPAIR_ADMIN,
            Self::Alice => KEYPAIR_ALICE,
            Self::Bob => KEYPAIR_BOB,
        };

        Keypair::from_base58_string(base58_string)
    }

    pub fn get_initial_asset_amount(&self) -> u64 {
        match self {
            Self::Admin => ASSET_AMOUNT_INCREASED,
            _ => ASSET_AMOUNT_DEFAULT,
        }
    }
}

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter, PartialEq)]
pub enum AppCoin {
    SOL,
}

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter, PartialEq)]
pub enum AppToken {
    USDC,
    PYTH,
    WBTC,
}

pub trait GetPrice {
    fn get_price(&self) -> Decimal;
}

impl GetPrice for AppAsset {
    fn get_price(&self) -> Decimal {
        match self {
            Self::Coin(project_coin) => project_coin.get_price(),
            Self::Token(project_token) => project_token.get_price(),
        }
    }
}

impl GetPrice for AppCoin {
    fn get_price(&self) -> Decimal {
        let price = match self {
            Self::SOL => PRICE_COIN_SOL,
        };

        str_to_dec(price)
    }
}

impl GetPrice for AppToken {
    fn get_price(&self) -> Decimal {
        let price = match self {
            Self::USDC => PRICE_TOKEN_USDC,
            Self::PYTH => PRICE_TOKEN_PYTH,
            Self::WBTC => PRICE_TOKEN_WBTC,
        };

        str_to_dec(price)
    }
}

pub trait GetDecimals {
    fn get_decimals(&self) -> u8;
}

impl GetDecimals for AppAsset {
    fn get_decimals(&self) -> u8 {
        match self {
            Self::Coin(project_coin) => project_coin.get_decimals(),
            Self::Token(project_token) => project_token.get_decimals(),
        }
    }
}

impl GetDecimals for AppCoin {
    fn get_decimals(&self) -> u8 {
        match self {
            Self::SOL => DECIMALS_COIN_SOL,
        }
    }
}

impl GetDecimals for AppToken {
    fn get_decimals(&self) -> u8 {
        match self {
            Self::USDC => DECIMALS_TOKEN_DEFAULT,
            Self::PYTH => DECIMALS_TOKEN_DEFAULT,
            Self::WBTC => DECIMALS_TOKEN_WBTC,
        }
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum AppAsset {
    Coin(AppCoin),
    Token(AppToken),
}

impl From<AppCoin> for AppAsset {
    fn from(project_coin: AppCoin) -> Self {
        Self::Coin(project_coin)
    }
}

impl From<AppToken> for AppAsset {
    fn from(project_token: AppToken) -> Self {
        Self::Token(project_token)
    }
}
