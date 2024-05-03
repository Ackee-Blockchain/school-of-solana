use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Po8QNbexAedzyagU3ZupR7pLeaqe4BmJmrNCaJ8xu6h");

#[program]
pub mod owner_checks_secure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        msg!("Your account balance is: {}", ctx.accounts.token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}
