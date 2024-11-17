import { verify } from '@noble/ed25519';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import bs58 from 'bs58';
import { FC, useCallback, useState } from 'react';
import { notify } from "../utils/notifications";

import { Program, AnchorProvider, web3, utils, BN, setProvider } from "@coral-xyz/anchor"
import idl from "./bank.json"
import { Bank } from "./bank"
import { PublicKey } from '@solana/web3.js';

const idl_string = JSON.stringify(idl)
const idl_object = JSON.parse(idl_string)
const programID = new PublicKey(idl.address)

export const Bank: FC = () => {
    const ourWallet = useWallet();
    const { connection } = useConnection()
    const [banks, setBanks] = useState([])

    const getProvider = () => {
        const provider = new AnchorProvider(connection, ourWallet, AnchorProvider.defaultOptions())
        setProvider(provider)
        return provider
    }

    const createBank = async () => {
        try {
            const anchProvider = getProvider()
            const program = new Program<Bank>(idl_object, anchProvider)

            await program.methods.create("New Bank").accounts({
                user: anchProvider.publicKey
            }).rpc()

            console.log("Wow, new bank was created")

        } catch (error) {
            console.error("Error while creating a bank: " + error)
        }
    }

    const getBanks = async () => {
        try {
            const anchProvider = getProvider()
            const program = new Program<Bank>(idl_object, anchProvider)
            Promise.all((await connection.getParsedProgramAccounts(programID)).map(async bank => ({
                ...(await program.account.bank.fetch(bank.pubkey)),
                pubkey: bank.pubkey
            }))).then(banks => {
                console.log(banks)
                setBanks(banks)
            })


        } catch (error) {
            console.error("Error while getting banks: " + error)
        }
    }

    const depositBank = async (publicKey) => {
        try {
            const anchProvider = getProvider()
            const program = new Program<Bank>(idl_object, anchProvider)

            await program.methods.deposit(new BN(0.1 * web3.LAMPORTS_PER_SOL))
                .accounts({
                    bank: publicKey,
                    user: anchProvider.publicKey
                }).rpc()

            console.log(" Deposit done: " + publicKey)

        } catch (error) {
            console.error("Error while depositing to a bank: " + error)
        }
    }

    return (
        <div>
            {
                banks.map((bank) => {
                    return (
                        <div className='md:hero-content flex flex-col'>
                            <h1>{bank.name.toString()}</h1>
                            <span>{bank.balance.toString()}</span>
                            <button
                                className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                                onClick={() => depositBank(bank.pubkey)}>
                                <span>
                                    Deposit 0.1
                                </span>
                            </button>
                        </div>
                    )
                })
            }
            <div className="flex flex-row justify-center">
                <div className="relative group items-center">
                    <div className="m-1 absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-fuchsia-500
                rounded-lg blur opacity-20 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={createBank}
                    >
                        <div className="hidden group-disabled:block">
                            Wallet not connected
                        </div>
                        <span className="block group-disabled:hidden" >
                            Create Bank
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={getBanks}
                    >
                        <div className="hidden group-disabled:block">
                            Wallet not connected
                        </div>
                        <span className="block group-disabled:hidden" >
                            Fetch Banks
                        </span>
                    </button>
                </div>
            </div>
        </div>
    );
};
