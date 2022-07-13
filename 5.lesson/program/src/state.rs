use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct State {
    pub locked: bool,
    pub payer: Pubkey,
}

impl State {
    pub const SERIALZED_SIZE: usize = 1 + 32;
}
