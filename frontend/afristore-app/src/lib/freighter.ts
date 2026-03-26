// ─────────────────────────────────────────────────────────────
// lib/freighter.ts — Freighter browser wallet helpers
// ─────────────────────────────────────────────────────────────

import {
  getPublicKey,
  isConnected,
  signTransaction,
  setAllowed,
  getNetworkDetails,
} from "@stellar/freighter-api";

export interface FreighterAccount {
  publicKey: string;
  networkPassphrase: string;
}

/**
 * Returns true if the Freighter extension is installed in this browser.
 */
export async function isFreighterInstalled(): Promise<boolean> {
  try {
    return await isConnected();
  } catch {
    return false;
  }
}

// ── Connect wallet ────────────────────────────────────────────

/**
 * Requests access to Freighter and returns the public key + network.
 * Opens the Freighter popup if not yet allowed.
 */
export async function connectFreighter(): Promise<FreighterAccount> {
  const allowed = await setAllowed();
  if (!allowed) {
    throw new Error("Freighter access was denied by the user.");
  }

  const publicKey = await getPublicKey();
  if (!publicKey) {
    throw new Error("Could not retrieve public key from Freighter.");
  }

  const networkResult = await getNetworkDetails();
  if (!networkResult) {
    throw new Error("Could not retrieve network details from Freighter.");
  }

  return {
    publicKey,
    networkPassphrase: networkResult.networkPassphrase,
  };
}

// ── Sign a transaction XDR ────────────────────────────────────

/**
 * Asks Freighter to sign a transaction XDR string.
 * Returns the signed XDR string ready for submission.
 */
export async function signWithFreighter(
  txXdr: string,
  networkPassphrase: string
): Promise<string> {
  const signedTxXdr = await signTransaction(txXdr, { networkPassphrase });
  if (!signedTxXdr) {
    throw new Error("Freighter sign error: result was empty.");
  }
  return signedTxXdr;
}

// ── Get connected public key ──────────────────────────────────

/**
 * Returns the currently connected public key, or null if not connected.
 */
export async function getConnectedPublicKey(): Promise<string | null> {
  try {
    const connected = await isConnected();
    if (!connected) return null;
    const publicKey = await getPublicKey();
    return publicKey || null;
  } catch {
    return null;
  }
}
