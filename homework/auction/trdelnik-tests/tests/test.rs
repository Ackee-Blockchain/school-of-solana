use fehler::throws;
use program_client;
use trdelnik_client::{anyhow::Result, *};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    fixture.deploy().await?;
    
    fixture
}

#[trdelnik_test]
async fn test_happy_path(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
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
}
