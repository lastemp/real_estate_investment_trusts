use anchor_lang::prelude::*;

#[error_code]
pub enum RealEstateInvestmentTrustsError {
    // investment trusts
    #[msg("Invalid issuer length")]
    InvalidIssuerLength,
    #[msg("Invalid name length")]
    InvalidNameLength,
    #[msg("Invalid type of reit")]
    InvalidTypeOfReit,
    #[msg("Invalid listing date length")]
    InvalidListingDateLength,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Invalid numeric value.")]
    InvalidNumeric,

    //
    #[msg("Invalid country length")]
    InvalidCountryLength,

    // Arithmetic
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,

    // investor
    #[msg("Invalid full names length")]
    InvalidFullNamesLength,
    #[msg("Investor has no active status.")]
    InvalidInvestorStatus,
    #[msg("Insufficient funds.")]
    InsufficientFunds,

    // account
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
