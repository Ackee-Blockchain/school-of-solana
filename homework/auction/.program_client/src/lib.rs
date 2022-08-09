// DO NOT EDIT - automatically generated file
pub mod auction_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn initialize(
        client: &Client,
        i_auction_duration: i64,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                auction::instruction::Initialize {
                    auction_duration: i_auction_duration,
                },
                auction::accounts::Initialize {
                    state: a_state,
                    treasury: a_treasury,
                    initializer: a_initializer,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn initialize_ix(
        i_auction_duration: i64,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: auction::instruction::Initialize {
                auction_duration: i_auction_duration,
            }
            .data(),
            accounts: auction::accounts::Initialize {
                state: a_state,
                treasury: a_treasury,
                initializer: a_initializer,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn bid(
        client: &Client,
        i_amount: u64,
        a_bid: anchor_lang::solana_program::pubkey::Pubkey,
        a_bidder: anchor_lang::solana_program::pubkey::Pubkey,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                auction::instruction::Bid { amount: i_amount },
                auction::accounts::Bid {
                    bid: a_bid,
                    bidder: a_bidder,
                    state: a_state,
                    treasury: a_treasury,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn bid_ix(
        i_amount: u64,
        a_bid: anchor_lang::solana_program::pubkey::Pubkey,
        a_bidder: anchor_lang::solana_program::pubkey::Pubkey,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: auction::instruction::Bid { amount: i_amount }.data(),
            accounts: auction::accounts::Bid {
                bid: a_bid,
                bidder: a_bidder,
                state: a_state,
                treasury: a_treasury,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn end_auction(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_highest_bidder: anchor_lang::solana_program::pubkey::Pubkey,
        a_winners_bid: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                auction::instruction::EndAuction {},
                auction::accounts::EndAuction {
                    state: a_state,
                    initializer: a_initializer,
                    treasury: a_treasury,
                    highest_bidder: a_highest_bidder,
                    winners_bid: a_winners_bid,
                },
                signers,
            )
            .await?)
    }
    pub fn end_auction_ix(
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_highest_bidder: anchor_lang::solana_program::pubkey::Pubkey,
        a_winners_bid: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: auction::instruction::EndAuction {}.data(),
            accounts: auction::accounts::EndAuction {
                state: a_state,
                initializer: a_initializer,
                treasury: a_treasury,
                highest_bidder: a_highest_bidder,
                winners_bid: a_winners_bid,
            }
            .to_account_metas(None),
        }
    }
    pub async fn refund(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_bid_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_bidder: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                auction::instruction::Refund {},
                auction::accounts::Refund {
                    state: a_state,
                    bid_account: a_bid_account,
                    bidder: a_bidder,
                    treasury: a_treasury,
                },
                signers,
            )
            .await?)
    }
    pub fn refund_ix(
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_bid_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_bidder: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: auction::instruction::Refund {}.data(),
            accounts: auction::accounts::Refund {
                state: a_state,
                bid_account: a_bid_account,
                bidder: a_bidder,
                treasury: a_treasury,
            }
            .to_account_metas(None),
        }
    }
}
