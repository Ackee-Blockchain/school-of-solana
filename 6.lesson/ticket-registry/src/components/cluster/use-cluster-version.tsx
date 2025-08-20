import { useWalletUi } from '@wallet-ui/react'
import { useQuery } from '@tanstack/react-query'

export function useClusterVersion() {
  const { client, cluster } = useWalletUi()
  return useQuery({
    retry: false,
    queryKey: ['version', { cluster }],
    queryFn: () => client.rpc.getVersion().send(),
  })
}
