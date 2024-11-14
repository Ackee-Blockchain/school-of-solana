import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LessonExample } from "../target/types/lesson_example";
import * as splToken from '@solana/spl-token';


describe("lesson-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LessonExample as Program<LessonExample>;

  let signer1 = anchor.web3.Keypair.generate();
  let signer2 = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    await airdrop(program.provider.connection, signer1.publicKey, 500_000_000_000);

    let [vault_data, bump_a] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_data"), signer1.publicKey.toBuffer()],
      program.programId
    );

    let [mint, bump_m] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint"), signer1.publicKey.toBuffer()],
      program.programId
    );

    let new_vault = splToken.getAssociatedTokenAddressSync(mint, vault_data, true);


    console.log("new vault: ", new_vault.toString())
    console.log("authority: ", vault_data.toString())
    console.log("new mint: ", mint.toString())
    console.log("signer: ", signer1.publicKey.toString())



    let tx = await program.methods.initialize().accounts({
      signer: signer1.publicKey,
      vaultData: vault_data,
      newMint: mint,
      newVault: new_vault,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: splToken.TOKEN_PROGRAM_ID,
      associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
    }).signers([signer1]).rpc({ skipPreflight: false });

    console.log(tx)
  });

  it("Grab", async () => {
    await airdrop(program.provider.connection, signer2.publicKey, 500_000_000_000);

    let [vault_data, bump_a] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_data"), signer1.publicKey.toBuffer()],
      program.programId
    );

    let [mint, bump_m] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint"), signer1.publicKey.toBuffer()],
      program.programId
    );

    let vault = splToken.getAssociatedTokenAddressSync(mint, vault_data, true);

    let signer_vault = await splToken.createAssociatedTokenAccount(program.provider.connection, signer2, mint, signer2.publicKey);

    console.log("signer_vault: ", signer_vault.toString())
    console.log("signer: ", signer2.publicKey.toString())


    let tx = await program.methods.grab().accounts({
      signer: signer2.publicKey,
      vaultData: vault_data,
      mint: mint,
      signerVault: signer_vault,
      vault: vault,
      tokenProgram: splToken.TOKEN_PROGRAM_ID,
    }).signers([signer2]).rpc({ skipPreflight: false });

    console.log(tx)
  });
});


export async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    'confirmed'
  );
}
