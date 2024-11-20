import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bank } from "../target/types/bank";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("bank", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;
  const program = anchor.workspace.Bank as Program<Bank>;

  const user = Keypair.generate();

  before("prepare", async () => {
    await airdrop(connection, user.publicKey);
  });

  it("Is initialized!", async () => {

    const [bankPda, bump] = await PublicKey.findProgramAddress(
      [Buffer.from("bankaccount"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .create("Bank Title")
      .accounts({
        bank: bankPda,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Bank account created successfully:", bankPda.toBase58());
  });
});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}
