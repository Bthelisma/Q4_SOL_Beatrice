use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("9vEtaZya9ePodChCbc2LGu1Wf41uJqSCZfJUSKwTT6Ds");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.account.initialize(&ctx.bumps)
    }
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.account.deposit(amount)
    }
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.account.withdraw(amount)
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
        
        ctx.accounts.close()?;

    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + VaultState:: INIT_SPACE,
        seeds = [b"state", user.key(), as_ref()],
        bump
    )]

    pub state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", state.key(), as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"state", user.key(), as_ref()],
        bump = state.state_bump
    )]

    pub state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", state.key(), as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Tranfer{
        from: self.user.to_account_info(),
        to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        tranfer(cpi_ctx, amount)?;
        
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Tranfer{
        from: self.user.to_account_info(),
        to: self.vault.to_account_info()
        };

        let seeds = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        tranfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}
impl<'info> Close<'info> {
    fn close(&mut self) -> Result<()> {
            
        let cpi_program = self.system_program.to_account_info();
            
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let seeds: &[&[u8]; 3] = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
    
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts, 
            signer_seeds); //vault is PDA, system_program does not own must be new_with_signer and pass seeds
    
            transfer(cpi_ctx, self.vault.lamports())?;
    
            Ok(())
        }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8,
}

