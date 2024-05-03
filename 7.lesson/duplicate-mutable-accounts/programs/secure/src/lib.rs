use anchor_lang::prelude::*;

declare_id!("Po8QNbexAedzyagU3ZupR7pLeaqe4BmJmrNCaJ8xu6h");

#[program]
pub mod duplicate_mutable_accounts_secure {
    use super::*;

    pub fn create_user_account(ctx: Context<CreateUser>) -> Result<()> {
        msg!("GM");
        ctx.accounts.user.balance = 10;
        
        Ok(())
    }

    pub fn close_user_a(ctx: Context<Close>) -> Result<()> {
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        let new_balance = user_a.balance + user_b.balance;
        
        // ...

        user_a.balance = 0;
        user_b.balance = new_balance;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + 8,
    )]
    user: Account<'info, User>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, constraint = user_a.key() != user_b.key())]
    user_a: Account<'info, User>,
    #[account(mut)]
    user_b: Account<'info, User>,
}

#[account]
pub struct User {
    pub balance: u64,
}
