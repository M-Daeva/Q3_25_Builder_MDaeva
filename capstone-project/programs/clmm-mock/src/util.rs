use {
    anchor_lang::{
        prelude::*,
        solana_program,
        system_program::{self, create_account, CreateAccount},
    },
    anchor_spl::{
        token,
        token_2022::{
            self, get_account_data_size, initialize_mint2,
            spl_token_2022::{
                self,
                extension::{metadata_pointer, ExtensionType},
            },
            GetAccountDataSize, InitializeAccount3, InitializeImmutableOwner, InitializeMint2,
            Token2022,
        },
        token_interface::{Mint, TokenInterface},
    },
};

pub fn create_token_vault_account<'info>(
    payer: &Signer<'info>,
    pool_state: &AccountInfo<'info>,
    token_account: &AccountInfo<'info>,
    token_mint: &InterfaceAccount<'info, Mint>,
    system_program: &Program<'info, System>,
    token_2022_program: &Interface<'info, TokenInterface>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let immutable_owner_required = false;
    // support both spl_token_program & token_program_2022
    let space = get_account_data_size(
        CpiContext::new(
            token_2022_program.to_account_info(),
            GetAccountDataSize {
                mint: token_mint.to_account_info(),
            },
        ),
        if immutable_owner_required {
            &[anchor_spl::token_2022::spl_token_2022::extension::ExtensionType::ImmutableOwner]
        } else {
            &[]
        },
    )?;

    // create account with or without lamports
    create_or_allocate_account(
        token_2022_program.key,
        payer.to_account_info(),
        system_program.to_account_info(),
        token_account.to_account_info(),
        signer_seeds,
        space.try_into().unwrap(),
    )?;

    // Call initializeImmutableOwner
    if immutable_owner_required {
        token_2022::initialize_immutable_owner(CpiContext::new(
            token_2022_program.to_account_info(),
            InitializeImmutableOwner {
                account: token_account.to_account_info(),
            },
        ))?;
    }

    // Call initializeAccount3
    token_2022::initialize_account3(CpiContext::new(
        token_2022_program.to_account_info(),
        InitializeAccount3 {
            account: token_account.to_account_info(),
            mint: token_mint.to_account_info(),
            authority: pool_state.to_account_info(),
        },
    ))?;

    Ok(())
}

pub fn create_or_allocate_account<'a>(
    program_id: &Pubkey,
    payer: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    target_account: AccountInfo<'a>,
    siger_seed: &[&[u8]],
    space: usize,
) -> Result<()> {
    let rent = Rent::get()?;
    let current_lamports = target_account.lamports();

    if current_lamports == 0 {
        let lamports = rent.minimum_balance(space);
        let cpi_accounts = system_program::CreateAccount {
            from: payer,
            to: target_account.clone(),
        };
        let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
        system_program::create_account(
            cpi_context.with_signer(&[siger_seed]),
            lamports,
            u64::try_from(space).unwrap(),
            program_id,
        )?;
    } else {
        let required_lamports = rent
            .minimum_balance(space)
            .max(1)
            .saturating_sub(current_lamports);
        if required_lamports > 0 {
            let cpi_accounts = system_program::Transfer {
                from: payer.to_account_info(),
                to: target_account.clone(),
            };
            let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
            system_program::transfer(cpi_context, required_lamports)?;
        }
        let cpi_accounts = system_program::Allocate {
            account_to_allocate: target_account.clone(),
        };
        let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
        system_program::allocate(
            cpi_context.with_signer(&[siger_seed]),
            u64::try_from(space).unwrap(),
        )?;

        let cpi_accounts = system_program::Assign {
            account_to_assign: target_account.clone(),
        };
        let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
        system_program::assign(cpi_context.with_signer(&[siger_seed]), program_id)?;
    }
    Ok(())
}

pub fn create_position_nft_mint_with_extensions<'info>(
    payer: &Signer<'info>,
    position_nft_mint: &AccountInfo<'info>,
    mint_authority: &AccountInfo<'info>,
    mint_close_authority: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    token_2022_program: &Program<'info, Token2022>,
    with_matedata: bool,
) -> Result<()> {
    let extensions = if with_matedata {
        [
            ExtensionType::MintCloseAuthority,
            ExtensionType::MetadataPointer,
        ]
        .to_vec()
    } else {
        [ExtensionType::MintCloseAuthority].to_vec()
    };
    let space =
        ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extensions)?;

    let lamports = Rent::get()?.minimum_balance(space);

    // create mint account
    create_account(
        CpiContext::new(
            system_program.to_account_info(),
            CreateAccount {
                from: payer.to_account_info(),
                to: position_nft_mint.to_account_info(),
            },
        ),
        lamports,
        space as u64,
        token_2022_program.key,
    )?;

    // initialize token extensions
    for e in extensions {
        match e {
            ExtensionType::MetadataPointer => {
                let ix = metadata_pointer::instruction::initialize(
                    token_2022_program.key,
                    position_nft_mint.key,
                    None,
                    Some(position_nft_mint.key()),
                )?;
                solana_program::program::invoke(
                    &ix,
                    &[
                        token_2022_program.to_account_info(),
                        position_nft_mint.to_account_info(),
                    ],
                )?;
            }
            ExtensionType::MintCloseAuthority => {
                let ix = spl_token_2022::instruction::initialize_mint_close_authority(
                    token_2022_program.key,
                    position_nft_mint.key,
                    Some(mint_close_authority.key),
                )?;
                solana_program::program::invoke(
                    &ix,
                    &[
                        token_2022_program.to_account_info(),
                        position_nft_mint.to_account_info(),
                    ],
                )?;
            }
            _ => {
                panic!("ErrorCode::NotSupportMint");
            }
        }
    }

    // initialize mint account
    initialize_mint2(
        CpiContext::new(
            token_2022_program.to_account_info(),
            InitializeMint2 {
                mint: position_nft_mint.to_account_info(),
            },
        ),
        0,
        &mint_authority.key(),
        None,
    )
}

pub fn transfer_from_user_to_pool_vault<'info>(
    signer: &Signer<'info>,
    from: &AccountInfo<'info>,
    to_vault: &AccountInfo<'info>,
    mint: Option<Box<InterfaceAccount<'info, Mint>>>,
    token_program: &AccountInfo<'info>,
    token_program_2022: Option<AccountInfo<'info>>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let mut token_program_info = token_program.to_account_info();
    let from_token_info = from.to_account_info();
    match (mint, token_program_2022) {
        (Some(mint), Some(token_program_2022)) => {
            if from_token_info.owner == token_program_2022.key {
                token_program_info = token_program_2022.to_account_info()
            }
            token_2022::transfer_checked(
                CpiContext::new(
                    token_program_info,
                    token_2022::TransferChecked {
                        from: from_token_info,
                        to: to_vault.to_account_info(),
                        authority: signer.to_account_info(),
                        mint: mint.to_account_info(),
                    },
                ),
                amount,
                mint.decimals,
            )
        }
        _ => token::transfer(
            CpiContext::new(
                token_program_info,
                token::Transfer {
                    from: from_token_info,
                    to: to_vault.to_account_info(),
                    authority: signer.to_account_info(),
                },
            ),
            amount,
        ),
    }
}
