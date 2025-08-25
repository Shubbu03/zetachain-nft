use crate::state::*;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
pub struct InitializeCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + size_of::<Collection>(),
        seeds = [b"collection"],
        bump
    )]
    pub collection: Account<'info, Collection>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeCollection<'info> {
    pub fn initialize_collection(
        &mut self,
        name: String,
        symbol: String,
        base_uri: String,
    ) -> Result<()> {
        self.collection.set_inner(Collection {
            authority: self.authority.key(),
            name: name.clone(),
            symbol: symbol.clone(),
            base_uri,
            total_supply: 0,
        });

        msg!("Collection initialized: {} ({})", name, symbol);
        Ok(())
    }
}
