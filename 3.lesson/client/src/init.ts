import {
    Connection,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";

import {
    getKeypair,
    getProgramId,
} from "./utils";

const init = async () => {
    const connection = new Connection("http://localhost:8899", "confirmed");
    const turnstileProgramId = getProgramId();
    const initializer = getKeypair("initializer");
    const state = getKeypair("state");

    const initStateIx = new TransactionInstruction({
        programId: turnstileProgramId,
        keys: [
            {
                pubkey: state.publicKey, 
                isSigner: true,
                isWritable: true,
            },
            {
                pubkey: initializer.publicKey, 
                isSigner: true,
                isWritable: true,
            },
            {
                pubkey: SystemProgram.programId, 
                isSigner: false,
                isWritable: false,
            },
        ],
        data: Buffer.from(
            Uint8Array.of(0, 1)    
        ),
    });

    const tx = new Transaction().add(
        initStateIx
    );

    console.log("Sending init transaction.");
    await connection.sendTransaction(
        tx,
        [initializer, state],
        { skipPreflight: false, preflightCommitment: "confirmed" }
    );

    await new Promise((resolve) => setTimeout(resolve, 1000));
}

init();
