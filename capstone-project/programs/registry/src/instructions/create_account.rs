use {
    crate::{
        error::CustomError,
        state::{
            Bump, Config, RotationState, UserAccount, UserCounter, UserId, SEED_BUMP, SEED_CONFIG,
            SEED_USER_ACCOUNT, SEED_USER_COUNTER, SEED_USER_ID, SEED_USER_ROTATION_STATE,
        },
    },
    anchor_lang::prelude::*,
    base::helpers::{get_clock_time, get_space},
};

#[derive(Accounts)]
#[instruction(max_data_size: u32, expected_user_id: u32)]
pub struct CreateAccount<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_USER_COUNTER.as_bytes()],
        bump = bump.user_counter
    )]
    pub user_counter: Account<'info, UserCounter>,

    #[account(
        init,
        payer = sender,
        space = get_space(UserId::INIT_SPACE),
        seeds = [SEED_USER_ID.as_bytes(), sender.key().as_ref()],
        bump
    )]
    pub user_id: Account<'info, UserId>,

    // user_id.id doesn't exist yet, use expected_user_id instead
    #[account(
        init,
        payer = sender,
        space = UserAccount::get_space(max_data_size),
        seeds = [SEED_USER_ACCOUNT.as_bytes(), expected_user_id.to_le_bytes().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = sender,
        space = get_space(RotationState::INIT_SPACE),
        seeds = [SEED_USER_ROTATION_STATE.as_bytes(), expected_user_id.to_le_bytes().as_ref()],
        bump
    )]
    pub user_rotation_state: Account<'info, RotationState>,
}

impl<'info> CreateAccount<'info> {
    pub fn create_account(
        &mut self,
        bumps: CreateAccountBumps,
        max_data_size: u32,
        expected_user_id: u32,
    ) -> Result<()> {
        let CreateAccount {
            sender,
            config,
            user_counter,
            user_id,
            user_account,
            user_rotation_state,
            ..
        } = self;

        if expected_user_id != user_counter.last_user_id + 1 {
            Err(CustomError::WrongUserId)?;
        }

        if config.is_paused {
            Err(CustomError::ContractIsPaused)?;
        }

        if max_data_size < config.data_size_range.min || max_data_size > config.data_size_range.max
        {
            Err(CustomError::MaxDataSizeIsOutOfRange)?;
        }

        user_counter.last_user_id = expected_user_id;

        user_id.set_inner(UserId {
            id: expected_user_id,
            is_open: true,
            is_activated: false,
            account_bump: bumps.user_account,
            rotation_state_bump: bumps.user_rotation_state,
        });

        user_account.set_inner(UserAccount {
            data: String::default(),
            nonce: 0,
            max_size: max_data_size,
        });

        user_rotation_state.set_inner(RotationState {
            owner: sender.key(),
            new_owner: None,
            expiration_date: get_clock_time()?,
        });

        Ok(())
    }
}
