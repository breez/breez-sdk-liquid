package com.breezliquidsdk

import breez_liquid_sdk.*
import com.facebook.react.bridge.*
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

class BreezLiquidSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var bindingWallet: BindingWallet? = null

    companion object {
        const val TAG = "RNBreezLiquidSDK"
    }

    override fun initialize() {
        super.initialize()

        executor = Executors.newFixedThreadPool(3)
    }

    override fun getName(): String {
        return TAG
    }

    @Throws(LsSdkException::class)
    fun getBindingWallet(): BindingWallet {
        if (bindingWallet != null) {
            return bindingWallet!!
        }

        throw LsSdkException.Generic("Not initialized")
    }

    @ReactMethod
    fun addListener(eventName: String) {}

    @ReactMethod
    fun removeListeners(count: Int) {}

    @ReactMethod
    fun connect(
        mnemonic: String,
        dataDir: String,
        network: String,
        promise: Promise,
    ) {
        if (bindingWallet != null) {
            promise.reject("Generic", "Already initialized")
            return
        }

        executor.execute {
            try {
                val dataDirTmp = dataDir.takeUnless { it.isEmpty() } ?: run { reactApplicationContext.filesDir.toString() + "/breezLiquidSdk" }
                val networkTmp = asNetwork(network)
                bindingWallet = connect(mnemonic, dataDirTmp, networkTmp)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun getInfo(
        withScan: Boolean,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBindingWallet().getInfo(withScan)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareSendPayment(
        invoice: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBindingWallet().prepareSendPayment(invoice)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendPayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareSendResponse =
                    asPrepareSendResponse(req) ?: run {
                        throw LsSdkException.Generic(errMissingMandatoryField("req", "PrepareSendResponse"))
                    }
                val res = getBindingWallet().sendPayment(prepareSendResponse)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareReceivePayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareReceiveRequest =
                    asPrepareReceiveRequest(req) ?: run {
                        throw LsSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveRequest"))
                    }
                val res = getBindingWallet().prepareReceivePayment(prepareReceiveRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun receivePayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareReceiveResponse =
                    asPrepareReceiveResponse(req) ?: run {
                        throw LsSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveResponse"))
                    }
                val res = getBindingWallet().receivePayment(prepareReceiveResponse)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun backup(promise: Promise) {
        executor.execute {
            try {
                getBindingWallet().backup()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun restore(
        backupPath: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val backupPathTmp = backupPath.takeUnless { it.isEmpty() }
                getBindingWallet().restore(backupPathTmp)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }
}
