use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Po8QNbexAedzyagU3ZupR7pLeaqe4BmJmrNCaJ8xu6h");

#[program]
pub mod account_data_matching_secure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        msg!("Your acocunt balance is: {}", ctx.accounts.token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    // Instead, do this—Anchor checks that a token account contains valid data,
    // and that its owner is the signer of transaction.
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}
