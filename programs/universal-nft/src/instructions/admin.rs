use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetPaused<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"universal_nft"],
        bump
    )]
    pub pda: Account<'info, UniversalNftPda>,
}

impl<'info> SetPaused<'info> {
    pub fn set_paused(&mut self, paused: bool) -> Result<()> {
        require_keys_eq!(
            self.authority.key(),
            self.pda.authority,
            ErrorCode::UnauthorizedAccess
        );

        self.pda.paused = paused;
        msg!("Program paused status set to: {}", paused);
        Ok(())
    }
}
