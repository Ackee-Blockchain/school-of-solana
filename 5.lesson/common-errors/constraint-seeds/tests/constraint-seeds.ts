import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConstraintSeeds } from "../target/types/constraint_seeds";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("constraint-seeds", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace.ConstraintSeeds as Program<ConstraintSeeds>;
  const user = Keypair.generate();

  // Keypair no longer works because data is now a PDA.
  // Check out lib.rs.
  const data = Keypair.generate().publicKey;

  // Derive the PDA for the data account.
  // const data = PublicKey.findProgramAddressSync(
  //   [Buffer.from("data2"), Buffer.from("data1")],
  //   program.programId
  // )[0];

  // Ensure correct seed order when deriving the PDA.
  // const data = PublicKey.findProgramAddressSync(
  //   [Buffer.from("data1"), Buffer.from("data2")],
  //   program.programId
  // )[0];

  before("prepare", async () => {
    await airdrop(connection, user.publicKey);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accountsStrict({
        user: user.publicKey,
        data: data,
        systemProgram: SystemProgram.programId,
      })
      .signers([user]) // Exclude data as the program will sign on behalf of the PDA.
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
