use anchor_lang::prelude::*;

#[account]
pub struct UniversalNftPda {
    pub authority: Pubkey,
    pub chain_id: u64,
    pub nonce: u64,
    pub paused: bool,
}
