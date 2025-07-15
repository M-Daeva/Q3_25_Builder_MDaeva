use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use crate::helpers::suite::decimal::{str_to_dec, Decimal};

pub const DEFAULT_SOL_AMOUNT: u64 = 1_000;
pub const INCREASED_SOL_AMOUNT: u64 = 100_000;

pub const SOL_DECIMALS: u8 = 9;
pub const DEFAULT_TOKEN_DECIMALS: u8 = 6;
pub const WBTC_TOKEN_DECIMALS: u8 = 8;

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectAccount {
    Admin,
    Alice,
    Bob,
    //
    // "5gi185z4U57MEkJJTzweNrJQJQftaQH2onL8ZGRyXKWA3zspJWyQfF1J8ZRV7zd3D8aZyZxtaw8MsPZpMLMGh6L2"
    // "8XdLEJXrM3yYfFg5EpMqcCKmXXSQeBKfTjvNP619LbE2"

    // "5fbcPBxRADG5oxsK3K7PtM5A2CXFSErQ7bWoTXA1qeZsngyFYzWKUm4R7pBtD9fazVA9FgFC4h4WschCTQ7xjeJG"
    // "HBtzyBH14hR6t5UfYT3ptL6d1pMnVCep2RY8vUgHmaRA"

    // "2RyN2wrHo8fDrvqULn61ThcSeMyBE3eQ35ADxk5bvjkMrtZRKZwYNRQgxS33UkTrw3udySYMeoJxapbLbyz3aDiZ"
    // "An6eCPnnsspFAy5bUrgnNkU4hkedv9ZDRUJazUTG1ewb"

    // "35DNq3ZSomYUDFCA3tGKQNFgmnbRLpYzoioUGo2Pbu6hSzz4uXKLFNMWHnxN3c95yiQooZ3L2gS3yfcpHog5WDB7"
    // "5uXsgfEWa6LNEMcFd7Box2haMVA5BnBTU26wr26irend"

    // "5pgbqemaPfHvkUP7tKBohFojQtwQ5eRVUGDX5DT9rbhBTx1cGjLi9eCLGQzUuigANBFkihsixn7BKYa1QkMEwjW2"
    // "H8aGuHSeWFSsStZX1oUyXGsdpDaAdW1EPTKjvJCBQZU6"

    // "2LPw7PJBb9LYsnjDSfquWqUwCUjnCZagTmNu728HCajL3qGWyKsoLCGJhG95FfFD9GjBaunvPUJoe3pHaVi3Rphj"
    // "DaZEjWGiyhKRNWzRRgkCqVzvF4GXaKRzSpykQ1j1g5id"

    // "2LqYQK6T3vfK2V9bqqMT9UHYzk1zhLqRpzupsDzn5B7ngEU4c17hPzPiP2QEWKKR87p98mK7QdEdCc1xuQkQykBj"
    // "3PqqfCCyx1gJeH78qVLdXAzzDYifb1MGgUGF3xJZZTnH"

    // "xwc4MbnqQQJP174H34wk4zHFkkkLxxmfWYaZN9rHJ8kRho861SbjZKjRwzWHysjLnWPg5P59GR3se56EeD846X3"
    // "74CFAjJLQwZ42rLuybcJZsBJ2zWtyhYsJarDp1CiRiFB"
}

impl ProjectAccount {
    pub fn pubkey(&self) -> Pubkey {
        let str_const = match self {
            ProjectAccount::Admin => "Fk5wpZL2pV8AYkMKnEo5TAJ1p88FmUxBbKsZLwpiqWqQ",
            ProjectAccount::Alice => "68ZZmGRDn5971SDrj5Ldj6MUJTeRUdSV1NQUuzsaQ4N3",
            ProjectAccount::Bob => "FPS369ZvUkQTdsU8pzmypafNnNghiDHi8G6gDCvux5SB",
        };

        Pubkey::from_str_const(str_const)
    }

    pub fn keypair(&self) -> Keypair {
        let base58_string = match self {
            ProjectAccount::Admin => "3SKiuW2cbAJH8KDAuhB5cdJnAGU8Y9a95gRWMFB6zPy8XH45HTNebRALhL1EqPv2QkBytb8iTu577TcmLutkzC9g",
            ProjectAccount::Alice => "4TwYiTAG6eHLznaSGZinmQGSFxKxxmx7DHwKcbs5WkasMmLPP5fv1BYKJjsfmR47KFzmA2gs5DHtsZnR8YvMCinB",
            ProjectAccount::Bob => "zsbe2oRXt1K3gRNCurjZFTVzQtYqJjhyPAQMk4VsLWe3QoU3pMGZDVVRvmgZXgLtXvAsC9kGi4ShpYpjrQbtaf5"
        };

        Keypair::from_base58_string(base58_string)
    }

    pub fn get_initial_sol_amount(&self) -> u64 {
        match self {
            ProjectAccount::Admin => INCREASED_SOL_AMOUNT,
            _ => DEFAULT_SOL_AMOUNT,
        }
    }
}

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectCoin {
    SOL,
}

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectToken {
    USDC,
    PYTH,
    WBTC,
}

// #[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
// pub enum ProjectNft {
//     Gopniks,
//     Pigeons,
// }

pub trait GetPrice {
    fn get_price(&self) -> Decimal;
}

impl GetPrice for ProjectAsset {
    fn get_price(&self) -> Decimal {
        match self {
            ProjectAsset::Coin(project_coin) => project_coin.get_price(),
            ProjectAsset::Token(project_token) => project_token.get_price(),
        }
    }
}

impl GetPrice for ProjectCoin {
    fn get_price(&self) -> Decimal {
        match self {
            ProjectCoin::SOL => str_to_dec("160"),
        }
    }
}

impl GetPrice for ProjectToken {
    fn get_price(&self) -> Decimal {
        match self {
            ProjectToken::USDC => str_to_dec("1"),
            ProjectToken::PYTH => str_to_dec("0.1"),
            ProjectToken::WBTC => str_to_dec("120000"),
        }
    }
}

pub trait GetDecimals {
    fn get_decimals(&self) -> u8;
}

impl GetDecimals for ProjectAsset {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectAsset::Coin(project_coin) => project_coin.get_decimals(),
            ProjectAsset::Token(project_token) => project_token.get_decimals(),
        }
    }
}

impl GetDecimals for ProjectCoin {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectCoin::SOL => SOL_DECIMALS,
        }
    }
}

impl GetDecimals for ProjectToken {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectToken::USDC => DEFAULT_TOKEN_DECIMALS,
            ProjectToken::PYTH => DEFAULT_TOKEN_DECIMALS,
            ProjectToken::WBTC => WBTC_TOKEN_DECIMALS,
        }
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum ProjectAsset {
    Coin(ProjectCoin),
    Token(ProjectToken),
}

impl From<ProjectCoin> for ProjectAsset {
    fn from(project_coin: ProjectCoin) -> Self {
        Self::Coin(project_coin)
    }
}

impl From<ProjectToken> for ProjectAsset {
    fn from(project_token: ProjectToken) -> Self {
        Self::Token(project_token)
    }
}
