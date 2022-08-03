use fehler::throws;
use program_client;
use trdelnik_client::{anyhow::Result, *};
use solana_sdk::system_program;
use duplicate_mutable_accounts_insecure::User;

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    fixture.deploy().await?;
    fixture
}

#[trdelnik_test]
async fn test_insecure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    program_client::duplicate_mutable_accounts_insecure_instruction::create_user_account(
        &fixture.client,
        fixture.user_a.pubkey(),
        fixture.client.payer().pubkey(),
        system_program::id(),
        [
            fixture.user_a.clone(),
        ]
    )
    .await?;    

    let user_a = fixture.client.account_data::<User>(fixture.user_a.pubkey()).await?;
    println!("user_a balance: {:?}", user_a.balance);

    program_client::duplicate_mutable_accounts_insecure_instruction::close_user_a(
        &fixture.client,
        fixture.user_a.pubkey(), 
        fixture.user_a.pubkey(),
        []
    )
    .await?;

    let user_a = fixture.client.account_data::<User>(fixture.user_a.pubkey()).await?;
    println!("user_a balance: {:?}", user_a.balance);
}

struct Fixture {
    client: Client,
    user_a: Keypair,
    _user_b: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            user_a: keypair(23),
            _user_b: keypair(24),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&program_keypair(0), "duplicate_mutable_accounts_insecure")
            .await?;
        self.client
            .deploy_by_name(&program_keypair(8), "duplicate_mutable_accounts_secure")
            .await?;
    }
}
