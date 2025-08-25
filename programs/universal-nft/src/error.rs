use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Program is currently paused")]
    ProgramPaused,
    #[msg("NFT is already locked for cross-chain transfer")]
    NftAlreadyLocked,
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    #[msg("Invalid caller - must be gateway program")]
    InvalidCaller,
    #[msg("Invalid cross-chain message format")]
    InvalidMessage,
    #[msg("Insufficient token balance")]
    InsufficientBalance,
}
