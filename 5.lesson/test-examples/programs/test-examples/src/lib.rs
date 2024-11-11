use anchor_lang::prelude::*;

declare_id!("62beeFZ1vxTPUSGQCB21w8orVcYdCePxa1L6eJrYWMYU");

#[program]
pub mod test_examples {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, count: u8) -> Result<()> {
        let data = &mut ctx.accounts.data;

        data.authority = ctx.accounts.user.key();
        require!(count <= 10, MyError::InvalidInstructionData);

        // Never panics due to require macro above.
        data.counter = math_function(count).unwrap();

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

#[error_code]
pub enum MyError {
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
}

fn math_function(count: u8) -> Option<u8> {
    10u8.checked_sub(count)
}

// Unit tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // It is possible to test even private functions.
    #[test]
    fn test_math_function() {
        assert_eq!(math_function(2), Some(8));
        assert_eq!(math_function(11), None);
    }
}
