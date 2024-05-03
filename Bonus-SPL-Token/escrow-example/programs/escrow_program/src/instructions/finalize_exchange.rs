use crate::state::*;
use anchor_lang::prelude::*;

use anchor_spl::token::{close_account, transfer, CloseAccount, Token, TokenAccount, Transfer};

pub fn _finalize_exchange(ctx: Context<FinalizeExchange>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.side_b_send_token_account.to_account_info(),
                to: ctx.accounts.side_a_receive_token_account.to_account_info(),
                authority: ctx.accounts.side_b.to_account_info(),
            },
        ),
        escrow.b_to_a_amount,
    )?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.side_b_receive_token_account.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            &[&[
                escrow.side_a.as_ref(),
                escrow.side_b.as_ref(),
                escrow.a_to_b_mint.as_ref(),
                escrow.b_to_a_mint.as_ref(),
                escrow.a_to_b_amount.to_le_bytes().as_ref(),
                escrow.b_to_a_amount.to_le_bytes().as_ref(),
                &[ctx.accounts.escrow.bump],
            ]],
        ),
        escrow.a_to_b_amount,
    )?;

    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.escrow_token_account.to_account_info(),
            destination: ctx.accounts.side_a.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        },
        &[&[
            escrow.side_a.as_ref(),
            escrow.side_b.as_ref(),
            escrow.a_to_b_mint.as_ref(),
            escrow.b_to_a_mint.as_ref(),
            escrow.a_to_b_amount.to_le_bytes().as_ref(),
            escrow.b_to_a_amount.to_le_bytes().as_ref(),
            &[ctx.accounts.escrow.bump],
        ]],
    ))?;

    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeExchange<'info> {
    pub side_b: Signer<'info>,
    #[account(
        mut,
        close = side_a,
        seeds=[
            side_a.key().as_ref(),
            side_b.key().as_ref(),
            escrow.a_to_b_mint.key().as_ref(),
            escrow.b_to_a_mint.key().as_ref(),
            escrow.a_to_b_amount.to_le_bytes().as_ref(),
            escrow.b_to_a_amount.to_le_bytes().as_ref(),
        ],
        bump=escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    ///CHECK; this has to be here as close will return rent, and it is also
    /// part of the seed which will ensure that we have correct account( because,
    /// wrong side_a or wrong escrow will result in PDA mismatch)
    #[account(mut)]
    pub side_a: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::authority = escrow.side_a,
        associated_token::mint = escrow.b_to_a_mint,
    )]
    pub side_a_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority = side_b,
        associated_token::mint = escrow.a_to_b_mint,
    )]
    pub side_b_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority = side_b,
        associated_token::mint = escrow.b_to_a_mint,
    )]
    pub side_b_send_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        close=side_a,
        token::mint = escrow.a_to_b_mint,
        token::authority = escrow,
        seeds=[
            escrow.key().as_ref(),
        ],
        bump = escrow.escrow_token_bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
