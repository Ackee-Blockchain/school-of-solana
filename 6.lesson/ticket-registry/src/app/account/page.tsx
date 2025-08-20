'use client'
import { redirect } from 'next/navigation'
import AccountFeatureIndex from '@/components/account/account-feature-index'

export default function Page() {
  return <AccountFeatureIndex redirect={redirect} />
}
