package com.breezliquidsdk

import breez_liquid_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
{% import "macros.kt" as kt %}

class BreezLiquidSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var bindingLiquidSdk: BindingLiquidSdk? = null

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
    fun getBindingLiquidSdk(): BindingLiquidSdk {
        if (bindingLiquidSdk != null) {
            return bindingLiquidSdk!!
        }

        throw LiquidSdkException.Generic("Not initialized")
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
    fun connect(req: ReadableMap, promise: Promise) {
        if (bindingLiquidSdk != null) {
            promise.reject("Generic", "Already initialized")
            return
        }

        executor.execute {
            try {
                var connectRequest = asConnectRequest(req) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "ConnectRequest")) }
                connectRequest.dataDir = connectRequest.dataDir?.takeUnless { it.isEmpty() } ?: run { reactApplicationContext.filesDir.toString() + "/breezLiquidSdk" }
                bindingLiquidSdk = connect(connectRequest)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun addEventListener(promise: Promise) {
        executor.execute {
            try {
                val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)
                var eventListener = BreezLiquidSDKEventListener(emitter)
                val res = getBindingLiquidSdk().addEventListener(eventListener)

                eventListener.setId(res)
                promise.resolve(res)
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }
    {%- include "Objects.kt" %}
}

