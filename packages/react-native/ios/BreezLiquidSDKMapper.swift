import BreezLiquidSDK
import Foundation

enum BreezLiquidSDKMapper {
    static func asConnectRequest(connectRequest: [String: Any?]) throws -> ConnectRequest {
        guard let mnemonic = connectRequest["mnemonic"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "mnemonic", typeName: "ConnectRequest"))
        }
        guard let networkTmp = connectRequest["network"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "ConnectRequest"))
        }
        let network = try asNetwork(network: networkTmp)

        var dataDir: String?
        if hasNonNilKey(data: connectRequest, key: "dataDir") {
            guard let dataDirTmp = connectRequest["dataDir"] as? String else {
                throw LiquidSdkError.Generic(message: errUnexpectedValue(fieldName: "dataDir"))
            }
            dataDir = dataDirTmp
        }

        return ConnectRequest(
            mnemonic: mnemonic,
            network: network,
            dataDir: dataDir
        )
    }

    static func dictionaryOf(connectRequest: ConnectRequest) -> [String: Any?] {
        return [
            "mnemonic": connectRequest.mnemonic,
            "network": valueOf(network: connectRequest.network),
            "dataDir": connectRequest.dataDir == nil ? nil : connectRequest.dataDir,
        ]
    }

    static func asConnectRequestList(arr: [Any]) throws -> [ConnectRequest] {
        var list = [ConnectRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var connectRequest = try asConnectRequest(connectRequest: val)
                list.append(connectRequest)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "ConnectRequest"))
            }
        }
        return list
    }

    static func arrayOf(connectRequestList: [ConnectRequest]) -> [Any] {
        return connectRequestList.map { v -> [String: Any?] in dictionaryOf(connectRequest: v) }
    }

    static func asGetInfoRequest(getInfoRequest: [String: Any?]) throws -> GetInfoRequest {
        guard let withScan = getInfoRequest["withScan"] as? Bool else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "withScan", typeName: "GetInfoRequest"))
        }

        return GetInfoRequest(
            withScan: withScan)
    }

    static func dictionaryOf(getInfoRequest: GetInfoRequest) -> [String: Any?] {
        return [
            "withScan": getInfoRequest.withScan,
        ]
    }

    static func asGetInfoRequestList(arr: [Any]) throws -> [GetInfoRequest] {
        var list = [GetInfoRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var getInfoRequest = try asGetInfoRequest(getInfoRequest: val)
                list.append(getInfoRequest)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "GetInfoRequest"))
            }
        }
        return list
    }

    static func arrayOf(getInfoRequestList: [GetInfoRequest]) -> [Any] {
        return getInfoRequestList.map { v -> [String: Any?] in dictionaryOf(getInfoRequest: v) }
    }

    static func asGetInfoResponse(getInfoResponse: [String: Any?]) throws -> GetInfoResponse {
        guard let balanceSat = getInfoResponse["balanceSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "balanceSat", typeName: "GetInfoResponse"))
        }
        guard let pubkey = getInfoResponse["pubkey"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "pubkey", typeName: "GetInfoResponse"))
        }

        return GetInfoResponse(
            balanceSat: balanceSat,
            pubkey: pubkey
        )
    }

    static func dictionaryOf(getInfoResponse: GetInfoResponse) -> [String: Any?] {
        return [
            "balanceSat": getInfoResponse.balanceSat,
            "pubkey": getInfoResponse.pubkey,
        ]
    }

    static func asGetInfoResponseList(arr: [Any]) throws -> [GetInfoResponse] {
        var list = [GetInfoResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var getInfoResponse = try asGetInfoResponse(getInfoResponse: val)
                list.append(getInfoResponse)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "GetInfoResponse"))
            }
        }
        return list
    }

    static func arrayOf(getInfoResponseList: [GetInfoResponse]) -> [Any] {
        return getInfoResponseList.map { v -> [String: Any?] in dictionaryOf(getInfoResponse: v) }
    }

    static func asPrepareReceiveRequest(prepareReceiveRequest: [String: Any?]) throws -> PrepareReceiveRequest {
        guard let payerAmountSat = prepareReceiveRequest["payerAmountSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceiveRequest"))
        }

        return PrepareReceiveRequest(
            payerAmountSat: payerAmountSat)
    }

    static func dictionaryOf(prepareReceiveRequest: PrepareReceiveRequest) -> [String: Any?] {
        return [
            "payerAmountSat": prepareReceiveRequest.payerAmountSat,
        ]
    }

    static func asPrepareReceiveRequestList(arr: [Any]) throws -> [PrepareReceiveRequest] {
        var list = [PrepareReceiveRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceiveRequest = try asPrepareReceiveRequest(prepareReceiveRequest: val)
                list.append(prepareReceiveRequest)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveRequestList: [PrepareReceiveRequest]) -> [Any] {
        return prepareReceiveRequestList.map { v -> [String: Any?] in dictionaryOf(prepareReceiveRequest: v) }
    }

    static func asPrepareReceiveResponse(prepareReceiveResponse: [String: Any?]) throws -> PrepareReceiveResponse {
        guard let pairHash = prepareReceiveResponse["pairHash"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "pairHash", typeName: "PrepareReceiveResponse"))
        }
        guard let payerAmountSat = prepareReceiveResponse["payerAmountSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceiveResponse"))
        }
        guard let feesSat = prepareReceiveResponse["feesSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareReceiveResponse"))
        }

        return PrepareReceiveResponse(
            pairHash: pairHash,
            payerAmountSat: payerAmountSat,
            feesSat: feesSat
        )
    }

    static func dictionaryOf(prepareReceiveResponse: PrepareReceiveResponse) -> [String: Any?] {
        return [
            "pairHash": prepareReceiveResponse.pairHash,
            "payerAmountSat": prepareReceiveResponse.payerAmountSat,
            "feesSat": prepareReceiveResponse.feesSat,
        ]
    }

    static func asPrepareReceiveResponseList(arr: [Any]) throws -> [PrepareReceiveResponse] {
        var list = [PrepareReceiveResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceiveResponse = try asPrepareReceiveResponse(prepareReceiveResponse: val)
                list.append(prepareReceiveResponse)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveResponseList: [PrepareReceiveResponse]) -> [Any] {
        return prepareReceiveResponseList.map { v -> [String: Any?] in dictionaryOf(prepareReceiveResponse: v) }
    }

    static func asPrepareSendRequest(prepareSendRequest: [String: Any?]) throws -> PrepareSendRequest {
        guard let invoice = prepareSendRequest["invoice"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "PrepareSendRequest"))
        }

        return PrepareSendRequest(
            invoice: invoice)
    }

    static func dictionaryOf(prepareSendRequest: PrepareSendRequest) -> [String: Any?] {
        return [
            "invoice": prepareSendRequest.invoice,
        ]
    }

    static func asPrepareSendRequestList(arr: [Any]) throws -> [PrepareSendRequest] {
        var list = [PrepareSendRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareSendRequest = try asPrepareSendRequest(prepareSendRequest: val)
                list.append(prepareSendRequest)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "PrepareSendRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareSendRequestList: [PrepareSendRequest]) -> [Any] {
        return prepareSendRequestList.map { v -> [String: Any?] in dictionaryOf(prepareSendRequest: v) }
    }

    static func asPrepareSendResponse(prepareSendResponse: [String: Any?]) throws -> PrepareSendResponse {
        guard let id = prepareSendResponse["id"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "id", typeName: "PrepareSendResponse"))
        }
        guard let payerAmountSat = prepareSendResponse["payerAmountSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareSendResponse"))
        }
        guard let receiverAmountSat = prepareSendResponse["receiverAmountSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "PrepareSendResponse"))
        }
        guard let totalFees = prepareSendResponse["totalFees"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "totalFees", typeName: "PrepareSendResponse"))
        }
        guard let fundingAddress = prepareSendResponse["fundingAddress"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "fundingAddress", typeName: "PrepareSendResponse"))
        }
        guard let invoice = prepareSendResponse["invoice"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "PrepareSendResponse"))
        }

        return PrepareSendResponse(
            id: id,
            payerAmountSat: payerAmountSat,
            receiverAmountSat: receiverAmountSat,
            totalFees: totalFees,
            fundingAddress: fundingAddress,
            invoice: invoice
        )
    }

    static func dictionaryOf(prepareSendResponse: PrepareSendResponse) -> [String: Any?] {
        return [
            "id": prepareSendResponse.id,
            "payerAmountSat": prepareSendResponse.payerAmountSat,
            "receiverAmountSat": prepareSendResponse.receiverAmountSat,
            "totalFees": prepareSendResponse.totalFees,
            "fundingAddress": prepareSendResponse.fundingAddress,
            "invoice": prepareSendResponse.invoice,
        ]
    }

    static func asPrepareSendResponseList(arr: [Any]) throws -> [PrepareSendResponse] {
        var list = [PrepareSendResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareSendResponse = try asPrepareSendResponse(prepareSendResponse: val)
                list.append(prepareSendResponse)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "PrepareSendResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareSendResponseList: [PrepareSendResponse]) -> [Any] {
        return prepareSendResponseList.map { v -> [String: Any?] in dictionaryOf(prepareSendResponse: v) }
    }

    static func asReceivePaymentResponse(receivePaymentResponse: [String: Any?]) throws -> ReceivePaymentResponse {
        guard let id = receivePaymentResponse["id"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "id", typeName: "ReceivePaymentResponse"))
        }
        guard let invoice = receivePaymentResponse["invoice"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "ReceivePaymentResponse"))
        }

        return ReceivePaymentResponse(
            id: id,
            invoice: invoice
        )
    }

    static func dictionaryOf(receivePaymentResponse: ReceivePaymentResponse) -> [String: Any?] {
        return [
            "id": receivePaymentResponse.id,
            "invoice": receivePaymentResponse.invoice,
        ]
    }

    static func asReceivePaymentResponseList(arr: [Any]) throws -> [ReceivePaymentResponse] {
        var list = [ReceivePaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receivePaymentResponse = try asReceivePaymentResponse(receivePaymentResponse: val)
                list.append(receivePaymentResponse)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "ReceivePaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(receivePaymentResponseList: [ReceivePaymentResponse]) -> [Any] {
        return receivePaymentResponseList.map { v -> [String: Any?] in dictionaryOf(receivePaymentResponse: v) }
    }

    static func asRestoreRequest(restoreRequest: [String: Any?]) throws -> RestoreRequest {
        var backupPath: String?
        if hasNonNilKey(data: restoreRequest, key: "backupPath") {
            guard let backupPathTmp = restoreRequest["backupPath"] as? String else {
                throw LiquidSdkError.Generic(message: errUnexpectedValue(fieldName: "backupPath"))
            }
            backupPath = backupPathTmp
        }

        return RestoreRequest(
            backupPath: backupPath)
    }

    static func dictionaryOf(restoreRequest: RestoreRequest) -> [String: Any?] {
        return [
            "backupPath": restoreRequest.backupPath == nil ? nil : restoreRequest.backupPath,
        ]
    }

    static func asRestoreRequestList(arr: [Any]) throws -> [RestoreRequest] {
        var list = [RestoreRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var restoreRequest = try asRestoreRequest(restoreRequest: val)
                list.append(restoreRequest)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "RestoreRequest"))
            }
        }
        return list
    }

    static func arrayOf(restoreRequestList: [RestoreRequest]) -> [Any] {
        return restoreRequestList.map { v -> [String: Any?] in dictionaryOf(restoreRequest: v) }
    }

    static func asSendPaymentResponse(sendPaymentResponse: [String: Any?]) throws -> SendPaymentResponse {
        guard let txid = sendPaymentResponse["txid"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "txid", typeName: "SendPaymentResponse"))
        }

        return SendPaymentResponse(
            txid: txid)
    }

    static func dictionaryOf(sendPaymentResponse: SendPaymentResponse) -> [String: Any?] {
        return [
            "txid": sendPaymentResponse.txid,
        ]
    }

    static func asSendPaymentResponseList(arr: [Any]) throws -> [SendPaymentResponse] {
        var list = [SendPaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sendPaymentResponse = try asSendPaymentResponse(sendPaymentResponse: val)
                list.append(sendPaymentResponse)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "SendPaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(sendPaymentResponseList: [SendPaymentResponse]) -> [Any] {
        return sendPaymentResponseList.map { v -> [String: Any?] in dictionaryOf(sendPaymentResponse: v) }
    }

    static func asNetwork(network: String) throws -> Network {
        switch network {
        case "liquid":
            return Network.liquid

        case "liquidTestnet":
            return Network.liquidTestnet

        default: throw LiquidSdkError.Generic(message: "Invalid variant \(network) for enum Network")
        }
    }

    static func valueOf(network: Network) -> String {
        switch network {
        case .liquid:
            return "liquid"

        case .liquidTestnet:
            return "liquidTestnet"
        }
    }

    static func arrayOf(networkList: [Network]) -> [String] {
        return networkList.map { v -> String in valueOf(network: v) }
    }

    static func asNetworkList(arr: [Any]) throws -> [Network] {
        var list = [Network]()
        for value in arr {
            if let val = value as? String {
                var network = try asNetwork(network: val)
                list.append(network)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "Network"))
            }
        }
        return list
    }

    static func hasNonNilKey(data: [String: Any?], key: String) -> Bool {
        if let val = data[key] {
            return !(val == nil || val is NSNull)
        }

        return false
    }

    static func errMissingMandatoryField(fieldName: String, typeName: String) -> String {
        return "Missing mandatory field \(fieldName) for type \(typeName)"
    }

    static func errUnexpectedType(typeName: String) -> String {
        return "Unexpected type \(typeName)"
    }

    static func errUnexpectedValue(fieldName: String) -> String {
        return "Unexpected value for optional field \(fieldName)"
    }
}
