use crate::{error::BankError, BankAccount}; // Import the BankError enum and BankAccount struct
use anchor_lang::prelude::*; // Import necessary modules and items

// Define the withdraw function for withdrawing funds from a bank account
pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // Get mutable references to the bank account and user
    let bank = &mut ctx.accounts.bank;
    let user = &mut ctx.accounts.user;

    // Check if the owner of the bank account matches the user
    if bank.owner != user.key() {
        return Err(BankError::OwnerMismatch.into()); // Return an error if the owner does not match
    }

    // Get the minimum balance required for the bank account
    let rent = Rent::get()?.minimum_balance(bank.to_account_info().data_len());

    // Check if there are sufficient funds in the bank account
    if **bank.to_account_info().lamports.borrow() - rent < amount {
        return Err(BankError::InsufficientFunds.into()); // Return an error if there are insufficient funds
    }

    // Withdraw funds from the bank account and deposit into the user's account
    **bank.to_account_info().try_borrow_mut_lamports()? -= amount;
    **user.to_account_info().try_borrow_mut_lamports()? += amount;

    // Update the balance of the bank account
    bank.balance -= amount;

    // Return success
    Ok(())
}

// Define the Withdraw accounts structure
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)] // Mutable reference to the bank account
    pub bank: Account<'info, BankAccount>,
    #[account(mut)] // Mutable reference to the user's account
    pub user: Signer<'info>,
}
