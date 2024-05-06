use anchor_lang::prelude::*; // Import necessary modules and items

// Define an enum to represent errors related to the bank
#[error_code]
pub enum BankError {
    #[msg("Incorrect owner")] // Error message for OwnerMismatch variant
    OwnerMismatch, // Variant representing mismatched owner error

    #[msg("Not enough Funds")] // Error message for InsufficientFunds variant
    InsufficientFunds, // Variant representing insufficient funds error
}
