import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bank } from "../target/types/bank"; // Importing the Bank interface
import { expect } from 'chai'; // Importing the Chai assertion library

// Define a seed for generating bank addresses
const BANK_ACCOUNT_SEED = "bank_account";

// Describe a test suite for the bank functionality
describe("bank", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Get the program from the workspace
  const program = anchor.workspace.Bank as Program<Bank>;

  // Generate a keypair for the Signer account
  const bankCreator = anchor.web3.Keypair.generate();

  // Define the name of the bank
  const bankName = "School of Solana Bank";

  // Test case: Create Bank
  it("Create Bank", async () => {
    // Airdrop tokens to the creator
    await airdrop(program.provider.connection, bankCreator.publicKey);

    // Get the address and bump for the bank
    const [bank_publickey, bank_bump] = getBankAddress(bankCreator.publicKey, program.programId);

    // Call the create method of the program
    await program.methods.create(bankName).accounts({
      user: bankCreator.publicKey,
      bank: bank_publickey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([bankCreator]).rpc();

    // Fetch the bank account data
    let bankAccountData = await program.account.bankAccount.fetch(bank_publickey);
    // Assert the properties of the bank account
    expect(bankAccountData.name).to.eql(bankName)
    expect(bankAccountData.owner).to.eql(bankCreator.publicKey)
  });

  // Test case: Deposit to Bank
  it("Deposit to Bank", async () => {
    // Get the bank address
    const [bank_publickey, bank_bump] = getBankAddress(bankCreator.publicKey, program.programId);

    // Define the deposit amount
    let depositAmount = new anchor.BN(500);
    // Call the deposit method of the program
    await program.methods.deposit(depositAmount).accounts({
      user: bankCreator.publicKey,
      bank: bank_publickey,
    }).signers([bankCreator]).rpc();

    // Fetch the bank account data
    let bankAccountData = await program.account.bankAccount.fetch(bank_publickey);
    // Assert the updated balance
    expect(bankAccountData.balance.toNumber()).to.eql(depositAmount.toNumber())
  });

  // Test case: Withdraw from Bank
  it("Withdraw from Bank", async () => {
    // Get the bank address
    const [bank_publickey, bank_bump] = getBankAddress(bankCreator.publicKey, program.programId);

    // Fetch the bank account data before withdrawal
    let bankAccountDataBefore = await program.account.bankAccount.fetch(bank_publickey);

    // Define the withdraw amount
    let withdrawAmount = new anchor.BN(258);
    // Call the withdraw method of the program
    await program.methods.withdraw(withdrawAmount).accounts({
      user: bankCreator.publicKey,
      bank: bank_publickey,
    }).signers([bankCreator]).rpc();

    // Fetch the bank account data after withdrawal
    let bankAccountDataAfter = await program.account.bankAccount.fetch(bank_publickey);
    // Assert the updated balance after withdrawal
    expect(bankAccountDataBefore.balance.toNumber() - withdrawAmount.toNumber()).to.eql(bankAccountDataAfter.balance.toNumber())
  });
});

// Function to airdrop tokens to an address
async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}

// Function to get the bank address
function getBankAddress(bankCreator: anchor.web3.PublicKey, programID: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(BANK_ACCOUNT_SEED),
      bankCreator.toBuffer()
    ], programID);
}
