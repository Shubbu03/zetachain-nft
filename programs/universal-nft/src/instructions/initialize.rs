use crate::state::*;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + size_of::<UniversalNftPda>(),
        seeds = [b"universal_nft"],
        bump
    )]
    pub pda: Account<'info, UniversalNftPda>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, chain_id: u64) -> Result<()> {
        self.pda.set_inner(UniversalNftPda {
            authority: self.authority.key(),
            chain_id,
            nonce: 0,
            paused: false,
        });

        msg!("Universal NFT program initialized for chain: {}", chain_id);
        Ok(())
    }
}
