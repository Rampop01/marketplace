// ─────────────────────────────────────────────────────────────
// hooks/useWallet.ts — Freighter wallet connection state
// ─────────────────────────────────────────────────────────────

"use client";

import { useState, useCallback, useEffect } from "react";
import {
  connectFreighter,
  getConnectedPublicKey,
  isFreighterInstalled,
} from "@/lib/freighter";

export type WalletStatus =
  | "NOT_INSTALLED"
  | "DISCONNECTED"
  | "CONNECTING"
  | "CONNECTED"
  | "WRONG_NETWORK";

export interface WalletState {
  publicKey: string | null;
  networkPassphrase: string | null;
  status: WalletStatus;
  isInstalled: boolean;
  isConnecting: boolean;
  isConnected: boolean;
  isWrongNetwork: boolean;
  error: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
  refresh: () => Promise<void>;
}

import { config } from "@/lib/config";

export function useWallet(): WalletState {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [networkPassphrase, setNetworkPassphrase] = useState<string | null>(null);
  const [isInstalled, setIsInstalled] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Derive status
  const isWrongNetwork = !!publicKey &&
    !!networkPassphrase &&
    networkPassphrase !== config.networkPassphrase;

  const status: WalletStatus = !isInstalled
    ? "NOT_INSTALLED"
    : isConnecting
      ? "CONNECTING"
      : !publicKey
        ? "DISCONNECTED"
        : isWrongNetwork
          ? "WRONG_NETWORK"
          : "CONNECTED";

  const refresh = useCallback(async () => {
    const installed = await isFreighterInstalled();
    setIsInstalled(installed);
    if (installed) {
      try {
        const key = await getConnectedPublicKey();
        if (key) {
          setPublicKey(key);
          // Also fetch network info if possible
          const account = await connectFreighter().catch(() => null);
          if (account) {
            setNetworkPassphrase(account.networkPassphrase);
          }
        } else {
          setPublicKey(null);
          setNetworkPassphrase(null);
        }
      } catch (err) {
        console.error("Wallet refresh error:", err);
      }
    }
  }, []);

  // Check install status + auto-reconnect on mount.
  useEffect(() => {
    refresh();
  }, [refresh]);

  const connect = useCallback(async () => {
    setIsConnecting(true);
    setError(null);
    try {
      const account = await connectFreighter();
      setPublicKey(account.publicKey);
      setNetworkPassphrase(account.networkPassphrase);

      if (account.networkPassphrase !== config.networkPassphrase) {
        setError(`Wrong network! Please switch Freighter to ${config.network}.`);
      }
    } catch (err: unknown) {
      if (err instanceof Error && err.message.includes("denied")) {
        setError("Connection request was rejected.");
      } else {
        setError(err instanceof Error ? err.message : "Failed to connect wallet");
      }
    } finally {
      setIsConnecting(false);
    }
  }, []);

  const disconnect = useCallback(() => {
    setPublicKey(null);
    setNetworkPassphrase(null);
    setError(null);
  }, []);

  return {
    publicKey,
    networkPassphrase,
    status,
    isInstalled,
    isConnecting,
    isConnected: status === "CONNECTED",
    isWrongNetwork,
    error,
    connect,
    disconnect,
    refresh,
  };
}

