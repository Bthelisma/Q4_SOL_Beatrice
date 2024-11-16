use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, TokenInterface, Mint, transfert_checked, TokenAccount, TransferChecked, CloseAccount}
}

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>
    pub mint_a: InterfaceAccount<'info. Mint>,
    mub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker
        assoicated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_progran = token_progran,
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        assoicated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_progran = token_progran,
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        init_if_needed,
        payer = taker
        assoicated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_progran = token_progran,
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"ecrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        assoicated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_progran = token_progran,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_progran: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
impl<'info>, Take<'info> {
    pub fn deposit(&mut self, deposit: u64) -> Result<()>{
        let tranfer_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_progran.to_account_info(), tranfer_accounts);
        transfert_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
    }
    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        let signer_seed: [&[&[u8]], 1] = [&[,
        b"escrow",
        self.maker.to_account_info().key().as_ref(),
        &self.escrow.seed.to_le_bytes()(..),
        &[self.escrow.bump],
        ]],
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let ctx = CpiContext::new_vith_signer(self.token_progran.to_account_info(), 
        accounts,
        signer_seeds
        );
        transfert_checked(ctx, self.vault.amount, self.mint_a.decimals)?;
        let account = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info,
        };
        let ctx = CpiContext::new_vith_signer(self.token_progran.to_account_info(), 
        accounts,
        signer_seeds
        );
        close_account(ctx)

    }
}
