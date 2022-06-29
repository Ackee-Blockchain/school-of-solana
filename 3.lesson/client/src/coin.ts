import {
    Connection,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";

import {
    getKeypair,
    getProgramId,
} from "./utils";

const coin = async () => {
    const connection = new Connection("http://localhost:8899", "confirmed");
    const turnstileProgramId = getProgramId();
    const state = getKeypair("state");
    const payer = getKeypair("initializer");

    const coinStateIx = new TransactionInstruction({
        programId: turnstileProgramId,
        keys: [
            {
                pubkey: state.publicKey, 
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: payer.publicKey, 
                isSigner: true,
                isWritable: true,
            },
        ],
        data: Buffer.from(
            Uint8Array.of(2)    
        ),
    });

    const tx = new Transaction().add(
        coinStateIx
    );

    console.log("Sending coin transaction.");
    await connection.sendTransaction(
        tx,
        [payer],
        { skipPreflight: false, preflightCommitment: "confirmed" }
    );

    await new Promise((resolve) => setTimeout(resolve, 1000));
}

coin();
