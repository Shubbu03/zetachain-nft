use anchor_lang::prelude::*;

#[account]
pub struct NftRecord {
    pub mint: Pubkey,
    pub original_chain: u64,
    pub token_id: u64,
    pub locked: bool,
    pub name: String,
    pub description: String,
    pub image: String,
}
