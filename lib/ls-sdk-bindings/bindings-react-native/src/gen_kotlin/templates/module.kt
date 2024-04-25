package com.lssdk

import ls_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
{% import "macros.kt" as kt %}

class LiquidSwapSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var bindingWallet: BindingWallet? = null

    companion object {
        const val TAG = "RNLiquidSwapSDK"
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

    {% let obj_interface = "" -%}
    {% for func in ci.function_definitions() %}
    {%- if func.name()|ignored_function == false -%}
    {% include "TopLevelFunctionTemplate.kt" %}
    {% endif -%}
    {%- endfor %}  

    @ReactMethod
    fun initBindingWallet(mnemonic: String, dataDir: String, network: String, promise: Promise) {
        if (bindingWallet != null) {
            promise.reject("Generic", "Already initialized")
            return
        }

        executor.execute {
            try {
                val dataDirTmp = dataDir.takeUnless { it.isEmpty() } ?: run { reactApplicationContext.filesDir.toString() + "/lsSdk" }
                val networkTmp = asNetwork(network)
                bindingWallet = init(mnemonic, dataDirTmp, networkTmp)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }
    {%- include "Objects.kt" %}
}

