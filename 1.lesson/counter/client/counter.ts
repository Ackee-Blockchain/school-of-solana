/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
  import * as fs from 'mz/fs';
  import * as path from 'path';
  import * as borsh from 'borsh';

  import {getPayer, getRpcUrl, readKeypairFromFile} from './utils';

  /**
   * Connection to the network
   */
  let connection: Connection;

  /**
   * Keypair associated to the fees' payer
   */
  let payer: Keypair;

  /**
   * Hello world's program id
   */
  let programId: PublicKey;

  /**
   * The public key of the account we are saying hello to
   */
  let counter: Keypair;

  /**
   * Path to program files
   */
  const PROGRAM_PATH = path.resolve(__dirname, '../dist/program');

  /**
   * Path to program shared object file which should be deployed on chain.
   * This file is created when running either:
   *   - `npm run build:program-c`
   *   - `npm run build:program-rust`
   */
  const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'counter.so');

  /**
   * Path to the keypair of the deployed program.
   * This file is created when running `solana program deploy dist/program/helloworld.so`
   */
  const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'counter-keypair.json');

  /**
   * The state of a counter account managed by the counter program
   */
  class CounterAccount {
    counter = 0;
    constructor(fields: {counter: number} | undefined = undefined) {
      if (fields) {
        this.counter = fields.counter;
      }
    }
  }

  /**
   * Borsh schema definition for greeting accounts
   */
  const CounterSchema = new Map([
    [CounterAccount, {kind: 'struct', fields: [['counter', 'u32']]}],
  ]);

  /**
   * The expected size of each greeting account.
   */
  const COUNTER_SIZE = borsh.serialize(
    CounterSchema,
    new CounterAccount(),
  ).length;

  /**
   * Establish a connection to the cluster
   */
  export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
  }

  /**
   * Establish an account to pay for everything
   */
  export async function establishPayer(): Promise<void> {
    let fees = 0;
    if (!payer) {
      const {feeCalculator} = await connection.getRecentBlockhash();

      // Calculate the cost to fund the greeter account
      fees += await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

      // Calculate the cost of sending transactions
      fees += feeCalculator.lamportsPerSignature * 100; // wag

      payer = await getPayer();
    }

    let lamports = await connection.getBalance(payer.publicKey);
    if (lamports < fees) {
      // If current balance is not enough to pay for fees, request an airdrop
      const sig = await connection.requestAirdrop(
        payer.publicKey,
        fees - lamports,
      );
      await connection.confirmTransaction(sig);
      lamports = await connection.getBalance(payer.publicKey);
    }

    console.log(
      'Using account',
      payer.publicKey.toBase58(),
      'containing',
      lamports / LAMPORTS_PER_SOL,
      'SOL to pay for fees',
    );
  }

  /**
   * Check if the counter BPF program has been deployed
   */
  export async function checkProgram(): Promise<void> {
    // Read program id from keypair file
    try {
      const programKeypair = await readKeypairFromFile(PROGRAM_KEYPAIR_PATH);
      programId = programKeypair.publicKey;
    } catch (err) {
      const errMsg = (err as Error).message;
      throw new Error(
        `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
      );
    }

    // Check if the program has been deployed
    const programInfo = await connection.getAccountInfo(programId);
    if (programInfo === null) {
      if (fs.existsSync(PROGRAM_SO_PATH)) {
        throw new Error(
          'Program needs to be deployed with `solana program deploy dist/program/helloworld.so`',
        );
      } else {
        throw new Error('Program needs to be built and deployed');
      }
    } else if (!programInfo.executable) {
      throw new Error(`Program is not executable`);
    }
    console.log(`Using program ${programId.toBase58()}`);

    // Generate the address (public key) of a counter account.
    counter = Keypair.generate()

    // Check if the counter account has already been created
    const counterAccount = await connection.getAccountInfo(counter.publicKey);
    if (counterAccount === null) {
      console.log(
        'Creating account',
        counter.publicKey.toBase58(),
      );
      const lamports = await connection.getMinimumBalanceForRentExemption(
        COUNTER_SIZE,
      );

      const tx = new Transaction().add(
          SystemProgram.createAccount({
              fromPubkey: payer.publicKey,
              newAccountPubkey: counter.publicKey,
              lamports,
              space: COUNTER_SIZE,
              programId
          }),
      );

      await sendAndConfirmTransaction(connection, tx, [payer, counter]);
    }
  }

  /**
   * Increment
   */
  export async function increment(): Promise<void> {
    console.log('Incrementing ', counter.publicKey.toBase58());
    const instruction = new TransactionInstruction({
      keys: [{pubkey: counter.publicKey, isSigner: false, isWritable: true}],
      programId,
      data: Buffer.alloc(1),
    });
    await sendAndConfirmTransaction(
      connection,
      new Transaction().add(instruction),
      [payer],
    );
  }

   /**
    * Decrement
    */
   export async function decrement(): Promise<void> {
    console.log('Incrementing ', counter.publicKey.toBase58());
    const instruction = new TransactionInstruction({
        keys: [{pubkey: counter.publicKey, isSigner: false, isWritable: true}],
        programId,
        data: Buffer.alloc(2),
    });
    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );
}

  /**
   * Report the number of times the greeted account has been said hello to
   */
  export async function getValue(): Promise<void> {
    const accountInfo = await connection.getAccountInfo(counter.publicKey);
    if (accountInfo === null) {
      throw 'Error: cannot find the greeted account';
    }
    const state = borsh.deserialize(
      CounterSchema,
      CounterAccount,
      accountInfo.data,
    );
    console.log(
      counter.publicKey.toBase58(),
      'has value',
      state.counter,
    );
}
