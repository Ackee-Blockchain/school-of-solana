// Import required modules and classes from Anchor and Solana's web3.js
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { assert } from 'chai';
import { Puppet } from '../target/types/puppet';
import { PuppetMaster } from '../target/types/puppet_master';

// Define a test suite for the Puppet program.
describe('puppet', () => {
  // Initialize the provider to interact with the Solana network.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Reference the deployed Puppet and PuppetMaster programs for testing.
  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const puppetMasterProgram = anchor.workspace.PuppetMaster as Program<PuppetMaster>;

  // Generate a new keypair for the puppet account to be used in tests.
  const puppetKeypair = Keypair.generate();

  // Define a test for Cross-Program Invocation (CPI) between Puppet and PuppetMaster.
  it('Does CPI!', async () => {
    // Initialize the puppet account with the generated keypair.
    await puppetProgram.methods
      .initialize()
      .accounts({
        puppet: puppetKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([puppetKeypair])
      .rpc();

    // Call the pullStrings method of the PuppetMaster program to manipulate puppet data.
    await puppetMasterProgram.methods
      .pullStrings(new anchor.BN(42)) // BN represents a BigNumber for handling large integers.
      .accounts({
        puppetProgram: puppetProgram.programId,
        puppet: puppetKeypair.publicKey,
      })
      .rpc();

    // Fetch the updated data from the puppet account to verify the change.
    const puppetData = await puppetProgram.account.data.fetch(puppetKeypair.publicKey);

    // Assert that the data was correctly set to 42 by the CPI call.
    assert.equal(puppetData.data.toNumber(), 42);
  });
});
