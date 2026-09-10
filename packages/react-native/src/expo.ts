import { requireOptionalNativeModule } from 'expo-modules-core';

/**
 * Storage for the wallet mnemonic, readable by the Android notification service.
 *
 * The service that handles incoming payments runs with the app killed and no JS runtime, so it
 * cannot call back into JS to ask for the mnemonic — it has to read it from somewhere itself.
 * expo-secure-store cannot serve that role on Android: its values are wrapped in a private
 * encryption envelope that is only readable from a JS context. So the plugin keeps its own copy,
 * encrypted with a key held in the AndroidKeyStore.
 *
 * Call `setMnemonic` once when the wallet is created or restored, and `deleteMnemonic` on sign out.
 *
 * These are no-ops on iOS, where the notification service extension reads the system keychain
 * directly.
 */
const BreezMnemonicStore = requireOptionalNativeModule<{
  setMnemonic(mnemonic: string): Promise<void>;
  getMnemonic(): Promise<string | null>;
  deleteMnemonic(): Promise<void>;
}>('BreezMnemonicStore');

/**
 * Stores the mnemonic so the Android notification service can connect while the app is not running.
 */
export async function setMnemonic(mnemonic: string): Promise<void> {
  await BreezMnemonicStore?.setMnemonic(mnemonic);
}

/** Returns the stored mnemonic, or null if none is stored or the key was invalidated. */
export async function getMnemonic(): Promise<string | null> {
  return (await BreezMnemonicStore?.getMnemonic()) ?? null;
}

/** Removes the stored mnemonic and the key protecting it. */
export async function deleteMnemonic(): Promise<void> {
  await BreezMnemonicStore?.deleteMnemonic();
}
