use {
    crate::{
        state::{Bump, Config, Route, SEED_BUMP, SEED_CONFIG, SEED_ROUTE},
        types::RouteItem,
    },
    anchor_lang::prelude::*,
    base::{error::AuthError, helpers::get_space},
};

#[derive(Accounts)]
#[instruction(mint_first: Pubkey, mint_last: Pubkey)]
pub struct SaveRoute<'info> {
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
        init_if_needed,
        payer = sender,
        space = get_space(Route::INIT_SPACE),
        // sorting mints isn't required as we use only single direction routes
        seeds = [SEED_ROUTE.as_bytes(), &mint_first.to_bytes(), &mint_last.to_bytes()],
        bump
    )]
    pub route: Account<'info, Route>,
}

impl<'info> SaveRoute<'info> {
    pub fn save_route(
        &mut self,
        _mint_first: Pubkey,
        _mint_last: Pubkey,
        route: Vec<RouteItem>,
    ) -> Result<()> {
        if self.sender.key() != self.config.admin {
            Err(AuthError::Unauthorized)?;
        }

        self.route.set_inner(Route { value: route });

        Ok(())
    }
}
