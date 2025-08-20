import { EVENT_DISCRIMINATOR, getEventDecoder, getTicketregistryProgramId } from '@project/anchor'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import { toast } from 'sonner'
import { useWalletUi } from '@wallet-ui/react'
import { toastTx } from '@/components/toast-tx'
import { useWalletTransactionSignAndSend } from '@/components/solana/use-wallet-transaction-sign-and-send'
import { useWalletUiSigner } from '@/components/solana/use-wallet-ui-signer'
import { addDecoderSizePrefix, Address, createTransaction, getBase58Decoder, Instruction, signAndSendTransactionMessageWithSigners, SolanaClient, TransactionSigner } from 'gill'

export function useTicketregistryProgramId() {
  const { cluster } = useWalletUi()

  return useMemo(() => getTicketregistryProgramId(cluster.id), [cluster])
}

export async function processTransaction(
  signer: TransactionSigner,
  client: SolanaClient,
  instructions: Instruction[]
) {
  const { value: latestBlockhash } = await client.rpc.getLatestBlockhash().send()

  console.log('Creating transaction...')
  const transaction = createTransaction({
    feePayer: signer,
    version: 'legacy',
    latestBlockhash,
    instructions: instructions,
  })

  const signature = await signAndSendTransactionMessageWithSigners(transaction)
  const decoder = getBase58Decoder()
  const sig58 = decoder.decode(signature)
  console.log(sig58)
}

export async function getEventAccounts(client: SolanaClient, programId: Address) {
  const allAccounts = await client.rpc.getProgramAccounts(programId, {
    encoding: 'base64'
  }).send()

  const filteredAccounts = allAccounts.filter((account) => {
    const data = Buffer.from(account.account.data[0], 'base64')
    const discriminator = data.subarray(0, 8)
    return discriminator.equals(Buffer.from(EVENT_DISCRIMINATOR))
  })

  const decoder = getEventDecoder()
  const decodedAccounts = filteredAccounts.map((account) => ({
    address: account.pubkey,
    data: decoder.decode(Buffer.from(account.account.data[0], "base64"))
  }))

  return decodedAccounts
}
