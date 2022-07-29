use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct State {
    pub locked: bool,
}

impl State {
    pub const SERIALZED_SIZE: usize = 1;
}
