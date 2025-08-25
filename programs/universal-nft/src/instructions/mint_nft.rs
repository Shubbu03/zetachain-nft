use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"universal_nft"],
        bump
    )]
    pub pda: Account<'info, UniversalNftPda>,

    #[account(
        mut,
        seeds = [b"collection"],
        bump
    )]
    pub collection: Account<'info, Collection>,

    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<NftRecord>(),
        seeds = [b"nft_record", nft_mint.key().as_ref()],
        bump
    )]
    pub nft_record: Account<'info, NftRecord>,

    #[account(
        init,
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

impl<'info> MintNft<'info> {
    pub fn mint_nft(
        &mut self,
        pda_bump: u8,
        name: String,
        description: String,
        image: String,
    ) -> Result<()> {
        require!(!self.pda.paused, ErrorCode::ProgramPaused);

        self.nft_record.set_inner(NftRecord {
            mint: self.nft_mint.key(),
            original_chain: self.pda.chain_id,
            token_id: self.collection.total_supply,
            locked: false,
            name: name.clone(),
            description,
            image,
        });

        self.collection.total_supply += 1;

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
            "NFT minted: {} (token_id: {})",
            self.nft_mint.key(),
            self.nft_record.token_id
        );
        Ok(())
    }
}
