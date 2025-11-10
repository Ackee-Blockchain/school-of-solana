use std::rc::Rc;
use std::{thread, time::Duration};
use anchor_client::Cluster;
use anchor_client::solana_sdk::{
    signature::Keypair,
    signer::Signer,
    pubkey::Pubkey,
};
#[allow(deprecated)]
use anchor_client::solana_sdk::system_program;
use anchor_lang::AccountDeserialize;


/// Integration test using Anchor Client:
///  - Connects to a local Solana validator
///  - Performs an airdrop for a fresh payer account
///  - Calls the `initialize` instruction of the program
///  - Fetches on-chain account data to verify correct state
#[test]
fn test_initialize_with_airdrop() {

    // Create a new random keypair and connect to the local Solana validator
    let payer = Rc::new(Keypair::new());
    let client = anchor_client::Client::new(Cluster::Localnet, payer.clone());
    
    // Access the deployed Anchor program by its ID
    let program = client.program(test_examples::id()).unwrap();
    
    // Get the RPC client from the program
    let rpc = program.rpc();

    // Airdrop 1 SOL to payer
    let sig = rpc
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .expect("Airdrop request failed");

    // Wait until the airdrop transaction is fully confirmed on-chain
    rpc.poll_for_signature(&sig).expect("Airdrop not finalized");

    // Derive PDA
    let (data_pda, _bump) =
        Pubkey::find_program_address(&[b"data1", b"data2", payer.pubkey().as_ref()], &program.id());

    // Build and send the `initialize` instruction
    program
        .request()
        .accounts(test_examples::accounts::Initialize {
            user: payer.pubkey(),
            data: data_pda,
            system_program: system_program::id(),
        })
        .args(test_examples::instruction::Initialize { count: 10 })
        .send()
        .unwrap();

    // Fetch on-chain account
    let acc: test_examples::MyData = program.account(data_pda).unwrap();

    // Verify that the logic ran correctly
    assert_eq!(acc.counter, 0);
}
