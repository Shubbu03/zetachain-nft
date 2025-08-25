use crate::{error::ErrorCode, events::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};
use std::mem::size_of;

#[derive(Accounts)]
pub struct SendNftCrossChain<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"universal_nft"],
        bump
    )]
    pub pda: Account<'info, UniversalNftPda>,

    #[account(
        mut,
        seeds = [b"nft_record", nft_mint.key().as_ref()],
        bump
    )]
    pub nft_record: Account<'info, NftRecord>,

    #[account(
        init,
        payer = user,
        space = 8 + size_of::<OutboundTransfer>(),
        seeds = [b"outbound", nft_mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub outbound_transfer: Account<'info, OutboundTransfer>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: ZetaChain Gateway PDA
    #[account(mut)]
    pub gateway_pda: UncheckedAccount<'info>,

    /// CHECK: ZetaChain Gateway Program
    pub gateway_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SendNftCrossChain<'info> {
    pub fn send_nft_cross_chain(
        &mut self,
        destination_chain: u64,
        recipient: [u8; 20],
    ) -> Result<()> {
        require!(!self.pda.paused, ErrorCode::ProgramPaused);
        require!(!self.nft_record.locked, ErrorCode::NftAlreadyLocked);
        require_eq!(
            self.user_token_account.amount,
            1,
            ErrorCode::InsufficientBalance
        );

        self.nft_record.locked = true;

        let burn_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.nft_mint.to_account_info(),
                from: self.user_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        );
        burn(burn_ctx, 1)?;

        let cross_chain_msg = CrossChainMessage {
            action: CrossChainAction::Transfer,
            token_id: self.nft_record.token_id,
            mint: self.nft_mint.key(),
            original_chain: self.nft_record.original_chain,
            destination_chain,
            recipient,
            name: self.nft_record.name.clone(),
            description: self.nft_record.description.clone(),
            image: self.nft_record.image.clone(),
        };

        let message_data = cross_chain_msg.try_to_vec()?;

        self.outbound_transfer.set_inner(OutboundTransfer {
            message: cross_chain_msg.clone(),
            user: self.user.key(),
            timestamp: Clock::get()?.unix_timestamp,
            completed: false,
        });

        let gateway_cpi_accounts = gateway::cpi::accounts::Deposit {
            signer: self.user.to_account_info(),
            pda: self.gateway_pda.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.gateway_program.to_account_info(), gateway_cpi_accounts);

        gateway::cpi::deposit_and_call(cpi_ctx, 1000000, recipient, message_data, None)?;

        msg!(
            "NFT cross-chain transfer initiated: mint={}, token_id={}, dest_chain={}, recipient={:?}",
            self.nft_mint.key(),
            self.outbound_transfer.message.token_id,
            destination_chain,
            recipient
        );

        emit!(CrossChainTransferEvent {
            mint: self.nft_mint.key(),
            from_chain: self.pda.chain_id,
            to_chain: destination_chain,
            recipient,
            token_id: self.outbound_transfer.message.token_id,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}
