import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramFailedToComplete } from "../target/types/program_failed_to_complete";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("program-failed-to-complete", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace
    .ProgramFailedToComplete as Program<ProgramFailedToComplete>;
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
      // Add a check and custom error in lib.rs.
      // This change will not pass the test,
      // however, the error message will now
      // be more informative.
      .initialize(11) // The value 11 is intentionally too high.
      .accountsStrict({
        user: user.publicKey,
        data: data,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);
  });
});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}
