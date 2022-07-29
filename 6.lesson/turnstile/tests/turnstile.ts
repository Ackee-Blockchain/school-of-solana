import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { Turnstile } from "../target/types/turnstile";
import { expect } from 'chai';

describe("turnstile", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Turnstile as Program<Turnstile>;
  const state = anchor.web3.Keypair.generate();
  const user = (program.provider as anchor.AnchorProvider).wallet;

  it("Is initialized!", async () => {

    /**
     * The network and wallet context used to send transactions paid for and signed
     * by the provider.
    */

    const [treasuryPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("treasury"),
        ],
        program.programId
      );
    await program.methods
      .initialize()
      .accounts({
        state: state.publicKey,
        user: user.publicKey,
        treasury: treasuryPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([state])
      .rpc();

    let turnstileState = await program.account.state.fetch(state.publicKey);
    expect(turnstileState.locked).to.equal(true);
  });

  it("Coin!", async () => {

    const [treasuryPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("treasury"),
        ],
        program.programId
      );

    /**
     * The network and wallet context used to send transactions paid for and signed
     * by the provider.
    */

    await program.methods
      .coin()
      .accounts({
        state: state.publicKey,
        treasury: treasuryPDA,
        user: user.publicKey,
      })
      .signers([])
      .rpc();

    let turnstileState = await program.account.state.fetch(state.publicKey);
    expect(turnstileState.locked).to.equal(false);
    expect(turnstileState.payer.toBase58).to.equal(user.publicKey.toBase58);
  });

  it("Push!", async () => {

    const [treasuryPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("treasury"),
        ],
        program.programId
      );

    /**
     * The network and wallet context used to send transactions paid for and signed
     * by the provider.
    */

    await program.methods
      .push()
      .accounts({
        state: state.publicKey,
        treasury: treasuryPDA,
        payer: user.publicKey,
      })
      .signers([])
      .rpc();

    let turnstileState = await program.account.state.fetch(state.publicKey);
    expect(turnstileState.locked).to.equal(true);
    expect(turnstileState.payer.toBase58).to.equal(PublicKey.default.toBase58);
  });
});
