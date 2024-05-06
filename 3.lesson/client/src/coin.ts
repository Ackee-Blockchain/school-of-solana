import {
    Connection,
    PublicKey,
    SystemProgram,
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
    const initializer = getKeypair("initializer");
    const user = getKeypair("user");
    const [treasury, _bump] = await PublicKey.findProgramAddress([initializer.publicKey.toBuffer()], turnstileProgramId);

    const coinStateIx = new TransactionInstruction({
        programId: turnstileProgramId,
        keys: [
            {
                pubkey: state.publicKey, 
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: treasury, 
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: user.publicKey, 
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
            Uint8Array.of(2)    
        ),
    });

    const tx = new Transaction().add(
        coinStateIx
    );

    console.log("Sending coin transaction.");
    await connection.sendTransaction(
        tx,
        [user],
        { skipPreflight: false, preflightCommitment: "confirmed" }
    );

    await new Promise((resolve) => setTimeout(resolve, 1000));
}

coin();
