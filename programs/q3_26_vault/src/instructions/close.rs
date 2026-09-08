use crate::{
    constants::{STATE, VAULT_SEED},
    state::VaultState,
};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [STATE, user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> Close<'info> {
    pub fn close (&mut self) -> Result<()> {

        let cpi_program = self.system_program.key();

        let vault_signer_seeds = [
            VAULT_SEED,
            self.user.key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let seeds = [&vault_signer_seeds[..]];

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer( cpi_program, cpi_accounts, &seeds);

        transfer(cpi_ctx, self.vault.lamports())
    }
}