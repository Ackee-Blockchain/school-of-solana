use anchor_lang::prelude::*;

declare_id!("BDS42gR1XMtdDGPJqUkCnsNvSRvrte1onM9y8JHQbcgJ");

// Defines the `game` module containing the business logic.
#[program]
pub mod game {
    use super::*;
    // Function to create user statistics and initialize an account.
    pub fn create_user_stats(ctx: Context<CreateUserStats>, name: String) -> Result<()> {
        // Access and modify the user_stats account.
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.level = 0; // Set initial level to 0.
        if name.as_bytes().len() > 200 {
            // If the name exceeds 200 bytes, terminate with an error.
            panic!(); // Error handling simplified for brevity.
        }
        user_stats.name = name; // Store the name in the user_stats account.
        user_stats.bump = ctx.bumps.user_stats; // Safely store the bump used for this account.
        Ok(())
    }
    // handler function (add this next to the create_user_stats function in the game module)
    pub fn change_user_name(ctx: Context<ChangeUserName>, new_name: String) -> Result<()> {
        if new_name.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        ctx.accounts.user_stats.name = new_name;
        Ok(())
    }
}

// Data structure to store user statistics.
#[account]
pub struct UserStats {
    level: u16,   // User's level, stored as a 16-bit unsigned integer.
    name: String, // User's name, stored as a dynamically sized string.
    bump: u8,     // Bump seed for address derivation.
}

// Accounts context structure for creating user statistics.
#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // Signer who pays for the account creation and processing.
    // Initializes a user_stats account with predefined space allocation.
    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1, // Account size calculation includes all fields.
        seeds = [b"user-stats", user.key().as_ref()], // Seeds for deterministic address.
        bump
    )]
    pub user_stats: Account<'info, UserStats>, // The user_stats account to be created.
    pub system_program: Program<'info, System>, // System program to handle account creation.
}

// Defines the accounts context for the `change_user_name` function in the smart contract.
#[derive(Accounts)]
pub struct ChangeUserName<'info> {
    // Signer of the transaction, who is assumed to be the user making the change.
    pub user: Signer<'info>,
    // The `user_stats` account is required to be mutable because its data will be updated.
    // This account is looked up using seeds comprising a static "user-stats" seed and the user's public key,
    // ensuring that the account is unique per user.
    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, UserStats>,
}
