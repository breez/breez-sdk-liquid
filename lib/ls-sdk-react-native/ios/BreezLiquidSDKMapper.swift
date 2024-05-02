import BreezLiquidSDK
import Foundation

enum BreezLiquidSDKMapper {
    static func asPrepareReceiveRequest(prepareReceiveRequest: [String: Any?]) throws -> PrepareReceiveRequest {
        guard let payerAmountSat = prepareReceiveRequest["payerAmountSat"] as? UInt64 else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceiveRequest"))
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
                throw LsSdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveRequestList: [PrepareReceiveRequest]) -> [Any] {
        return prepareReceiveRequestList.map { v -> [String: Any?] in dictionaryOf(prepareReceiveRequest: v) }
    }

    static func asPrepareReceiveResponse(prepareReceiveResponse: [String: Any?]) throws -> PrepareReceiveResponse {
        guard let pairHash = prepareReceiveResponse["pairHash"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "pairHash", typeName: "PrepareReceiveResponse"))
        }
        guard let payerAmountSat = prepareReceiveResponse["payerAmountSat"] as? UInt64 else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceiveResponse"))
        }
        guard let feesSat = prepareReceiveResponse["feesSat"] as? UInt64 else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareReceiveResponse"))
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
                throw LsSdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveResponseList: [PrepareReceiveResponse]) -> [Any] {
        return prepareReceiveResponseList.map { v -> [String: Any?] in dictionaryOf(prepareReceiveResponse: v) }
    }

    static func asPrepareSendResponse(prepareSendResponse: [String: Any?]) throws -> PrepareSendResponse {
        guard let id = prepareSendResponse["id"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "id", typeName: "PrepareSendResponse"))
        }
        guard let payerAmountSat = prepareSendResponse["payerAmountSat"] as? UInt64 else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareSendResponse"))
        }
        guard let receiverAmountSat = prepareSendResponse["receiverAmountSat"] as? UInt64 else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "PrepareSendResponse"))
        }
        guard let totalFees = prepareSendResponse["totalFees"] as? UInt64 else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "totalFees", typeName: "PrepareSendResponse"))
        }
        guard let fundingAddress = prepareSendResponse["fundingAddress"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "fundingAddress", typeName: "PrepareSendResponse"))
        }
        guard let invoice = prepareSendResponse["invoice"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "PrepareSendResponse"))
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
                throw LsSdkError.Generic(message: errUnexpectedType(typeName: "PrepareSendResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareSendResponseList: [PrepareSendResponse]) -> [Any] {
        return prepareSendResponseList.map { v -> [String: Any?] in dictionaryOf(prepareSendResponse: v) }
    }

    static func asReceivePaymentResponse(receivePaymentResponse: [String: Any?]) throws -> ReceivePaymentResponse {
        guard let id = receivePaymentResponse["id"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "id", typeName: "ReceivePaymentResponse"))
        }
        guard let invoice = receivePaymentResponse["invoice"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "ReceivePaymentResponse"))
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
                throw LsSdkError.Generic(message: errUnexpectedType(typeName: "ReceivePaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(receivePaymentResponseList: [ReceivePaymentResponse]) -> [Any] {
        return receivePaymentResponseList.map { v -> [String: Any?] in dictionaryOf(receivePaymentResponse: v) }
    }

    static func asSendPaymentResponse(sendPaymentResponse: [String: Any?]) throws -> SendPaymentResponse {
        guard let txid = sendPaymentResponse["txid"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "txid", typeName: "SendPaymentResponse"))
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
                throw LsSdkError.Generic(message: errUnexpectedType(typeName: "SendPaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(sendPaymentResponseList: [SendPaymentResponse]) -> [Any] {
        return sendPaymentResponseList.map { v -> [String: Any?] in dictionaryOf(sendPaymentResponse: v) }
    }

    static func asWalletInfo(walletInfo: [String: Any?]) throws -> WalletInfo {
        guard let balanceSat = walletInfo["balanceSat"] as? UInt64 else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "balanceSat", typeName: "WalletInfo"))
        }
        guard let pubkey = walletInfo["pubkey"] as? String else {
            throw LsSdkError.Generic(message: errMissingMandatoryField(fieldName: "pubkey", typeName: "WalletInfo"))
        }

        return WalletInfo(
            balanceSat: balanceSat,
            pubkey: pubkey
        )
    }

    static func dictionaryOf(walletInfo: WalletInfo) -> [String: Any?] {
        return [
            "balanceSat": walletInfo.balanceSat,
            "pubkey": walletInfo.pubkey,
        ]
    }

    static func asWalletInfoList(arr: [Any]) throws -> [WalletInfo] {
        var list = [WalletInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var walletInfo = try asWalletInfo(walletInfo: val)
                list.append(walletInfo)
            } else {
                throw LsSdkError.Generic(message: errUnexpectedType(typeName: "WalletInfo"))
            }
        }
        return list
    }

    static func arrayOf(walletInfoList: [WalletInfo]) -> [Any] {
        return walletInfoList.map { v -> [String: Any?] in dictionaryOf(walletInfo: v) }
    }

    static func asNetwork(network: String) throws -> Network {
        switch network {
        case "liquid":
            return Network.liquid

        case "liquidTestnet":
            return Network.liquidTestnet

        default: throw LsSdkError.Generic(message: "Invalid variant \(network) for enum Network")
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
                throw LsSdkError.Generic(message: errUnexpectedType(typeName: "Network"))
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
