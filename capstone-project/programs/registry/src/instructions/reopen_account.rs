use {
    anchor_lang::prelude::*,
    base::helpers::{get_clock_time, get_space},
    registry_cpi::{
        error::CustomError,
        state::{
            Bump, Config, RotationState, UserAccount, UserId, SEED_BUMP, SEED_CONFIG,
            SEED_USER_ACCOUNT, SEED_USER_ID, SEED_USER_ROTATION_STATE,
        },
    },
};

#[derive(Accounts)]
#[instruction(max_data_size: u32)]
pub struct ReopenAccount<'info> {
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
        seeds = [SEED_USER_ID.as_bytes(), sender.key().as_ref()],
        bump
    )]
    pub user_id: Account<'info, UserId>,

    #[account(
        init,
        payer = sender,
        space = UserAccount::get_space(max_data_size),
        seeds = [SEED_USER_ACCOUNT.as_bytes(), user_id.id.to_le_bytes().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = sender,
        space = get_space(RotationState::INIT_SPACE),
        seeds = [SEED_USER_ROTATION_STATE.as_bytes(), user_id.id.to_le_bytes().as_ref()],
        bump
    )]
    pub user_rotation_state: Account<'info, RotationState>,
}

impl<'info> ReopenAccount<'info> {
    pub fn reopen_account(&mut self, max_data_size: u32) -> Result<()> {
        let Self {
            sender,
            config,
            user_id,
            user_account,
            user_rotation_state,
            ..
        } = self;

        // only closed account can be open
        if user_id.is_open {
            Err(CustomError::OpenAccountTwice)?;
        }

        // validate max allocated data size
        if max_data_size < config.data_size_range.min || max_data_size > config.data_size_range.max
        {
            Err(CustomError::MaxDataSizeIsOutOfRange)?;
        }

        user_id.is_open = true;

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
