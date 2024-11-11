use anchor_lang::prelude::*;

declare_id!("F8vgyLcvK2PP1WcjBF2swnQYzHZRt6CyVwtwCcZrbiXq");

#[program]
pub mod program_failed_to_complete {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, count: u8) -> Result<()> {
        let data = &mut ctx.accounts.data;

        // Prevent underflow by checking that `count` does not exceed 10.
        // require!(count <= 10, MyError::InvalidInstructionData);

        data.authority = ctx.accounts.user.key();
        data.counter = 10 - count;

        msg!("Data.counter = {}", data.counter);
        msg!("Data pubkey = {}", data.key().to_string());
        msg!("User pubkey = {}", data.authority.key().to_string());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(init,
        space = 8 + 32 + 1,
        payer = user,
        seeds = [b"data1", b"data2"],
        bump
    )]
    data: Account<'info, MyData>,

    system_program: Program<'info, System>,
}

#[account]
pub struct MyData {
    authority: Pubkey,
    counter: u8,
}

// Add custom error for clearer error messages.
// #[error_code]
// pub enum MyError {
//     #[msg("Invalid instruction data")]
//     InvalidInstructionData,
// }
