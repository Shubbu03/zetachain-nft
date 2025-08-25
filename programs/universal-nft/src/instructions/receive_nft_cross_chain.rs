use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct ReceiveNftCrossChain<'info> {
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

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ReceiveNftCrossChain<'info> {
    pub fn receive_nft_cross_chain(
        &mut self,
        pda_bump: u8,
        cross_chain_msg: CrossChainMessage,
    ) -> Result<()> {
        require!(!self.pda.paused, ErrorCode::ProgramPaused);

        match cross_chain_msg.action {
            CrossChainAction::Transfer => {
                self.handle_incoming_nft_transfer(pda_bump, cross_chain_msg)?;
            }
        }

        Ok(())
    }

    fn handle_incoming_nft_transfer(
        &mut self,
        pda_bump: u8,
        cross_chain_msg: CrossChainMessage,
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

        Ok(())
    }
}
