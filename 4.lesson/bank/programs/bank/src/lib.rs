// Import necessary modules and items
use anchor_lang::prelude::*;
// Import modules from the crate
pub mod constants; // Module containing constants used in the program
pub mod error; // Module containing custom error types
pub mod instructions; // Module containing instruction handlers
pub mod state; // Module containing program state definition

// Re-export items from sub-modules for easier access
pub use constants::*; // Re-export constants
pub use instructions::*; // Re-export instruction handlers
pub use state::*; // Re-export program state

// Define the program ID for the bank program
declare_id!("DgdeWA1fTTvy8SNtYBookhNiJC5Q2K3TqSbPKcGDxmsn");

// Define the bank program module
#[program]
pub mod bank {
    use super::*;

    // Define the create instruction handler, which calls the _create function
    pub fn create(ctx: Context<Create>, name: String) -> Result<()> {
        _create(ctx, name) // Call the _create function
    }

    // Define the deposit instruction handler, which calls the _deposit function
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        _deposit(ctx, amount) // Call the _deposit function
    }

    // Define the withdraw instruction handler, which calls the _withdraw function
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        _withdraw(ctx, amount) // Call the _withdraw function
    }
}
