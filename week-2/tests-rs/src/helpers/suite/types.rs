// use cosmwasm_std::{Addr, Binary, Decimal, StdResult};
// use cw_multi_test::AppResponse;

use solana_keypair::Keypair;
// use anyhow::Error;
use strum_macros::{Display, EnumIter, IntoStaticStr};

// use coral_base::converters::str_to_dec;

// pub const P6: u128 = 1_000_000; // 1 asset with 6 decimals
// pub const P12: u128 = P6.pow(2); // 1_000_000 of assets with 6 decimals
// pub const P18: u128 = P6 * P12; // 1 of asset with 18 decimals
// pub const P24: u128 = P12.pow(2); // 1_000_000 of assets with 18 decimals

// pub const DEFAULT_FUNDS_AMOUNT: u128 = P12; // give each user 1 asset (1 CRD, 1 INJ, etc.)
// pub const INCREASED_FUNDS_AMOUNT: u128 = 100 * P12; // give admin such amount of assets to ensure providing 1e6 of assets to each pair

// pub const DEFAULT_DECIMALS: u8 = 6;
// pub const INCREASED_DECIMALS: u8 = 18;

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectAccount {
    #[strum(serialize = "Fk5wpZL2pV8AYkMKnEo5TAJ1p88FmUxBbKsZLwpiqWqQ")]
    Admin,
    #[strum(serialize = "68ZZmGRDn5971SDrj5Ldj6MUJTeRUdSV1NQUuzsaQ4N3")]
    Alice,
    #[strum(serialize = "FPS369ZvUkQTdsU8pzmypafNnNghiDHi8G6gDCvux5SB")]
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
    pub fn keypair(&self) -> Keypair {
        let base58_string = match self {
            ProjectAccount::Admin => "3SKiuW2cbAJH8KDAuhB5cdJnAGU8Y9a95gRWMFB6zPy8XH45HTNebRALhL1EqPv2QkBytb8iTu577TcmLutkzC9g",
            ProjectAccount::Alice => "4TwYiTAG6eHLznaSGZinmQGSFxKxxmx7DHwKcbs5WkasMmLPP5fv1BYKJjsfmR47KFzmA2gs5DHtsZnR8YvMCinB",
            ProjectAccount::Bob => "zsbe2oRXt1K3gRNCurjZFTVzQtYqJjhyPAQMk4VsLWe3QoU3pMGZDVVRvmgZXgLtXvAsC9kGi4ShpYpjrQbtaf5"
        };

        Keypair::from_base58_string(base58_string)
    }
}

// impl ProjectAccount {
//     pub fn get_initial_funds_amount(&self) -> u128 {
//         match self {
//             ProjectAccount::Admin => INCREASED_FUNDS_AMOUNT,
//             _ => DEFAULT_FUNDS_AMOUNT,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
// pub enum ProjectCoin {
//     #[strum(serialize = "ustars")]
//     Stars,
//     #[strum(serialize = "factory/wasm1s/uusdc")]
//     Usdc,
//     #[strum(serialize = "factory/wasm1s/ukuji")]
//     Kuji,
//     #[strum(serialize = "factory/wasm1s/uusk")]
//     Usk,
// }

// #[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
// pub enum ProjectToken {
//     #[strum(serialize = "wasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsqq5gpq")]
//     Atom,
//     #[strum(serialize = "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d")]
//     Luna,
//     #[strum(serialize = "wasm1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrss5maay")]
//     Inj,
// }

// pub trait GetPrice {
//     fn get_price(&self) -> Decimal;
// }

// impl GetPrice for ProjectAsset {
//     fn get_price(&self) -> Decimal {
//         match self {
//             ProjectAsset::Coin(project_coin) => project_coin.get_price(),
//             ProjectAsset::Token(project_token) => project_token.get_price(),
//         }
//     }
// }

// impl GetPrice for ProjectCoin {
//     fn get_price(&self) -> Decimal {
//         match self {
//             ProjectCoin::Stars => str_to_dec("0.01"),
//             ProjectCoin::Usdc => str_to_dec("1"),
//             ProjectCoin::Kuji => str_to_dec("1.5"),
//             ProjectCoin::Usk => str_to_dec("1"),
//         }
//     }
// }

// impl GetPrice for ProjectToken {
//     fn get_price(&self) -> Decimal {
//         match self {
//             ProjectToken::Atom => str_to_dec("10"),
//             ProjectToken::Luna => str_to_dec("0.5"),
//             ProjectToken::Inj => str_to_dec("5"),
//         }
//     }
// }

// pub trait GetDecimals {
//     fn get_decimals(&self) -> u8;
// }

// impl GetDecimals for ProjectAsset {
//     fn get_decimals(&self) -> u8 {
//         match self {
//             ProjectAsset::Coin(project_coin) => project_coin.get_decimals(),
//             ProjectAsset::Token(project_token) => project_token.get_decimals(),
//         }
//     }
// }

// impl GetDecimals for ProjectCoin {
//     fn get_decimals(&self) -> u8 {
//         match self {
//             ProjectCoin::Stars => DEFAULT_DECIMALS,
//             ProjectCoin::Usdc => DEFAULT_DECIMALS,
//             ProjectCoin::Kuji => DEFAULT_DECIMALS,
//             ProjectCoin::Usk => DEFAULT_DECIMALS,
//         }
//     }
// }

// impl GetDecimals for ProjectToken {
//     fn get_decimals(&self) -> u8 {
//         match self {
//             ProjectToken::Atom => DEFAULT_DECIMALS,
//             ProjectToken::Luna => DEFAULT_DECIMALS,
//             ProjectToken::Inj => INCREASED_DECIMALS,
//         }
//     }
// }

// impl From<ProjectAccount> for Addr {
//     fn from(project_account: ProjectAccount) -> Self {
//         Self::unchecked(project_account.to_string())
//     }
// }

// impl From<ProjectToken> for Addr {
//     fn from(project_token: ProjectToken) -> Self {
//         Addr::unchecked(project_token.to_string())
//     }
// }

// impl From<ProjectCoin> for Token {
//     fn from(project_coin: ProjectCoin) -> Self {
//         Self::new_native(&project_coin.to_string())
//     }
// }

// impl From<ProjectToken> for Token {
//     fn from(project_token: ProjectToken) -> Self {
//         Self::new_cw20(&project_token.into())
//     }
// }

// #[derive(Debug, Clone, Copy, Display)]
// pub enum ProjectAsset {
//     Coin(ProjectCoin),
//     Token(ProjectToken),
// }

// impl From<ProjectCoin> for ProjectAsset {
//     fn from(project_coin: ProjectCoin) -> Self {
//         Self::Coin(project_coin)
//     }
// }

// impl From<ProjectToken> for ProjectAsset {
//     fn from(project_token: ProjectToken) -> Self {
//         Self::Token(project_token)
//     }
// }

// #[derive(Debug, Clone, Copy, EnumIter)]
// pub enum ProjectPair {
//     AtomLuna,
//     StarsInj,
//     StarsLuna,
//     StarsNoria,
// }

// impl ProjectPair {
//     pub fn split_pair(&self) -> (ProjectAsset, ProjectAsset) {
//         match self {
//             ProjectPair::AtomLuna => (ProjectToken::Atom.into(), ProjectToken::Luna.into()),
//             ProjectPair::StarsInj => (ProjectCoin::Kuji.into(), ProjectToken::Inj.into()),
//             ProjectPair::StarsLuna => (ProjectCoin::Kuji.into(), ProjectToken::Luna.into()),
//             ProjectPair::StarsNoria => (ProjectCoin::Kuji.into(), ProjectCoin::Usk.into()),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum WrappedResponse {
//     Execute(Result<AppResponse, Error>),
//     Query(StdResult<Binary>),
// }

// impl From<Result<AppResponse, Error>> for WrappedResponse {
//     fn from(exec_res: Result<AppResponse, Error>) -> Self {
//         Self::Execute(exec_res)
//     }
// }

// impl From<StdResult<Binary>> for WrappedResponse {
//     fn from(query_res: StdResult<Binary>) -> Self {
//         Self::Query(query_res)
//     }
// }
