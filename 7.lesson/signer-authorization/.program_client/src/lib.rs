// DO NOT EDIT - automatically generated file
pub mod signer_authorization_insecure_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn create_state(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                signer_authorization_insecure::instruction::CreateState {},
                signer_authorization_insecure::accounts::CreateState {
                    state: a_state,
                    authority: a_authority,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_state_ix(
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: signer_authorization_insecure::instruction::CreateState {}.data(),
            accounts: signer_authorization_insecure::accounts::CreateState {
                state: a_state,
                authority: a_authority,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn log_message(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                signer_authorization_insecure::instruction::LogMessage {},
                signer_authorization_insecure::accounts::LogMessage {
                    state: a_state,
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: signer_authorization_insecure::instruction::LogMessage {}.data(),
            accounts: signer_authorization_insecure::accounts::LogMessage {
                state: a_state,
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
pub mod signer_authorization_secure_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 239u8, 58u8, 114u8, 151u8, 218u8, 23u8, 115u8, 211u8, 109u8, 171u8, 244u8,
        210u8, 156u8, 47u8, 250u8, 168u8, 157u8, 98u8, 83u8, 7u8, 236u8, 150u8, 238u8, 226u8,
        191u8, 69u8, 100u8, 128u8, 159u8, 2u8,
    ]);
    pub async fn create_state(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                signer_authorization_secure::instruction::CreateState {},
                signer_authorization_secure::accounts::CreateState {
                    state: a_state,
                    authority: a_authority,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_state_ix(
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: signer_authorization_secure::instruction::CreateState {}.data(),
            accounts: signer_authorization_secure::accounts::CreateState {
                state: a_state,
                authority: a_authority,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn log_message(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                signer_authorization_secure::instruction::LogMessage {},
                signer_authorization_secure::accounts::LogMessage {
                    state: a_state,
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: signer_authorization_secure::instruction::LogMessage {}.data(),
            accounts: signer_authorization_secure::accounts::LogMessage {
                state: a_state,
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
