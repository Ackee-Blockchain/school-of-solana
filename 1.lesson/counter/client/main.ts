/**
 * Hello world
 */

 import {
    establishConnection,
    establishPayer,
    checkProgram,
    increment,
    getValue,
    decrement,
  } from './counter';

  async function main() {
    // Establish connection to the cluster
    await establishConnection();

    // Determine who pays for the fees
    await establishPayer();

    // Check if the program has been deployed
    await checkProgram();

    // Say hello to an account
    await increment();

    await decrement();

    // Find out the value of counter
    await getValue();

    console.log('Success');
  }

  main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
  );
