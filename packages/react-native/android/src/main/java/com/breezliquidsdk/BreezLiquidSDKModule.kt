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

    @Throws(LiquidSdkException::class)
    fun getBindingWallet(): BindingWallet {
        if (bindingWallet != null) {
            return bindingWallet!!
        }

        throw LiquidSdkException.Generic("Not initialized")
    }

    @ReactMethod
    fun addListener(eventName: String) {}

    @ReactMethod
    fun removeListeners(count: Int) {}

    @ReactMethod
    fun connect(
        req: ReadableMap,
        promise: Promise,
    ) {
        if (bindingWallet != null) {
            promise.reject("Generic", "Already initialized")
            return
        }

        executor.execute {
            try {
                var connectRequest =
                    asConnectRequest(
                        req,
                    ) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "ConnectRequest")) }
                connectRequest.dataDir = connectRequest.dataDir.takeUnless {
                    it.isEmpty()
                } ?: run { reactApplicationContext.filesDir.toString() + "/breezLiquidSdk" }
                bindingWallet = connect(connectRequest)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun getInfo(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val getInfoRequest =
                    asGetInfoRequest(
                        req,
                    ) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "GetInfoRequest")) }
                val res = getBindingWallet().getInfo(getInfoRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareSendPayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareSendRequest =
                    asPrepareSendRequest(req) ?: run {
                        throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareSendRequest"))
                    }
                val res = getBindingWallet().prepareSendPayment(prepareSendRequest)
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
                        throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareSendResponse"))
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
                        throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveRequest"))
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
                        throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveResponse"))
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
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val restoreRequest =
                    asRestoreRequest(
                        req,
                    ) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "RestoreRequest")) }
                getBindingWallet().restore(restoreRequest)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }
}
