/**
 * Hook for Solana wallet connection.
 * TODO: Integrate @solana/wallet-adapter-react
 */
import { useState } from 'react';

export function useWalletAuth() {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);

  const connect = async () => {
    setIsConnecting(true);
    try {
      // TODO: Replace with actual wallet adapter
      // const wallet = useWallet();
      // await wallet.connect();
      // setPublicKey(wallet.publicKey?.toBase58() ?? null);
      console.log('Wallet connect not yet implemented');
    } catch (err) {
      console.error('Failed to connect wallet:', err);
    } finally {
      setIsConnecting(false);
    }
  };

  const disconnect = () => {
    setPublicKey(null);
  };

  return { publicKey, isConnecting, isConnected: !!publicKey, connect, disconnect };
}
