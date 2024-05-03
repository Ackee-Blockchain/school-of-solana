use crate::state::*;
use anchor_lang::prelude::*;

use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

pub fn _initialize_exchange(
    ctx: Context<InitializeExchange>,
    a_to_b_amount: u64,
    b_to_a_amount: u64,
    side_b: Pubkey,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    escrow.side_a = ctx.accounts.side_a.key();
    escrow.side_b = side_b;

    escrow.a_to_b_amount = a_to_b_amount;
    escrow.b_to_a_amount = b_to_a_amount;

    escrow.a_to_b_mint = ctx.accounts.a_to_b_mint.key();
    escrow.b_to_a_mint = ctx.accounts.b_to_a_mint.key();

    escrow.bump = ctx.bumps.escrow;
    escrow.escrow_token_bump = ctx.bumps.escrow_token_account;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.side_a_send_token_account.to_account_info(),
                to: ctx.accounts.escrow_token_account.to_account_info(),
                authority: ctx.accounts.side_a.to_account_info(),
            },
        ),
        a_to_b_amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(a_to_b_amount: u64,b_to_a_amount:u64,side_b:Pubkey)]
pub struct InitializeExchange<'info> {
    #[account(mut)]
    pub side_a: Signer<'info>,
    #[account(
        init,
        payer = side_a,
        space = 8 + Escrow::LEN,
        seeds = [
            side_a.key().as_ref(),
            side_b.key().as_ref(),
            a_to_b_mint.key().as_ref(),
            b_to_a_mint.key().as_ref(),
            a_to_b_amount.to_le_bytes().as_ref(),
            b_to_a_amount.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = a_to_b_mint,
        associated_token::authority = side_a
    )]
    pub side_a_send_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=side_a,
        token::mint = a_to_b_mint,
        token::authority = escrow,
        seeds = [
            escrow.key().as_ref()
        ],
        bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    pub a_to_b_mint: Account<'info, Mint>,
    pub b_to_a_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
