use crate::state::PoolConfig;

pub fn calc_sqrt(a: u64, b: u64) -> u64 {
    (a as u128 * b as u128).isqrt() as u64
}

pub fn calc_shares(
    lp_amount: u64,
    is_mint_out_x: bool,
    total_mint_x: u64,
    total_mint_y: u64,
    total_mint_lp: u64,
) -> u64 {
    let total_mint_y = total_mint_y as u128;
    let total_mint_x = total_mint_x as u128;
    let total_mint_lp = total_mint_lp as u128;
    let lp_amount = lp_amount as u128;

    if total_mint_lp == 0 {
        return 0;
    }

    let shares = if is_mint_out_x {
        total_mint_x * lp_amount / total_mint_lp
    } else {
        total_mint_y * lp_amount / total_mint_lp
    };

    shares as u64
}

pub fn calc_amount_out(
    amount_in: u64,
    is_mint_in_x: bool,
    total_mint_x: u64,
    total_mint_y: u64,
) -> u64 {
    let total_mint_y = total_mint_y as u128;
    let total_mint_x = total_mint_x as u128;
    let amount_in = amount_in as u128;

    let k = total_mint_x * total_mint_y;
    let amount_out = if is_mint_in_x {
        let amount_in_full = total_mint_x + amount_in;

        if amount_in_full == 0 {
            0
        } else {
            total_mint_y - k / amount_in_full
        }
    } else {
        let amount_in_full = total_mint_y + amount_in;

        if amount_in_full == 0 {
            0
        } else {
            total_mint_x - k / amount_in_full
        }
    };

    amount_out as u64
}

pub fn calc_fee(amount_out: u64, pool_config: &PoolConfig) -> u64 {
    (amount_out as u128 * pool_config.fee_bps as u128 / 10_000_u128) as u64
}
