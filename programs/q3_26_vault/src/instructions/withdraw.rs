use crate::{
    constants::{STATE, VAULT_SEED},
    error::ErrorCode,
    state::VaultState,
};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
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
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {

         // check that the user leaves the vault with a rent-exempt balance
        let vault_info = self.vault.to_account_info();

        // Get the rent-exempt minimum for the vault account
        let rent_exempt_balance = Rent::get()?.minimum_balance(vault_info.data_len());

        // Check vault has enough lamports to satisfy the rent-exempt requirement after withdrawal
        let remaining_balance = vault_info.lamports()
            .checked_sub(amount)
            .ok_or(ErrorCode::Underflow)?;

        require!(
            remaining_balance >= rent_exempt_balance,
            ErrorCode::ViolateRentExemption
        );

        // check vault has enough for this withdrawal minus rent exempt
        require!(
            vault_info.lamports() >= amount,
            ErrorCode::InsufficientFunds
        );
        require!(amount > 0, ErrorCode::InvalidAmount);

        let cpi_program = self.system_program.key();
        // let signer_seeds: [&[&[u8]]; 1] = [&[
        //     ESCROW_SEED,
        //     self.maker.key.as_ref(),
        //     &self.escrow.seed.to_le_bytes()[..],
        //     &[self.escrow.bump],
        // ]];
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

        transfer(cpi_ctx, amount)
    }
}
