use crate::{error::ErrorCode, state::*, GATEWAY_PROGRAM_ID};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct OnCall<'info> {
    #[account(
        mut,
        seeds = [b"universal_nft"],
        bump
    )]
    pub pda: Account<'info, UniversalNftPda>,

    /// CHECK: Gateway program for verification
    pub gateway_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnCallComplete<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"universal_nft"],
        bump
    )]
    pub pda: Account<'info, UniversalNftPda>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + size_of::<NftRecord>(),
        seeds = [b"nft_record", nft_mint.key().as_ref()],
        bump
    )]
    pub nft_record: Account<'info, NftRecord>,

    #[account(
        init_if_needed,
        payer = payer,
        mint::decimals = 0,
        mint::authority = pda,
        mint::freeze_authority = pda,
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = nft_mint,
        associated_token::authority = recipient_authority,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// CHECK: The recipient of the NFT
    pub recipient_authority: UncheckedAccount<'info>,

    /// CHECK: Gateway program for verification
    pub gateway_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnRevert<'info> {
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
        mut,
        seeds = [b"outbound", nft_mint.key().as_ref(), user_authority.key().as_ref()],
        bump
    )]
    pub outbound_transfer: Account<'info, OutboundTransfer>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user_authority,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: The original owner
    pub user_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> OnCallComplete<'info> {
    pub fn on_call(
        &mut self,
        gateway_program_key: Pubkey,
        pda_bump: u8,
        amount: u64,
        sender: [u8; 20],
        data: Vec<u8>,
    ) -> Result<()> {
        require!(
            gateway_program_key == GATEWAY_PROGRAM_ID,
            ErrorCode::InvalidCaller
        );

        let cross_chain_msg: CrossChainMessage =
            CrossChainMessage::try_from_slice(&data).map_err(|_| ErrorCode::InvalidMessage)?;

        let token_id = cross_chain_msg.token_id;

        match cross_chain_msg.action {
            CrossChainAction::Transfer => {
                self.handle_incoming_nft_transfer_from_gateway(
                    pda_bump,
                    cross_chain_msg,
                    sender,
                    amount,
                )?;
            }
        }

        msg!(
            "Cross-chain NFT transfer completed for token_id: {}",
            token_id
        );
        Ok(())
    }

    fn handle_incoming_nft_transfer_from_gateway(
        &mut self,
        pda_bump: u8,
        cross_chain_msg: CrossChainMessage,
        sender: [u8; 20],
        amount: u64,
    ) -> Result<()> {
        if cross_chain_msg.original_chain == self.pda.chain_id {
            self.nft_record.locked = false;
            msg!("NFT returned to original chain: {}", cross_chain_msg.mint);
        } else {
            self.nft_record.set_inner(NftRecord {
                mint: self.nft_mint.key(),
                original_chain: cross_chain_msg.original_chain,
                token_id: cross_chain_msg.token_id,
                locked: false,
                name: format!("Wrapped {}", cross_chain_msg.name),
                description: cross_chain_msg.description,
                image: cross_chain_msg.image,
            });
            msg!("Wrapped NFT created: {}", self.nft_mint.key());
        }

        let seeds = &[b"universal_nft".as_ref(), &[pda_bump]];
        let signer_seeds = &[&seeds[..]];

        let mint_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.nft_mint.to_account_info(),
                to: self.recipient_token_account.to_account_info(),
                authority: self.pda.to_account_info(),
            },
            signer_seeds,
        );
        mint_to(mint_ctx, 1)?;

        msg!(
            "NFT minted via gateway: {} with amount {} from sender {:?}",
            self.nft_mint.key(),
            amount,
            sender
        );

        Ok(())
    }
}

impl<'info> OnRevert<'info> {
    pub fn on_revert(
        &mut self,
        pda_bump: u8,
        _amount: u64,
        _sender: Pubkey,
        data: Vec<u8>,
    ) -> Result<()> {
        let cross_chain_msg: CrossChainMessage =
            CrossChainMessage::try_from_slice(&data).map_err(|_| ErrorCode::InvalidMessage)?;

        self.nft_record.locked = false;
        self.outbound_transfer.completed = true;

        let seeds = &[b"universal_nft".as_ref(), &[pda_bump]];
        let signer_seeds = &[&seeds[..]];

        let mint_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.nft_mint.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.pda.to_account_info(),
            },
            signer_seeds,
        );
        mint_to(mint_ctx, 1)?;

        msg!(
            "Cross-chain transfer reverted for token_id: {}",
            cross_chain_msg.token_id
        );
        Ok(())
    }
}
