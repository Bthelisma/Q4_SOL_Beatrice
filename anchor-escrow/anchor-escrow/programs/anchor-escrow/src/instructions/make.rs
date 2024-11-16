use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenInterface, Mint, transfert_checked, TokenAccount, TransferChecked}
}

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info. Mint>,
    mub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        assoicated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds = [b"ecrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker,
        assoicated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_progran: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
impl<'info>, Make<'info> {
    pub fn init_escrow(&mut, self, seed: u64, bumps: &Makebumps) -> Result<()> {
        self.escrow.set.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump: bumps.escrow 
        });
        ok(());
    }
    pub fn deposit(&mut self, deposit: u64) -> Result<()>{
        let tranfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_progran.to_account_info(), tranfer_accounts);
        transfert_checked(cpi_ctx, deposit, self.mint_a.decimals)
    }
}
