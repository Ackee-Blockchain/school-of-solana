use fehler::throws;
use program_client::auction_instruction::*;
use std::{thread, time::Duration};
use trdelnik_client::{
    anyhow::Result,
    solana_sdk::{
        native_token::{lamports_to_sol, sol_to_lamports},
        system_program,
    },
    *,
};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    fixture.deploy().await?;

    fixture
        .client
        .airdrop(fixture.initializer.pubkey(), 5_000_000_000)
        .await?;

    fixture
        .client
        .airdrop(fixture.bidder1.pubkey(), 5_000_000_000)
        .await?;

    fixture
        .client
        .airdrop(fixture.bidder2.pubkey(), 5_000_000_000)
        .await?;

    fixture
        .client
        .airdrop(fixture.bidder3.pubkey(), 5_000_000_000)
        .await?;

    initialize(
        &fixture.client,
        10,
        fixture.state.pubkey(),
        fixture.treasury.pubkey(),
        fixture.initializer.pubkey(),
        system_program::id(),
        [
            fixture.initializer.clone(),
            fixture.state.clone(),
            fixture.treasury.clone(),
        ],
    )
    .await?;

    fixture
}

#[trdelnik_test]
async fn test_bidder1_wins(#[future] init_fixture: Result<Fixture>) {
    let mut fixture = init_fixture.await?;

    fixture.print_state().await?;
    // bid from bidder1
    bid(
        &fixture.client,
        sol_to_lamports(1.0),
        fixture.bid1,
        fixture.bidder1.pubkey(),
        fixture.state.pubkey(),
        fixture.treasury.pubkey(),
        system_program::id(),
        [fixture.bidder1.clone()],
    )
    .await?;
    println!("Bidder1 bids 1 SOL");

    // bid from bidder2
    bid(
        &fixture.client,
        sol_to_lamports(1.0),
        fixture.bid2,
        fixture.bidder2.pubkey(),
        fixture.state.pubkey(),
        fixture.treasury.pubkey(),
        system_program::id(),
        [fixture.bidder2.clone()],
    )
    .await?;
    println!("Bidder2 bids 1 SOL");

    // bid from bidder3
    bid(
        &fixture.client,
        sol_to_lamports(1.0),
        fixture.bid3,
        fixture.bidder3.pubkey(),
        fixture.state.pubkey(),
        fixture.treasury.pubkey(),
        system_program::id(),
        [fixture.bidder3.clone()],
    )
    .await?;
    println!("Bidder3 bids 1 SOL");

    // second bid from bidder1
    bid(
        &fixture.client,
        sol_to_lamports(2.0),
        fixture.bid1,
        fixture.bidder1.pubkey(),
        fixture.state.pubkey(),
        fixture.treasury.pubkey(),
        system_program::id(),
        [fixture.bidder1.clone()],
    )
    .await?;
    println!("Bidder1 bids 2 SOL");

    // wait till the auction ends
    thread::sleep(Duration::from_secs(10));

    fixture.print_state().await?;

    end_auction(
        &fixture.client,
        fixture.state.pubkey(),
        fixture.initializer.pubkey(),
        fixture.treasury.pubkey(),
        fixture.bidder1.pubkey(),
        fixture.bid1,
        Some(fixture.initializer.clone()),
    )
    .await?;
    println!("Initializer ends the auction!");

    refund(
        &fixture.client,
        fixture.state.pubkey(),
        fixture.bid2,
        fixture.bidder2.pubkey(),
        fixture.treasury.pubkey(),
        Some(fixture.bidder2.clone()),
    )
    .await?;
    println!("Bidder2 refunds the bid");

    refund(
        &fixture.client,
        fixture.state.pubkey(),
        fixture.bid3,
        fixture.bidder3.pubkey(),
        fixture.treasury.pubkey(),
        Some(fixture.bidder3.clone()),
    )
    .await?;
    println!("Bidder3 refunds the bid");

    fixture.print_state().await?;
}

#[trdelnik_test]
async fn test_initializer_ends_auction_early(#[future] init_fixture: Result<Fixture>) {
    let mut fixture = init_fixture.await?;

    fixture.print_state().await?;
    // bid from bidder1
    bid(
        &fixture.client,
        sol_to_lamports(1.0),
        fixture.bid1,
        fixture.bidder1.pubkey(),
        fixture.state.pubkey(),
        fixture.treasury.pubkey(),
        system_program::id(),
        [fixture.bidder1.clone()],
    )
    .await?;
    println!("Bidder1 bids 1 SOL");

    let res = end_auction(
        &fixture.client,
        fixture.state.pubkey(),
        fixture.initializer.pubkey(),
        fixture.treasury.pubkey(),
        fixture.bidder1.pubkey(),
        fixture.bid1,
        Some(fixture.initializer.clone()),
    )
    .await;

    match res {
        Ok(_) => {
            println!("Initializer try ends the auction!");
            panic!("Early end allowed");
        }
        Err(_) => (),
    }
}

#[trdelnik_test]
async fn test_early_refund(#[future] init_fixture: Result<Fixture>) {
    let mut fixture = init_fixture.await?;

    fixture.print_state().await?;
    // bid from bidder1
    bid(
        &fixture.client,
        sol_to_lamports(1.0),
        fixture.bid1,
        fixture.bidder1.pubkey(),
        fixture.state.pubkey(),
        fixture.treasury.pubkey(),
        system_program::id(),
        [fixture.bidder1.clone()],
    )
    .await?;
    println!("Bidder1 bids 1 SOL");

    let res = refund(
        &fixture.client,
        fixture.state.pubkey(),
        fixture.bid3,
        fixture.bidder3.pubkey(),
        fixture.treasury.pubkey(),
        Some(fixture.bidder3.clone()),
    )
    .await;

    match res {
        Ok(_) => {
            println!("Bidder3 refunds the bid");
            panic!("Early refund allowed");
        }
        Err(_) => (),
    }
}

struct Fixture {
    client: Client,
    state: Keypair,
    treasury: Keypair,
    initializer: Keypair,
    bidder1: Keypair,
    bid1: Pubkey,
    bidder2: Keypair,
    bid2: Pubkey,
    bidder3: Keypair,
    bid3: Pubkey,
}
impl Fixture {
    fn new() -> Self {
        let auction_program = program_keypair(0);

        let state = keypair(42);
        let bidder1 = keypair(21);
        let bidder2 = keypair(22);
        let bidder3 = keypair(23);

        let (bid1, _) = Pubkey::find_program_address(
            &[state.pubkey().as_ref(), bidder1.pubkey().as_ref()],
            &auction_program.pubkey(),
        );
        let (bid2, _) = Pubkey::find_program_address(
            &[state.pubkey().as_ref(), bidder2.pubkey().as_ref()],
            &auction_program.pubkey(),
        );
        let (bid3, _) = Pubkey::find_program_address(
            &[state.pubkey().as_ref(), bidder3.pubkey().as_ref()],
            &auction_program.pubkey(),
        );

        Fixture {
            client: Client::new(system_keypair(0)),
            state: keypair(42),
            treasury: keypair(99),
            initializer: keypair(32),
            bidder1,
            bid1,
            bidder2,
            bid2,
            bidder3,
            bid3,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&program_keypair(0), "auction")
            .await?;
    }

    #[throws]
    async fn print_state(&mut self) {
        println!("\n-------------STATE---------------");
        println!(
            "bidder1 balance: {:?}\nbidder2 balance: {:?}\nbidder3 balance: {:?}\ninitializer balance: {:?}\ntreasury balance: {:?}",
            lamports_to_sol(self.client.get_balance(self.bidder1.pubkey()).await?),
            lamports_to_sol(self.client.get_balance(self.bidder2.pubkey()).await?),
            lamports_to_sol(self.client.get_balance(self.bidder3.pubkey()).await?),
            lamports_to_sol(self.client.get_balance(self.initializer.pubkey()).await?),
            lamports_to_sol(self.client.get_balance(self.treasury.pubkey()).await?),
        );
        println!("---------------------------------\n");
    }
}
