use anchor_lang::prelude::*;

#[account]
pub struct OutboundTransfer {
    pub message: CrossChainMessage,
    pub user: Pubkey,
    pub timestamp: i64,
    pub completed: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CrossChainMessage {
    pub action: CrossChainAction,
    pub token_id: u64,
    pub mint: Pubkey,
    pub original_chain: u64,
    pub destination_chain: u64,
    pub recipient: [u8; 20],
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum CrossChainAction {
    Transfer,
}
