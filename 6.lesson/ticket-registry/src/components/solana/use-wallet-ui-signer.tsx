import { UiWalletAccount, useWalletAccountTransactionSendingSigner, useWalletUi } from '@wallet-ui/react'

export function useWalletUiSigner() {
  const { account, cluster } = useWalletUi()

  return useWalletAccountTransactionSendingSigner(account as UiWalletAccount, cluster.id)
}
