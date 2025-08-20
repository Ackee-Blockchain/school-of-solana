// Here we export some useful types and functions for interacting with the Anchor program.
import { address } from 'gill'
import { SolanaClusterId } from '@wallet-ui/react'
import { TICKETREGISTRY_PROGRAM_ADDRESS } from './client/js'
import TicketregistryIDL from '../target/idl/ticketregistry.json'

// Re-export the generated IDL and type
export { TicketregistryIDL }

// This is a helper function to get the program ID for the Ticketregistry program depending on the cluster.
export function getTicketregistryProgramId(cluster: SolanaClusterId) {
  switch (cluster) {
    case 'solana:devnet':
    case 'solana:testnet':
      // This is the program ID for the Ticketregistry program on devnet and testnet.
      return address('BmFQacFwfih2uHD5w4S8jZYJXSapKdFUW4WtUunt4cbM')
    case 'solana:mainnet':
    default:
      return TICKETREGISTRY_PROGRAM_ADDRESS
  }
}

export * from './client/js'
