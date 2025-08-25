use anchor_lang::prelude::*;

pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

pub use error::ErrorCode;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("8A3MwvuqnrggowLQuvPu7AjW5NgxrKYXe894mk86vXUh");

pub const GATEWAY_PROGRAM_ID: Pubkey = pubkey!("ZETAjseVjuFsxdRxo6MmTCvqFwb3ZHUx56Co3vCmGis");

#[program]
pub mod universal_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, chain_id: u64) -> Result<()> {
        ctx.accounts.initialize(chain_id)
    }

    pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        name: String,
        symbol: String,
        base_uri: String,
    ) -> Result<()> {
        ctx.accounts.initialize_collection(name, symbol, base_uri)
    }

    pub fn mint_nft(
        ctx: Context<MintNft>,
        name: String,
        description: String,
        image: String,
    ) -> Result<()> {
        ctx.accounts
            .mint_nft(ctx.bumps.pda, name, description, image)
    }

    pub fn send_nft_cross_chain(
        ctx: Context<SendNftCrossChain>,
        destination_chain: u64,
        recipient: [u8; 20],
    ) -> Result<()> {
        ctx.accounts
            .send_nft_cross_chain(destination_chain, recipient)
    }

    pub fn receive_nft_cross_chain(
        ctx: Context<ReceiveNftCrossChain>,
        cross_chain_msg: CrossChainMessage,
    ) -> Result<()> {
        ctx.accounts
            .receive_nft_cross_chain(ctx.bumps.pda, cross_chain_msg)
    }

    pub fn on_call(
        ctx: Context<OnCallComplete>,
        amount: u64,
        sender: [u8; 20],
        data: Vec<u8>,
    ) -> Result<()> {
        ctx.accounts.on_call(
            ctx.accounts.gateway_program.key(),
            ctx.bumps.pda,
            amount,
            sender,
            data,
        )
    }

    pub fn on_revert(
        ctx: Context<OnRevert>,
        amount: u64,
        sender: Pubkey,
        data: Vec<u8>,
    ) -> Result<()> {
        ctx.accounts.on_revert(ctx.bumps.pda, amount, sender, data)
    }

    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        ctx.accounts.set_paused(paused)
    }
}
