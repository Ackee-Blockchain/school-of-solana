import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NoPriorCredit } from "../target/types/no_prior_credit";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("no-prior-credit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace.NoPriorCredit as Program<NoPriorCredit>;

  const user = Keypair.generate();
  const data = Keypair.generate();

  before("prepare", async () => {
    // Fund the user's account with SOL to cover transaction cost.
    // await airdrop(connection, user.publicKey);

    // Retrieve and log the user's account balance with `getBalance`.
    // console.log(
    //   "User balance = " + (await connection.getBalance(user.publicKey))
    // );
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accountsStrict({
        user: user.publicKey,
        data: data.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user, data])
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
