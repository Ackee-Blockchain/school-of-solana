import { WalletButton } from '../solana/solana-provider'
import { TicketregistryProgram, TicketregistryProgramExplorerLink } from './ticketregistry-ui'
import { AppHero } from '../app-hero'
import { useWalletUi } from '@wallet-ui/react'

export default function TicketregistryFeature() {
  const { account } = useWalletUi()

  if (!account) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="hero py-[64px]">
          <div className="hero-content text-center">
            <WalletButton />
          </div>
        </div>
      </div>
    )
  }

  return (
    <div>
      <AppHero title="Ticketregistry" subtitle={'Run the program by clicking the "Run program" button.'}>
      </AppHero>
      <TicketregistryProgram />
    </div>
  )
}
