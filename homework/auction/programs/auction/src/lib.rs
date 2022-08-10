use anchor_lang::{
    prelude::*,
    solana_program::{clock::UnixTimestamp, program::invoke, system_instruction},
};

declare_id!("Poo5jhFcGjMjYaz2cpmSNVq4ehvjKJhjU7aCZiS2LMP");

#[program]
pub mod auction {
    use super::*;
    /// Creates and initialize a new state of our program
    pub fn initialize(ctx: Context<Initialize>, auction_duration: i64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        state.auction_end_time = Clock::get()?.unix_timestamp + auction_duration as UnixTimestamp;
        state.treasury = *ctx.accounts.treasury.key;
        state.initializer = *ctx.accounts.initializer.to_account_info().key;
        state.ended = false;
        state.highest_bid = 0;
        state.highest_bidder = Pubkey::default();

        Ok(())
    }
    /// Bid
    pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let bid = &mut ctx.accounts.bid;

        if Clock::get()?.unix_timestamp > state.auction_end_time {
            panic!("Auction Inactive")
        }

        let raised_by = amount.saturating_sub(bid.amount_locked);
        invoke(
            &system_instruction::transfer(
                ctx.accounts.bidder.key,
                ctx.accounts.treasury.key,
                raised_by,
            ),
            &[
                ctx.accounts.bidder.to_account_info().clone(),
                ctx.accounts.treasury.clone(),
            ],
        )?;

        bid.amount_locked = bid.amount_locked.checked_add(raised_by).unwrap();
        bid.bump = *ctx.bumps.get("bid").unwrap();

        if amount > state.highest_bid {
            state.highest_bid = amount;
            state.highest_bidder = *ctx.accounts.bidder.key;
        }

        Ok(())
    }
    /// After an auction ends (determined by `auction_duration`), a seller can claim the
    /// heighest bid by calling this instruction
    pub fn end_auction(ctx: Context<EndAuction>) -> Result<()> {
        let state = &mut ctx.accounts.state;

        if Clock::get()?.unix_timestamp < state.auction_end_time as UnixTimestamp {
            panic!("Auction Active");
        }

        **ctx.accounts.treasury.lamports.borrow_mut() -= state.highest_bid;
        **ctx.accounts.initializer.lamports.borrow_mut() += state.highest_bid;

        state.ended = true;

        Ok(())
    }
    /// After an auction ends (the initializer/seller already received the winning bid),
    /// the unsuccessfull bidders can claim their money back by calling this instruction
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        let amount = ctx.accounts.bid_account.amount_locked;

        **ctx.accounts.treasury.lamports.borrow_mut() -= amount;
        **ctx.accounts.bidder.lamports.borrow_mut() += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// State of our auction program (up to you)
    #[account(
        init,
        payer = initializer,
        space = 8 + 32 + 32 + 8 + 1 + 8 + 32,
    )]
    pub state: Account<'info, State>,
    /// Account which holds tokens bidded by biders
    #[account(
        init,
        payer = initializer,
        space = 0,
    )]
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    /// Seller
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(
        init_if_needed,
        payer = bidder,
        space = 8 + 8 + 1,
        seeds = [state.to_account_info().key.as_ref(), bidder.to_account_info().key.as_ref()],
        bump,
    )]
    pub bid: Account<'info, BidInfo>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut, has_one = treasury)]
    pub state: Account<'info, State>,
    /// CHECK: already checked by state constraint
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndAuction<'info> {
    #[account(
        mut,
        has_one = treasury,
        has_one = initializer,
        has_one = highest_bidder,
        constraint = state.ended == false
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// CHECK: already checked by state constraint
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub highest_bidder: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [state.to_account_info().key.as_ref(), highest_bidder.to_account_info().key.as_ref()],
        bump = winners_bid.bump,
        close = highest_bidder
    )]
    pub winners_bid: Account<'info, BidInfo>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(has_one = treasury, constraint = state.ended == true)]
    pub state: Account<'info, State>,
    #[account(
        mut,
        seeds = [state.to_account_info().key.as_ref(), bidder.to_account_info().key.as_ref()],
        bump = bid_account.bump,
        close = bidder)]
    pub bid_account: Account<'info, BidInfo>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    /// CHECK:
    pub treasury: AccountInfo<'info>,
}

#[account]
pub struct State {
    pub initializer: Pubkey,
    pub treasury: Pubkey,
    pub auction_end_time: i64,
    pub ended: bool,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
}

#[account]
pub struct BidInfo {
    pub amount_locked: u64,
    pub bump: u8,
}
