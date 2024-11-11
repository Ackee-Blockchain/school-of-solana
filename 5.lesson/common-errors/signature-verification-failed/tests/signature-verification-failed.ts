import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SignatureVerificationFailed } from "../target/types/signature_verification_failed";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("signature-verification-failed", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace
    .SignatureVerificationFailed as Program<SignatureVerificationFailed>;

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
      .signers([user]) // Missing signer.
      // .signers([user, data]) // Include data as an additional signer.
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
