import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TestExamples } from "../target/types/test_examples";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("test-examples", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace.TestExamples as Program<TestExamples>;
  const user = Keypair.generate();
  const data = PublicKey.findProgramAddressSync(
    [Buffer.from("data1"), Buffer.from("data2")],
    program.programId
  )[0];

  before("prepare", async () => {
    await airdrop(connection, user.publicKey);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(10)
      .accountsStrict({
        user: user.publicKey,
        data: data,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    // Fetch and verify the on-chain data.
    let dataAccount = await program.account.myData.fetch(data);
    assert.deepEqual(dataAccount.authority, user.publicKey);
    assert.strictEqual(dataAccount.counter, 0);
  });

  // Do not forget to test both happy and unhappy case
  it("Cannot initialize with incorrect data account!", async () => {
    const bad_data = Keypair.generate();

    try {
      await program.methods
        .initialize(10)
        .accountsStrict({
          user: user.publicKey,
          data: bad_data.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();

    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "ConstraintSeeds");
    }
  });
});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}
