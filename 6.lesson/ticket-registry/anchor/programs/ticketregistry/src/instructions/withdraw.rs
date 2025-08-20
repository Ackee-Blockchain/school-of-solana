use anchor_lang::prelude::*;

use crate::state::{Event};

pub fn _withdraw(ctx: Context<WithdrawContext>, amount: u64) -> Result<()> {
    let event = &mut ctx.accounts.event;
    let event_organizer = &mut ctx.accounts.event_organizer;

    event.sub_lamports(amount)?;
    event_organizer.add_lamports(amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account(mut)]
    pub event_organizer: Signer<'info>,
    #[account(
        mut,
        has_one = event_organizer
    )]
    pub event: Account<'info, Event>,
} 