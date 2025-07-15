use cosmwasm_std::StdResult;
use rand::{rngs::StdRng, Rng, SeedableRng};

use pretty_assertions::assert_eq;

use coral_base::{
    chat::types::{ChatItem, Text},
    converters::u128_to_dec,
};

use crate::helpers::{
    chat::ChatExtension,
    hash_generator::HashGeneratorExtension,
    suite::{core::Project, types::ProjectAccount},
};

#[test]
fn send_message_default() -> StdResult<()> {
    let mut p = Project::new();

    Ok(())
}
