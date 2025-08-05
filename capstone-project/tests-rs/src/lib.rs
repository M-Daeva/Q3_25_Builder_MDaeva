// #[cfg(test)]
// pub mod clmm;
#[cfg(test)]
pub mod clmm_mock;
#[cfg(test)]
pub mod dex_adapter;
#[cfg(test)]
pub mod registry;
#[cfg(test)]
pub mod wsol;

pub mod helpers {
    pub mod extensions {
        pub mod clmm;
        pub mod clmm_mock;
        pub mod dex_adapter;
        pub mod registry;
        pub mod wsol;
    }

    pub mod suite {
        pub mod core;
        pub mod decimal;
        pub mod types;
    }
}
