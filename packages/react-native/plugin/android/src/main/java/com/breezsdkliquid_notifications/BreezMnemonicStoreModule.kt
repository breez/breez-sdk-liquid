package com.breezsdkliquid_notifications

import expo.modules.kotlin.exception.Exceptions
import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition

/**
 * Exposes [BreezMnemonicStore] to JS so the app can hand over the mnemonic once, at sign-in.
 *
 * Only the public expo-modules-core Module API is used here — no reaching into another module's
 * internals — so this keeps working across Expo SDK upgrades.
 */
class BreezMnemonicStoreModule : Module() {
    private val store: BreezMnemonicStore
        get() {
            val context = appContext.reactContext ?: throw Exceptions.ReactContextLost()
            return BreezMnemonicStore(context, BuildConfig.MNEMONIC_KEY_NAME)
        }

    override fun definition() = ModuleDefinition {
        Name("BreezMnemonicStore")

        AsyncFunction("setMnemonic") { mnemonic: String ->
            store.put(mnemonic)
        }

        AsyncFunction("getMnemonic") {
            store.get()
        }

        AsyncFunction("deleteMnemonic") {
            store.delete()
        }
    }
}
