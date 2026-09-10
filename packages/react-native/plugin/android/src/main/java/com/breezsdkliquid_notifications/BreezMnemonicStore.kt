package com.breezsdkliquid_notifications

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyPermanentlyInvalidatedException
import android.security.keystore.KeyProperties
import android.util.Base64
import java.security.GeneralSecurityException
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

/**
 * Stores the wallet mnemonic where the headless notification service can read it.
 *
 * expo-secure-store deliberately isn't used. It is a JS-context module, and on Android it wraps
 * values in a private encryption envelope, so reading it from a Service without a JS runtime
 * means importing its internals and duplicating its storage format — which breaks whenever Expo
 * changes them. This owns its format instead: an AES-GCM key that never leaves the
 * AndroidKeyStore, and ciphertext in SharedPreferences.
 *
 * The mnemonic is written from JS via BreezMnemonicStoreModule and read natively by
 * ForegroundService, so the same class serves both sides and the format is only defined once.
 */
class BreezMnemonicStore(
    context: Context,
    private val keyName: String,
) {
    private val prefs = context.getSharedPreferences(SHARED_PREFERENCES_NAME, Context.MODE_PRIVATE)

    private val keyStore: KeyStore = KeyStore.getInstance(KEYSTORE_PROVIDER).apply { load(null) }

    private val keystoreAlias get() = "$KEYSTORE_ALIAS_PREFIX$keyName"

    /** Encrypts and persists the mnemonic, replacing any previous value. */
    fun put(mnemonic: String) {
        val cipher = Cipher.getInstance(TRANSFORMATION)
        cipher.init(Cipher.ENCRYPT_MODE, getOrCreateKey())

        val ciphertext = cipher.doFinal(mnemonic.toByteArray(Charsets.UTF_8))
        val encoded = "${cipher.iv.toBase64()}$SEPARATOR${ciphertext.toBase64()}"

        prefs.edit().putString(keyName, encoded).apply()
    }

    /**
     * Returns the stored mnemonic, or null if nothing is stored or the key is no longer usable.
     *
     * A null is expected rather than exceptional: the keystore entry is dropped when the device's
     * secure lock screen is removed, and the caller can only respond by asking the user to sign in
     * again, which the service cannot do on its own.
     */
    fun get(): String? {
        val encoded = prefs.getString(keyName, null) ?: return null

        val (iv, ciphertext) = encoded.split(SEPARATOR)
            .takeIf { it.size == 2 }
            ?.let { it[0].fromBase64() to it[1].fromBase64() }
            ?: return null

        val entry = keyStore.getEntry(keystoreAlias, null) as? KeyStore.SecretKeyEntry ?: return null

        return try {
            val cipher = Cipher.getInstance(TRANSFORMATION)
            cipher.init(Cipher.DECRYPT_MODE, entry.secretKey, GCMParameterSpec(TAG_LENGTH_BITS, iv))
            String(cipher.doFinal(ciphertext), Charsets.UTF_8)
        } catch (_: KeyPermanentlyInvalidatedException) {
            null
        } catch (_: GeneralSecurityException) {
            null
        }
    }

    /** Removes the mnemonic and the key that protects it. */
    fun delete() {
        prefs.edit().remove(keyName).apply()
        if (keyStore.containsAlias(keystoreAlias)) {
            keyStore.deleteEntry(keystoreAlias)
        }
    }

    private fun getOrCreateKey(): SecretKey {
        (keyStore.getEntry(keystoreAlias, null) as? KeyStore.SecretKeyEntry)?.let { return it.secretKey }

        val generator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, KEYSTORE_PROVIDER)
        generator.init(
            KeyGenParameterSpec
                .Builder(keystoreAlias, KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT)
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setKeySize(KEY_SIZE_BITS)
                // The service runs with no user present, so it must decrypt without authentication.
                .setUserAuthenticationRequired(false)
                .build(),
        )
        return generator.generateKey()
    }

    private fun ByteArray.toBase64(): String = Base64.encodeToString(this, Base64.NO_WRAP)

    private fun String.fromBase64(): ByteArray = Base64.decode(this, Base64.NO_WRAP)

    companion object {
        private const val KEYSTORE_PROVIDER = "AndroidKeyStore"
        private const val SHARED_PREFERENCES_NAME = "BreezSdkLiquidMnemonicStore"
        private const val KEYSTORE_ALIAS_PREFIX = "breez_sdk_liquid_mnemonic."
        private const val TRANSFORMATION = "AES/GCM/NoPadding"
        private const val TAG_LENGTH_BITS = 128
        private const val KEY_SIZE_BITS = 256
        private const val SEPARATOR = ":"
    }
}
