import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { Turnstile } from "../target/types/turnstile";

describe("turnstile", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Turnstile as Program<Turnstile>;
  const state = anchor.web3.Keypair.generate();
  const user = (program.provider as anchor.AnchorProvider).wallet;

  it("Is initialized!", async () => {
    // Add your test here.
    await program.methods.initialize().accounts({
      state: state.publicKey,
      user: user.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([state])
    .rpc();
  });


  it("Coin!", async () => {
    // Add your test here.
    await program.methods.coin().accounts({
      state: state.publicKey,
    })
    .signers([])
    .rpc();

    let turnstileState = await program.account.state.fetch(state.publicKey);
    expect(turnstileState.locked).to.equal(false);
  });

  it("Push!", async () => {
    // Add your test here.
    await program.methods.push().accounts({
      state: state.publicKey,
    })
    .signers([])
    .rpc();
  });


});
