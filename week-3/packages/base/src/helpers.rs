use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface},
    std::{collections::HashSet, hash::Hash},
};

pub const DISCRIMINATOR_SPACE: usize = 8;

/// checks if a list has duplicates
pub fn has_duplicates<T: Eq + Hash>(list: &[T]) -> bool {
    let mut set = HashSet::with_capacity(list.len());

    for item in list {
        if !set.insert(item) {
            return true;
        }
    }

    false
}

/// removes duplicates from a list
pub fn deduplicate<T: Eq + Hash + Clone>(list: &[T]) -> Vec<T> {
    let mut set = HashSet::with_capacity(list.len());

    list.iter()
        .filter_map(|item| {
            if set.insert(item) {
                Some(item.to_owned())
            } else {
                None
            }
        })
        .collect()
}

pub fn deserialize_account<T>(account: &mut AccountInfo) -> Result<T>
where
    T: AnchorSerialize + AnchorDeserialize,
{
    let data = &account.try_borrow_data()?;
    Ok(T::deserialize(&mut &data[DISCRIMINATOR_SPACE..])?)
}

pub fn get_space(struct_space: usize) -> usize {
    DISCRIMINATOR_SPACE + struct_space
}

pub fn get_rent_exempt<'a, T>(account: &T) -> Result<u64>
where
    T: ToAccountInfo<'a>,
{
    Ok(Rent::get()?.minimum_balance(account.to_account_info().data_len()))
}

pub fn transfer_sol_from_user<'a>(
    amount: u64,
    from: &Signer<'a>,
    to: &AccountInfo<'a>,
    system_program: &Program<'a, System>,
) -> Result<()> {
    let cpi_program = system_program.to_account_info();
    let cpi_accounts = system_program::Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };

    system_program::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)
}

pub fn transfer_sol_from_program<'a>(
    amount: u64,
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    seeds: &[&[u8]],
    bump: u8,
    system_program: &Program<'a, System>,
) -> Result<()> {
    let cpi_program = system_program.to_account_info();
    let cpi_accounts = system_program::Transfer {
        from: from.clone(),
        to: to.clone(),
    };

    let mut seeds_with_bump = seeds.to_vec();
    let binding = [bump];
    seeds_with_bump.push(&binding);

    system_program::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&seeds_with_bump]),
        amount,
    )
}

pub fn transfer_token_from_user<'a>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    from: &InterfaceAccount<'a, TokenAccount>,
    to: &InterfaceAccount<'a, TokenAccount>,
    signer: &Signer<'a>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()> {
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: signer.to_account_info(),
    };

    token_interface::transfer_checked(
        CpiContext::new(cpi_program, cpi_accounts),
        amount,
        mint.decimals,
    )
}

pub fn transfer_token_from_program<'a, T>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    from: &InterfaceAccount<'a, TokenAccount>,
    to: &InterfaceAccount<'a, TokenAccount>,
    seeds: &[&[u8]],
    bump: u8,
    authority: &Account<'a, T>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()>
where
    T: AccountSerialize + AccountDeserialize + Clone,
{
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: authority.to_account_info(),
    };

    let mut seeds_with_bump = seeds.to_vec();
    let binding = [bump];
    seeds_with_bump.push(&binding);

    token_interface::transfer_checked(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&seeds_with_bump]),
        amount,
        mint.decimals,
    )
}

pub fn mint_token_to<'a, T>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    to: &InterfaceAccount<'a, TokenAccount>,
    seeds: &[&[u8]],
    bump: u8,
    authority: &InterfaceAccount<'a, T>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()>
where
    T: AccountSerialize + AccountDeserialize + Clone,
{
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::MintToChecked {
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let mut seeds_with_bump = seeds.to_vec();
    let binding = [bump];
    seeds_with_bump.push(&binding);

    token_interface::mint_to_checked(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&seeds_with_bump]),
        amount,
        mint.decimals,
    )
}

pub fn burn_token_from<'a>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    from: &InterfaceAccount<'a, TokenAccount>,
    signer: &Signer<'a>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()> {
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::BurnChecked {
        mint: mint.to_account_info(),
        from: from.to_account_info(),
        authority: signer.to_account_info(),
    };

    token_interface::burn_checked(
        CpiContext::new(cpi_program, cpi_accounts),
        amount,
        mint.decimals,
    )
}
