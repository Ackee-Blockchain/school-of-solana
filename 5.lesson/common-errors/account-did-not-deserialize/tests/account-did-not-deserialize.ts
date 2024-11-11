import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountDidNotDeserialize } from "../target/types/account_did_not_deserialize";
import {
  Keypair,
  SystemProgram,
} from "@solana/web3.js";

describe("account-did-not-deserialize", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace.AccountDidNotDeserialize as Program<AccountDidNotDeserialize>;
  const user = Keypair.generate();
  const data = Keypair.generate();

  before("prepare", async () => {
    await airdrop(connection, user.publicKey);
  });

  it("Is initialized!", async () => {
    // Account failed to deserialize. Check out lib.rs to fix the test.
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
