import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";

describe("hello_world", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;
  const signer = anchor.web3.Keypair.generate();
  const hellog_world = anchor.web3.Keypair.generate();

  it("Greetings Solana!", async () => {
    await airdrop(program.provider.connection, signer.publicKey);

    const tx = await program.methods
      .initialize()
      .accounts({
        signer: signer.publicKey,
        helloWorldAccount: hellog_world.publicKey,
      })
      .signers([signer, hellog_world])
      .rpc();
  });
});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}
