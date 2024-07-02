import Foundation
import BreezLiquidSDK

@objc(RNBreezLiquidSDK)
class RNBreezLiquidSDK: RCTEventEmitter {
    static let TAG: String = "BreezLiquidSDK"
    
    public static var emitter: RCTEventEmitter!
    public static var hasListeners: Bool = false
    public static var supportedEvents: [String] = ["breezLiquidSdkLog"]

    private var bindingLiquidSdk: BindingLiquidSdk!


    static var breezLiquidSdkDirectory: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let breezLiquidSdkDirectory = applicationDirectory.appendingPathComponent("breezLiquidSdk", isDirectory: true)
        
        if !FileManager.default.fileExists(atPath: breezLiquidSdkDirectory.path) {
            try! FileManager.default.createDirectory(atPath: breezLiquidSdkDirectory.path, withIntermediateDirectories: true)
        }
        
        return breezLiquidSdkDirectory
    }
    
    override init() {
        super.init()
        RNBreezLiquidSDK.emitter = self
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }

    static func addSupportedEvent(name: String) {
        RNBreezLiquidSDK.supportedEvents.append(name)
    }
    
    override func supportedEvents() -> [String]! {
        return RNBreezLiquidSDK.supportedEvents
    }
    
    override func startObserving() {
        RNBreezLiquidSDK.hasListeners = true
    }
    
    override func stopObserving() {
        RNBreezLiquidSDK.hasListeners = false
    }
    
    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }
    
    func getBindingLiquidSdk() throws -> BindingLiquidSdk {
        if bindingLiquidSdk != nil {
            return bindingLiquidSdk
        }
        
        throw LiquidSdkError.Generic(message: "Not initialized")
    }
        
    private func ensureWorkingDir(workingDir: String) throws {
        do {
            if !FileManager.default.fileExists(atPath: workingDir) {
                try FileManager.default.createDirectory(atPath: workingDir, withIntermediateDirectories: true)
            }
        } catch {
            throw LiquidSdkError.Generic(message: "Mandatory field workingDir must contain a writable directory")
        }
    }

    {% let obj_interface = "BreezLiquidSDK." -%}
    {% for func in ci.function_definitions() %}
    {%- if func.name()|ignored_function == false -%}
    {% include "TopLevelFunctionTemplate.swift" %}
    {% endif -%}
    {%- endfor %}  
    @objc(setLogger:reject:)
    func setLogger(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try BreezLiquidSDK.setLogger(logger: BreezLiquidSDKLogger())
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(connect:resolve:reject:)
    func connect(_ req:[String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        if bindingLiquidSdk != nil {
            reject("Generic", "Already initialized", nil)
            return
        }

        do {
            var connectRequest = try BreezLiquidSDKMapper.asConnectRequest(connectRequest: req)
            try ensureWorkingDir(workingDir: connectRequest.config.workingDir)

            bindingLiquidSdk = try BreezLiquidSDK.connect(req: connectRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
 
    @objc(addEventListener:reject:)
    func addEventListener(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var eventListener = BreezLiquidSDKEventListener()
            var res = try getBindingLiquidSdk().addEventListener(listener: eventListener)

            eventListener.setId(id: res)
            resolve(res)
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
