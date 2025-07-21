use {crate::state::Collection, anchor_lang::prelude::*, base::helpers::get_space};

#[derive(Accounts)]
#[instruction(id: u8)]
pub struct CreateCollection<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = get_space(Collection::INIT_SPACE),
        seeds = [b"collection", admin.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection: Account<'info, Collection>,
}

impl<'info> CreateCollection<'info> {
    pub fn create_collection(&mut self, bump: u8, id: u8, metadata: String) -> Result<()> {
        let CreateCollection {
            admin, collection, ..
        } = self;

        collection.set_inner(Collection {
            bump,
            id,
            next_token_id: 0,
            creator: admin.key(),
            address: collection.key(),
            metadata,
        });

        Ok(())
    }
}
