use {
    anchor_lang::prelude::*,
    registry_cpi::{
        error::CustomError,
        state::{UserAccount, UserId, SEED_USER_ACCOUNT, SEED_USER_ID},
    },
};

#[derive(Accounts)]
pub struct WriteData<'info> {
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_USER_ID.as_bytes(), sender.key().as_ref()],
        bump
    )]
    pub user_id: Account<'info, UserId>,

    #[account(
        mut,
        seeds = [SEED_USER_ACCOUNT.as_bytes(), user_id.id.to_le_bytes().as_ref()],
        bump = user_id.account_bump
    )]
    pub user_account: Account<'info, UserAccount>,
}

impl<'info> WriteData<'info> {
    pub fn write_data(&mut self, data: String, nonce: u64) -> Result<()> {
        let Self {
            user_id,
            user_account,
            ..
        } = self;

        if !user_id.is_activated {
            Err(CustomError::AccountIsNotActivated)?;
        }

        if data.len() > user_account.max_size as usize {
            Err(CustomError::MaxDataSizeIsExceeded)?;
        }

        if nonce == user_account.nonce {
            Err(CustomError::BadNonce)?;
        }

        user_account.data = data;
        user_account.nonce = nonce;

        Ok(())
    }
}
