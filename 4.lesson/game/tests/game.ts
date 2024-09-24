// Import required modules from Anchor and Solana's web3.js.
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { Game } from '../target/types/game';
import { expect } from 'chai';

// Define a test suite for the Game program.
describe('game', async () => {
  // Initialize the provider to interact with the Solana network.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Reference the deployed Game program for testing.
  const program = anchor.workspace.Game as Program<Game>;

  // Test to verify name setting and changing functionality.
  it('Sets and changes name!', async () => {
    // Find the Program Derived Address (PDA) for the userStats account.
    const [userStatsPDA, _] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('user-stats'), // Encode the seed for the PDA.
        provider.wallet.publicKey.toBuffer(), // Include the public key of the wallet as part of the seed.
      ],
      program.programId // Specify the program ID used for generating the PDA.
    );

    // Create user stats with the name 'brian'.
    await program.methods
      .createUserStats('brian')
      .accounts({
        user: provider.wallet.publicKey, // Specify the user account as the payer.
        userStats: userStatsPDA, // Specify the PDA as the userStats account.
      })
      .rpc(); // Perform the RPC call to execute the method on-chain.

    // Fetch the userStats account and assert that the name is correctly set to 'brian'.
    expect((await program.account.userStats.fetch(userStatsPDA)).name).to.equal('brian');

    // Change the name in the userStats account to 'tom'.
    await program.methods
      .changeUserName('tom')
      .accounts({
        user: provider.wallet.publicKey, // Specify the user account as the payer.
        userStats: userStatsPDA, // Specify the PDA as the userStats account.
      })
      .rpc(); // Perform the RPC call to execute the method on-chain.

    // Fetch the userStats account again and assert that the name is changed to 'tom'.
    expect((await program.account.userStats.fetch(userStatsPDA)).name).to.equal('tom');
  });
});
