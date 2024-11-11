import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountAlreadyInUse } from "../target/types/account_already_in_use";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("account-already-in-use", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace
    .AccountAlreadyInUse as Program<AccountAlreadyInUse>;
  const user = Keypair.generate();
  const data = Keypair.generate();

  before("prepare", async () => {
    await airdrop(connection, user.publicKey);
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

    // Log public keys for debugging.
    // console.log("Data account pubkey: ", data.publicKey.toString());
    // console.log("User account pubkey: ", user.publicKey.toString());

    // Attempting to initialize the data account twice.
    // To fix the test, comment out the transaction and log below.
    const repeat_tx = await program.methods
      .initialize()
      .accountsStrict({
        user: user.publicKey,
        data: data.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user, data])
      .rpc();

    console.log("Your transaction signature", repeat_tx);
  });
});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}
