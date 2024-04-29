import Foundation
import LiquidSwapSDK

@objc(RNLiquidSwapSDK)
class RNLiquidSwapSDK: RCTEventEmitter {
    static let TAG: String = "LiquidSwapSDK"
    
    public static var emitter: RCTEventEmitter!
    public static var hasListeners: Bool = false

    private var bindingWallet: BindingWallet!

    static var defaultDataDir: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        
        return applicationDirectory.appendingPathComponent("lsSdk", isDirectory: true)
    }
    
    override init() {
        super.init()
        RNLiquidSwapSDK.emitter = self
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }
    
    override func supportedEvents() -> [String]! {
        return []
    }
    
    override func startObserving() {
        RNLiquidSwapSDK.hasListeners = true
    }
    
    override func stopObserving() {
        RNLiquidSwapSDK.hasListeners = false
    }
    
    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }
    
    func getBindingWallet() throws -> BindingWallet {
        if bindingWallet != nil {
            return bindingWallet
        }
        
        throw LsSdkError.Generic(message: "Not initialized")
    }
    
    {% let obj_interface = "LiquidSwapSDK." -%}
    {% for func in ci.function_definitions() %}
    {%- if func.name()|ignored_function == false -%}
    {% include "TopLevelFunctionTemplate.swift" %}
    {% endif -%}
    {%- endfor %}  
    @objc(connect:dataDir:network:resolve:reject:)
    func connect(_ mnemonic: String, dataDir: String, network: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        if bindingWallet != nil {
            reject("Generic", "Already initialized", nil)
            return
        }

        do {
            let dataDirTmp = dataDir.isEmpty ? RNLiquidSwapSDK.defaultDataDir.path : dataDir
            let networkTmp = try LiquidSwapSDKMapper.asNetwork(network: network)
            bindingWallet = try LiquidSwapSDK.connect(mnemonic: mnemonic, dataDir: dataDirTmp, network: networkTmp)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    {%- include "Objects.swift" %}
    
    func rejectErr(err: Error, reject: @escaping RCTPromiseRejectBlock) {
        var errorName = "Generic"
        var message = "\(err)"
        if let errAssociated = Mirror(reflecting: err).children.first {
            errorName = errAssociated.label ?? errorName
            if let associatedMessage = Mirror(reflecting: errAssociated.value).children.first {
                message = associatedMessage.value as! String
            }
        }
        reject(errorName, message, err)
    }
}

{% import "macros.swift" as swift %}
