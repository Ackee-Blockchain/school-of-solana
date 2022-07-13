import {
    Connection,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";

import {
    getKeypair,
    getProgramId,
} from "./utils";

const push = async () => {
    const connection = new Connection("http://localhost:8899", "confirmed");
    const turnstileProgramId = getProgramId();
    const state = getKeypair("state");
    const payer = getKeypair("initializer");
    const user = getKeypair("user");

    const pushStateIx = new TransactionInstruction({
        programId: turnstileProgramId,
        keys: [
            {
                pubkey: state.publicKey,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: user.publicKey,
                isSigner: true,
                isWritable: false,
            },
        ],
        data: Buffer.from(
            Uint8Array.of(1)
        ),
    });

    const tx = new Transaction().add(
        pushStateIx
    );

    console.log("Sending push transaction.");
    await connection.sendTransaction(
        tx,
        [user],
        { skipPreflight: false, preflightCommitment: "confirmed" }
    );

    await new Promise((resolve) => setTimeout(resolve, 1000));
}

push();
