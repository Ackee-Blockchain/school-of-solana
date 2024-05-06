use anchor_lang::prelude::*; // Import necessary modules and items

// Define the BankAccount struct, representing a bank account
#[account]
pub struct BankAccount {
    pub name: String,  // Name of the bank account
    pub balance: u64,  // Balance of the bank account
    pub owner: Pubkey, // Owner of the bank account
}
