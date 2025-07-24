package com.breezsdkliquid_notifications

import android.content.Context
import android.security.keystore.KeyPermanentlyInvalidatedException
import expo.modules.kotlin.exception.CodedException
import expo.modules.securestore.SecureStoreOptions
import expo.modules.securestore.encryptors.AESEncryptor
import expo.modules.securestore.encryptors.HybridAESEncryptor
import expo.modules.securestore.encryptors.KeyBasedEncryptor
import org.json.JSONException
import org.json.JSONObject
import java.security.GeneralSecurityException
import java.security.KeyStore
import java.security.KeyStore.PrivateKeyEntry
import java.security.KeyStore.SecretKeyEntry
import javax.crypto.BadPaddingException
import expo.modules.securestore.AuthenticationHelper
import expo.modules.core.ModuleRegistry

const val SCHEME_PROPERTY = "scheme"
const val KEYSTORE_PROVIDER = "AndroidKeyStore"
const val SHARED_PREFERENCES_NAME = "SecureStore"

class SecureStoreHelper(val context: Context) {
    private val mAESEncryptor = AESEncryptor()
    private val keyStore: KeyStore = KeyStore.getInstance(KEYSTORE_PROVIDER)
    private val hybridAESEncryptor: HybridAESEncryptor = HybridAESEncryptor(context, mAESEncryptor)
    // Authentication Helper will in theory never been used since it's not available in a Service outside React Native Context.
    // To avoid rewriting the Helper itself we pass it a shell ModuleRegistry
    private val authenticationHelper: AuthenticationHelper = AuthenticationHelper(context, ModuleRegistry(emptyList(), emptyList()))

    private val prefs = context.getSharedPreferences(SHARED_PREFERENCES_NAME, Context.MODE_PRIVATE)
    private val options = SecureStoreOptions() // Default values serve us well (No authentication, default keychain)

    init {
        keyStore.load(null)
    }

    private fun <E : KeyStore.Entry> getKeyEntry(
        keyStoreEntryClass: Class<E>,
        encryptor: KeyBasedEncryptor<E>,
        options: SecureStoreOptions
    ): E? {
        val keystoreAlias = encryptor.getExtendedKeyStoreAlias(options, false)
        return if (keyStore.containsAlias(keystoreAlias)) {
            val entry = keyStore.getEntry(keystoreAlias, null)
            if (!keyStoreEntryClass.isInstance(entry)) {
                throw CodedException("The entry for the keystore alias \"$keystoreAlias\" is not a ${keyStoreEntryClass.simpleName}")
            }
            keyStoreEntryClass.cast(entry)
                ?: throw CodedException("The entry for the keystore alias \"$keystoreAlias\" couldn't be cast to correct class")
        } else {
            null
        }
    }

    private fun createKeychainAwareKey(key: String, keychainService: String): String {
        return "$keychainService-$key"
    }

    suspend fun getItem(key: String): String? {
        val keychainAwareKey = createKeychainAwareKey(key, options.keychainService)

        val encryptedItemString = prefs.getString(keychainAwareKey, null)

        encryptedItemString ?: return null

        val encryptedItem: JSONObject = try {
            JSONObject(encryptedItemString)
        } catch (e: JSONException) {
            throw CodedException("Could not parse the encrypted JSON item in SecureStore: ${e.message}")
        }

        val scheme = encryptedItem.optString(SCHEME_PROPERTY).takeIf { it.isNotEmpty() }
            ?: throw CodedException("Could not find the encryption scheme used for key: $key")

        try {
            when (scheme) {
                AESEncryptor.NAME -> {
                    val secretKeyEntry = getKeyEntry(SecretKeyEntry::class.java, mAESEncryptor, options) ?: run {
                        return null
                    }
                    return mAESEncryptor.decryptItem(key, encryptedItem, secretKeyEntry, options, authenticationHelper)
                }
                HybridAESEncryptor.NAME -> {
                    val privateKeyEntry = getKeyEntry(PrivateKeyEntry::class.java, hybridAESEncryptor, options)
                        ?: return null
                    return hybridAESEncryptor.decryptItem(key, encryptedItem, privateKeyEntry, options, authenticationHelper)
                }
                else -> {
                    throw CodedException("The item for key $key in SecureStore has an unknown encoding scheme $scheme)")
                }
            }
        } catch (e: KeyPermanentlyInvalidatedException) {
            return null
        } catch (e: BadPaddingException) {
            return null
        } catch (e: GeneralSecurityException) {
            throw (CodedException(e.message))
        } catch (e: CodedException) {
            throw e
        } catch (e: Exception) {
            throw (CodedException(e.message))
        }
    }
}
