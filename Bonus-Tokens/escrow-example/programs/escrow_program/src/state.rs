use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub side_a: Pubkey,
    pub side_b: Pubkey,
    pub a_to_b_amount: u64,
    pub b_to_a_amount: u64,
    pub a_to_b_mint: Pubkey,
    pub b_to_a_mint: Pubkey,
    pub bump: u8,
    pub escrow_token_bump: u8,
}

impl Escrow {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 32 + 32 + 1 + 1;
}
