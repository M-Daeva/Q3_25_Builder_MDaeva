use crate::state::Vault;

/// total_rewards = vault.rewards + rewards_rate * vault.amount * (clock_time - vault.updated_at)
pub fn calc_rewards(vault: &Vault, rewards_rate: u8, clock_time: u64) -> u64 {
    let staking_period = clock_time - std::cmp::min(clock_time, vault.updated_at);
    vault.rewards + (rewards_rate as u64) * vault.tokens.len() as u64 * staking_period
}

pub fn get_updated_vault(vault: &Vault, rewards_rate: u8, clock_time: u64) -> Vault {
    Vault {
        bump: vault.bump,
        tokens: vault.tokens.clone(),
        updated_at: clock_time,
        rewards: calc_rewards(vault, rewards_rate, clock_time),
    }
}
