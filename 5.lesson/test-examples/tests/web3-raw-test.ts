import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { assert } from "chai";
import crypto from "crypto";

// Compute Anchor's 8-byte instruction discriminator for a global method name.
function ixDiscriminator(name: string): Buffer {
  const preimage = `global:${name}`;
  return crypto.createHash("sha256").update(preimage).digest().subarray(0, 8);
}

describe("web3 raw -> test-examples", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.TestExamples as anchor.Program;

  const user = Keypair.generate();
  const data = PublicKey.findProgramAddressSync(
    [Buffer.from("data1"), Buffer.from("data2"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  before("airdrop user", async () => {
    await connection.confirmTransaction(
      await connection.requestAirdrop(user.publicKey, 1e9),
      "confirmed"
    );
  });

  it("initializes via raw instruction", async () => {
    // discriminator + borsh-encoded args (count: u8)
    const dataBytes = Buffer.concat([ixDiscriminator("initialize"), Buffer.from([10])]);
    const ix = new TransactionInstruction({
      programId: program.programId,
      keys: [
        { pubkey: user.publicKey, isSigner: true, isWritable: true },
        { pubkey: data, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: dataBytes,
    });

    const tx = new Transaction().add(ix);
    await provider.sendAndConfirm(tx, [user]);

    const acc = await program.account.myData.fetch(data);
    assert.deepEqual(acc.authority, user.publicKey);
    assert.strictEqual(acc.counter, 0);
  });
});

