use anchor_lang::prelude::*;

pub const MAX_NAME_LEN: usize = 30;
pub const MAX_DESCRIPTION_LEN: usize = 300;

#[account]
#[derive(InitSpace)]
pub struct Event {
    #[max_len(30)]
    pub name: String,
    #[max_len(300)]
    pub description: String,
    pub ticket_price: u64,
    pub available_tickets: u64,
    pub event_organizer: Pubkey,
    pub start_date: i64
}

#[account]
#[derive(InitSpace)]
pub struct Ticket {
    pub event: Pubkey,
    pub buyer: Pubkey,
    pub price: u64
}