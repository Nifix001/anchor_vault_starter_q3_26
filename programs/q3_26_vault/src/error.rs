use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Deposit amount must be greater than zero")]
    InvalidAmount,
    
    #[msg("Vault does not have enough lamports to stay rent-exempt after withdrawal.")]
    ViolateRentExemption,
    
    #[msg("Vault does not have enough funds to fulfill this withdrawal.")]
    InsufficientFunds,

    #[msg("Arithmetic underflow occurred.")]
    Underflow,
}
