use anchor_lang::prelude::*;

use anchor_spl::{
    associated_tokento::AssociatedToken,
    token_interface::{close_account, TokenInterface, Mint, transfert_checked, TokenAccount, TransferChecked, CloseAccount}
}

use crate::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info. Mint>,
    #[account(
        mut,
        assoicated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"ecrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        assoicated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_progran: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
impl<'info>, Refund<'info> {
    // pub fn deposit(&mut self, deposit: u64) -> Result<()>{
    //     let tranfer_accounts = TransferChecked {
    //         from: self.taker_ata_b.to_account_info(),
    //         mint: self.mint_b.to_account_info(),
    //         to: self.maker_ata_b.to_account_info(),
    //         authority: self.taker.to_account_info(),
    //     };
    //     let cpi_ctx = CpiContext::new(self.token_progran.to_account_info(), tranfer_accounts);
    //     transfert_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
    // }
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        
        let signer_seed: [&[&[u8]], 1] = [&[,
        b"escrow",
        self.maker.to_account_info().key().as_ref(),
        &self.escrow.seed.to_le_bytes()(..),
        &[self.escrow.bump],
        ]],
       
        let tranfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        
        let tranfer_cpi_ctx = CpiContext::new_vith_signer(self.token_progran.to_account_info(), 
        tranfer_accounts,
        signer_seeds
        );
        transfert_checked(tranfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;
        
        let close_account = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info,
        };
       
        let close_cpi_ctx = CpiContext::new_vith_signer(self.token_progran.to_account_info(), 
        close_accounts,
        signer_seeds
        );
        close_account(close_cpi_ctx)

    }
}
