import { assertIsAddress } from 'gill'
import { useMemo } from 'react'
import { useParams } from 'next/navigation'
import { ExplorerLink } from '@/components/cluster/cluster-ui'
import { AppHero } from '@/components/app-hero'
import { ellipsify } from '@/lib/utils'

import { AccountBalance, AccountButtons, AccountTokens, AccountTransactions } from './account-ui'

export default function AccountFeatureDetail() {
  const params = useParams()
  const address = useMemo(() => {
    if (!params.address || typeof params.address !== 'string') {
      return
    }
    assertIsAddress(params.address)
    return params.address
  }, [params])
  if (!address) {
    return <div>Error loading account</div>
  }

  return (
    <div>
      <AppHero
        title={<AccountBalance address={address} />}
        subtitle={
          <div className="my-4">
            <ExplorerLink address={address.toString()} label={ellipsify(address.toString())} />
          </div>
        }
      >
        <div className="my-4">
          <AccountButtons address={address} />
        </div>
      </AppHero>
      <div className="space-y-8">
        <AccountTokens address={address} />
        <AccountTransactions address={address} />
      </div>
    </div>
  )
}
