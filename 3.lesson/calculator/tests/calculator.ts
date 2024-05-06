import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { expect } from 'chai';

//Mocha works using predescribed it blocks
describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  //Referencing the program - Abstraction that allows us to call methods of our SOL program.
  const program = anchor.workspace.Calculator as Program<Calculator>;
  const programProvider = program.provider as anchor.AnchorProvider;

  //Generating a keypair for our Calculator account
  const calculatorPair = anchor.web3.Keypair.generate();

  //Generating a keypair for the Signer account
  const signerPair = anchor.web3.Keypair.generate();

  const text = "School Of Solana"

  //Creating a test block
  it("Creating Calculator Instance", async () => {
    //Airdrop SOL to the Signer he will pay Rent for the Calculator Account
    await airdrop(programProvider.connection, signerPair.publicKey);
    //Calling create instance - Set our calculator keypair as a signer
    await program.methods.create(text).accounts(
      {
        calculator: calculatorPair.publicKey,
        user: signerPair.publicKey,
      }
    ).signers([calculatorPair, signerPair]).rpc()

    //We fecth the account and read if the string is actually in the account
    const account = await program.account.calculator.fetch(calculatorPair.publicKey)
    expect(account.greeting).to.eql(text)
  });

  //Another test step - test out addition
  it('Addition', async () => {
    await program.methods.add(new anchor.BN(2), new anchor.BN(3))
      .accounts({
        calculator: calculatorPair.publicKey,
      })
      .rpc()
    const account = await program.account.calculator.fetch(calculatorPair.publicKey)
    expect(account.result).to.eql(new anchor.BN(5))
  })

});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}
