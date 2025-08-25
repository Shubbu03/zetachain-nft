use anchor_lang::prelude::*;

#[event]
pub struct CrossChainTransferEvent {
    pub mint: Pubkey,
    pub from_chain: u64,
    pub to_chain: u64,
    pub recipient: [u8; 20],
    pub token_id: u64,
    pub timestamp: i64,
}
