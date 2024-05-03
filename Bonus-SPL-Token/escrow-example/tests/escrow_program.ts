import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowProgram } from "../target/types/escrow_program";
import { assert } from "chai";

import * as token from '@solana/spl-token';


describe("escrow_program", async () => {

  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  anchor.setProvider(provider);

  const program = anchor.workspace.EscrowProgram as Program<EscrowProgram>;


  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();


  const tom = anchor.web3.Keypair.generate();

  const a_to_b_mint_authority = anchor.web3.Keypair.generate();
  const b_to_a_mint_authority = anchor.web3.Keypair.generate();


  const a_to_c_mint_authority = anchor.web3.Keypair.generate();
  const c_to_a_mint_authority = anchor.web3.Keypair.generate();


  let a_to_b_mint;
  let b_to_a_mint;
  let a_to_c_mint;
  let c_to_a_mint;


  const a_to_b_amount = new anchor.BN(15);
  const b_to_a_amount = new anchor.BN(38);
  const a_to_c_amount = new anchor.BN(55);
  const c_to_a_amount = new anchor.BN(13);

  const to_mint = new anchor.BN(100)

  let alice_send_ata;
  let alice_receive_ata;
  let bob_send_ata;
  let bob_receive_ata;

  let alice_send_ata2;
  let alice_receive_ata2;

  describe("Setup", async () => {
    it("Setup mints and token accs", async () => {
      await airdrop(provider.connection, a_to_b_mint_authority.publicKey);
      await airdrop(provider.connection, b_to_a_mint_authority.publicKey);
      await airdrop(provider.connection, a_to_c_mint_authority.publicKey);
      await airdrop(provider.connection, c_to_a_mint_authority.publicKey);

      await airdrop(provider.connection, alice.publicKey);
      await airdrop(provider.connection, bob.publicKey);
      await airdrop(provider.connection, tom.publicKey);


      // Euroes
      a_to_b_mint = await token.createMint(
        provider.connection,
        a_to_b_mint_authority,
        a_to_b_mint_authority.publicKey,
        null,
        9
      );

      // Dollars
      b_to_a_mint = await token.createMint(
        provider.connection,
        b_to_a_mint_authority,
        b_to_a_mint_authority.publicKey,
        null,
        6
      );

      a_to_c_mint = await token.createMint(
        provider.connection,
        a_to_c_mint_authority,
        a_to_c_mint_authority.publicKey,
        null,
        9
      );

      c_to_a_mint = await token.createMint(
        provider.connection,
        c_to_a_mint_authority,
        c_to_a_mint_authority.publicKey,
        null,
        9
      );

      alice_send_ata = await token.createAccount(provider.connection, alice, a_to_b_mint, alice.publicKey);
      alice_receive_ata = await token.createAccount(provider.connection, alice, b_to_a_mint, alice.publicKey);

      alice_send_ata2 = await token.createAccount(provider.connection, alice, a_to_c_mint, alice.publicKey);
      alice_receive_ata2 = await token.createAccount(provider.connection, alice, c_to_a_mint, alice.publicKey);

      bob_send_ata = await token.createAccount(provider.connection, bob, b_to_a_mint, bob.publicKey);
      bob_receive_ata = await token.createAccount(provider.connection, bob, a_to_b_mint, bob.publicKey);

      await token.mintTo(provider.connection, bob, b_to_a_mint, bob_send_ata, b_to_a_mint_authority, to_mint.toNumber());
      await token.mintTo(provider.connection, alice, a_to_b_mint, alice_send_ata, a_to_b_mint_authority, to_mint.toNumber());
      await token.mintTo(provider.connection, alice, a_to_c_mint, alice_send_ata2, a_to_c_mint_authority, to_mint.toNumber());

    });
  });

  describe("Initialize Exchange", () => {
    it("Alice Initializes Exchange to Bob", async () => {

      const [escrow_pkey, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          alice.publicKey.toBuffer(),
          bob.publicKey.toBuffer(),
          a_to_b_mint.toBuffer(),
          b_to_a_mint.toBuffer(),
          a_to_b_amount.toArrayLike(Buffer, "le", 8),
          b_to_a_amount.toArrayLike(Buffer, "le", 8),
        ], program.programId);

      const [escrow_token_pkey, escrow_token_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          escrow_pkey.toBuffer(),
        ], program.programId);

      const alice_sendATABefore = await token.getAccount(provider.connection, alice_send_ata, "confirmed");
      //console.log("Side A Balance before Initialization: ", alice_sendATABefore.amount.toString())

      await program.methods.initializeExchange(a_to_b_amount, b_to_a_amount, bob.publicKey).accounts({
        sideA: alice.publicKey,
        escrow: escrow_pkey,
        sideASendTokenAccount: alice_send_ata,
        escrowTokenAccount: escrow_token_pkey,
        aToBMint: a_to_b_mint,
        bToAMint: b_to_a_mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      }).signers([alice]).rpc({ commitment: "confirmed" })

      const EscrowData = await program.account.escrow.fetch(escrow_pkey);
      const EscrowTokenAccount = await token.getAccount(provider.connection, escrow_token_pkey, "confirmed");
      const alice_sendATAAfter = await token.getAccount(provider.connection, alice_send_ata, "confirmed");

      console.log("Escrow Account Data:")
      console.log("sideA: ", EscrowData.sideA.toString())
      console.log("sideB: ", EscrowData.sideB.toString())
      console.log("A to B amount: ", EscrowData.aToBAmount.toString())
      console.log("B to A amount: ", EscrowData.bToAAmount.toString())

      console.log("Escrow Token Account balance after Initialization: ", EscrowTokenAccount.amount.toString())
      console.log("Side A Balance after Initialization: ", alice_sendATAAfter.amount.toString())


      assert.strictEqual(EscrowData.sideA.toString(), alice.publicKey.toString());
      assert.strictEqual(EscrowData.sideB.toString(), bob.publicKey.toString());
      assert.strictEqual(EscrowData.aToBAmount.toString(), a_to_b_amount.toString());
      assert.strictEqual(EscrowData.bToAAmount.toString(), b_to_a_amount.toString());
      assert.strictEqual(EscrowData.aToBMint.toString(), a_to_b_mint.toString());
      assert.strictEqual(EscrowData.bToAMint.toString(), b_to_a_mint.toString());

      assert.strictEqual(EscrowTokenAccount.mint.toString(), a_to_b_mint.toString());
      assert.strictEqual(EscrowTokenAccount.owner.toString(), escrow_pkey.toString());
      assert.strictEqual(EscrowTokenAccount.amount.toString(), a_to_b_amount.toString());

      assert.strictEqual(alice_sendATAAfter.mint.toString(), a_to_b_mint.toString());
      assert.strictEqual(alice_sendATAAfter.owner.toString(), alice.publicKey.toString());
      assert.strictEqual(alice_sendATAAfter.amount.toString(), to_mint.sub(a_to_b_amount).toString());


    });
    it("Alice Initializes Exchange to Tom", async () => {
      const [escrow_pkey, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          alice.publicKey.toBuffer(),
          tom.publicKey.toBuffer(),
          a_to_c_mint.toBuffer(),
          c_to_a_mint.toBuffer(),
          a_to_c_amount.toArrayLike(Buffer, "le", 8),
          c_to_a_amount.toArrayLike(Buffer, "le", 8),
        ], program.programId);


      const [escrow_token_pkey, escrow_token_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          escrow_pkey.toBuffer(),
        ], program.programId);


      await program.methods.initializeExchange(a_to_c_amount, c_to_a_amount, tom.publicKey).accounts({
        sideA: alice.publicKey,
        escrow: escrow_pkey,
        sideASendTokenAccount: alice_send_ata2,
        escrowTokenAccount: escrow_token_pkey,
        aToBMint: a_to_c_mint,
        bToAMint: c_to_a_mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      }).signers([alice]).rpc({ commitment: "confirmed" })


      const EscrowData = await program.account.escrow.fetch(escrow_pkey);

      assert.strictEqual(EscrowData.sideA.toString(), alice.publicKey.toString());
      assert.strictEqual(EscrowData.sideB.toString(), tom.publicKey.toString());
      assert.strictEqual(EscrowData.aToBAmount.toString(), a_to_c_amount.toString());
      assert.strictEqual(EscrowData.bToAAmount.toString(), c_to_a_amount.toString());
      assert.strictEqual(EscrowData.aToBMint.toString(), a_to_c_mint.toString());
      assert.strictEqual(EscrowData.bToAMint.toString(), c_to_a_mint.toString());

      const EscrowTokenAccount = await token.getAccount(provider.connection, escrow_token_pkey, "confirmed");
      assert.strictEqual(EscrowTokenAccount.mint.toString(), a_to_c_mint.toString());
      assert.strictEqual(EscrowTokenAccount.owner.toString(), escrow_pkey.toString());
      assert.strictEqual(EscrowTokenAccount.amount.toString(), a_to_c_amount.toString());

      const alice_sendATAAfter = await token.getAccount(provider.connection, alice_send_ata2, "confirmed");
      assert.strictEqual(alice_sendATAAfter.mint.toString(), a_to_c_mint.toString());
      assert.strictEqual(alice_sendATAAfter.owner.toString(), alice.publicKey.toString());
      assert.strictEqual(alice_sendATAAfter.amount.toString(), to_mint.sub(a_to_c_amount).toString());
    });
  });
  describe("Cancel Exchange", async () => {
    it("Cancel Exchange to Tom", async () => {
      const [escrow_pkey, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          alice.publicKey.toBuffer(),
          tom.publicKey.toBuffer(),
          a_to_c_mint.toBuffer(),
          c_to_a_mint.toBuffer(),
          a_to_c_amount.toArrayLike(Buffer, "le", 8),
          c_to_a_amount.toArrayLike(Buffer, "le", 8),
        ], program.programId);

      const [escrow_token_pkey, escrow_token_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          escrow_pkey.toBuffer(),
        ], program.programId);

      const alice_sendATABefore = await token.getAccount(provider.connection, alice_send_ata2, "confirmed");
      const alice_balance_before = await provider.connection.getBalance(alice.publicKey);

      await program.methods.cancelExchange().accounts({
        sideA: alice.publicKey,
        escrow: escrow_pkey,
        escrowTokenAccount: escrow_token_pkey,
        sideASendTokenAccount: alice_send_ata2,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      }).signers([alice]).rpc({ commitment: "confirmed" });

      const alice_balance_after = await provider.connection.getBalance(alice.publicKey);
      const alice_sendATAafter = await token.getAccount(provider.connection, alice_send_ata2, "confirmed");

      const escrow_rent = await provider.connection.getMinimumBalanceForRentExemption(154);
      const escrow_token_rent = await provider.connection.getMinimumBalanceForRentExemption(165);

      assert.strictEqual((BigInt(a_to_c_amount.toNumber()) + alice_sendATABefore.amount).toString(), alice_sendATAafter.amount.toString());
      assert.strictEqual(alice_balance_after - alice_balance_before, escrow_rent + escrow_token_rent);
    });
  });
  describe("Finalize Exchange", async () => {
    it("Bob finalizes Exchange", async () => {
      const [escrow_pkey, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          alice.publicKey.toBuffer(),
          bob.publicKey.toBuffer(),
          a_to_b_mint.toBuffer(),
          b_to_a_mint.toBuffer(),
          a_to_b_amount.toArrayLike(Buffer, "le", 8),
          b_to_a_amount.toArrayLike(Buffer, "le", 8),
        ], program.programId);

      const [escrow_token_pkey, escrow_token_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          escrow_pkey.toBuffer(),
        ], program.programId);


      const alice_TokenATAReceiveBefore = await token.getAccount(provider.connection, alice_receive_ata, "confirmed");
      const bob_TokenATAReceiveBefore = await token.getAccount(provider.connection, bob_receive_ata, "confirmed");
      const bob_TokenATASendBefore = await token.getAccount(provider.connection, bob_send_ata, "confirmed");
      const alice_balance_before = await provider.connection.getBalance(alice.publicKey);

      await program.methods.finalizeExchange().accounts({
        sideB: bob.publicKey,
        escrow: escrow_pkey,
        sideA: alice.publicKey,
        sideAReceiveTokenAccount: alice_receive_ata,
        sideBReceiveTokenAccount: bob_receive_ata,
        sideBSendTokenAccount: bob_send_ata,
        escrowTokenAccount: escrow_token_pkey,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      }).signers([bob]).rpc({ commitment: "confirmed" });

      const alice_TokenATAReceiveAfter = await token.getAccount(provider.connection, alice_receive_ata, "confirmed");
      const bob_TokenATAReceiveAfter = await token.getAccount(provider.connection, bob_receive_ata, "confirmed");
      const bob_TokenATASendAfter = await token.getAccount(provider.connection, bob_send_ata, "confirmed");

      assert.strictEqual((BigInt(b_to_a_amount.toNumber()) + alice_TokenATAReceiveBefore.amount).toString(), alice_TokenATAReceiveAfter.amount.toString());
      assert.strictEqual((BigInt(a_to_b_amount.toNumber()) + bob_TokenATAReceiveBefore.amount).toString(), bob_TokenATAReceiveAfter.amount.toString());
      assert.strictEqual((BigInt(b_to_a_amount.toNumber()) + bob_TokenATASendAfter.amount).toString(), bob_TokenATASendBefore.amount.toString());


      const alice_balance_after = await provider.connection.getBalance(alice.publicKey);

      const escrow_rent = await provider.connection.getMinimumBalanceForRentExemption(154);
      const escrow_token_rent = await provider.connection.getMinimumBalanceForRentExemption(165);

      assert.strictEqual(alice_balance_after - alice_balance_before, escrow_rent + escrow_token_rent);

      console.log("sideA Balance after Exchange: ", alice_TokenATAReceiveAfter.amount.toString())
      console.log("sideB Balance after Exchange: ", bob_TokenATAReceiveAfter.amount.toString())

    });
  });
});
async function airdrop(connection: any, address: any, amount = 10000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}
