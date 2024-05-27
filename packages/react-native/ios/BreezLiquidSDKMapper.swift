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
        guard let pendingSendSat = getInfoResponse["pendingSendSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingSendSat", typeName: "GetInfoResponse"))
        }
        guard let pendingReceiveSat = getInfoResponse["pendingReceiveSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingReceiveSat", typeName: "GetInfoResponse"))
        }
        guard let pubkey = getInfoResponse["pubkey"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "pubkey", typeName: "GetInfoResponse"))
        }

        return GetInfoResponse(
            balanceSat: balanceSat,
            pendingSendSat: pendingSendSat,
            pendingReceiveSat: pendingReceiveSat,
            pubkey: pubkey
        )
    }

    static func dictionaryOf(getInfoResponse: GetInfoResponse) -> [String: Any?] {
        return [
            "balanceSat": getInfoResponse.balanceSat,
            "pendingSendSat": getInfoResponse.pendingSendSat,
            "pendingReceiveSat": getInfoResponse.pendingReceiveSat,
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

    static func asPayment(payment: [String: Any?]) throws -> Payment {
        guard let txId = payment["txId"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "txId", typeName: "Payment"))
        }
        var swapId: String?
        if hasNonNilKey(data: payment, key: "swapId") {
            guard let swapIdTmp = payment["swapId"] as? String else {
                throw LiquidSdkError.Generic(message: errUnexpectedValue(fieldName: "swapId"))
            }
            swapId = swapIdTmp
        }
        guard let timestamp = payment["timestamp"] as? UInt32 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "timestamp", typeName: "Payment"))
        }
        guard let amountSat = payment["amountSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "Payment"))
        }
        var feesSat: UInt64?
        if hasNonNilKey(data: payment, key: "feesSat") {
            guard let feesSatTmp = payment["feesSat"] as? UInt64 else {
                throw LiquidSdkError.Generic(message: errUnexpectedValue(fieldName: "feesSat"))
            }
            feesSat = feesSatTmp
        }
        var preimage: String?
        if hasNonNilKey(data: payment, key: "preimage") {
            guard let preimageTmp = payment["preimage"] as? String else {
                throw LiquidSdkError.Generic(message: errUnexpectedValue(fieldName: "preimage"))
            }
            preimage = preimageTmp
        }
        var refundTxId: String?
        if hasNonNilKey(data: payment, key: "refundTxId") {
            guard let refundTxIdTmp = payment["refundTxId"] as? String else {
                throw LiquidSdkError.Generic(message: errUnexpectedValue(fieldName: "refundTxId"))
            }
            refundTxId = refundTxIdTmp
        }
        var refundTxAmountSat: UInt64?
        if hasNonNilKey(data: payment, key: "refundTxAmountSat") {
            guard let refundTxAmountSatTmp = payment["refundTxAmountSat"] as? UInt64 else {
                throw LiquidSdkError.Generic(message: errUnexpectedValue(fieldName: "refundTxAmountSat"))
            }
            refundTxAmountSat = refundTxAmountSatTmp
        }
        guard let paymentTypeTmp = payment["paymentType"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentType", typeName: "Payment"))
        }
        let paymentType = try asPaymentType(paymentType: paymentTypeTmp)

        guard let statusTmp = payment["status"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "status", typeName: "Payment"))
        }
        let status = try asPaymentState(paymentState: statusTmp)

        return Payment(
            txId: txId,
            swapId: swapId,
            timestamp: timestamp,
            amountSat: amountSat,
            feesSat: feesSat,
            preimage: preimage,
            refundTxId: refundTxId,
            refundTxAmountSat: refundTxAmountSat,
            paymentType: paymentType,
            status: status
        )
    }

    static func dictionaryOf(payment: Payment) -> [String: Any?] {
        return [
            "txId": payment.txId,
            "swapId": payment.swapId == nil ? nil : payment.swapId,
            "timestamp": payment.timestamp,
            "amountSat": payment.amountSat,
            "feesSat": payment.feesSat == nil ? nil : payment.feesSat,
            "preimage": payment.preimage == nil ? nil : payment.preimage,
            "refundTxId": payment.refundTxId == nil ? nil : payment.refundTxId,
            "refundTxAmountSat": payment.refundTxAmountSat == nil ? nil : payment.refundTxAmountSat,
            "paymentType": valueOf(paymentType: payment.paymentType),
            "status": valueOf(paymentState: payment.status),
        ]
    }

    static func asPaymentList(arr: [Any]) throws -> [Payment] {
        var list = [Payment]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var payment = try asPayment(payment: val)
                list.append(payment)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "Payment"))
            }
        }
        return list
    }

    static func arrayOf(paymentList: [Payment]) -> [Any] {
        return paymentList.map { v -> [String: Any?] in dictionaryOf(payment: v) }
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
        guard let payerAmountSat = prepareReceiveResponse["payerAmountSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceiveResponse"))
        }
        guard let feesSat = prepareReceiveResponse["feesSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareReceiveResponse"))
        }

        return PrepareReceiveResponse(
            payerAmountSat: payerAmountSat,
            feesSat: feesSat
        )
    }

    static func dictionaryOf(prepareReceiveResponse: PrepareReceiveResponse) -> [String: Any?] {
        return [
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
        guard let invoice = prepareSendResponse["invoice"] as? String else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "PrepareSendResponse"))
        }
        guard let feesSat = prepareSendResponse["feesSat"] as? UInt64 else {
            throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareSendResponse"))
        }

        return PrepareSendResponse(
            invoice: invoice,
            feesSat: feesSat
        )
    }

    static func dictionaryOf(prepareSendResponse: PrepareSendResponse) -> [String: Any?] {
        return [
            "invoice": prepareSendResponse.invoice,
            "feesSat": prepareSendResponse.feesSat,
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

    static func asLiquidSdkEvent(liquidSdkEvent: [String: Any?]) throws -> LiquidSdkEvent {
        let type = liquidSdkEvent["type"] as! String
        if type == "paymentFailed" {
            guard let detailsTmp = liquidSdkEvent["details"] as? [String: Any?] else {
                throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "LiquidSdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return LiquidSdkEvent.paymentFailed(details: _details)
        }
        if type == "paymentPending" {
            guard let detailsTmp = liquidSdkEvent["details"] as? [String: Any?] else {
                throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "LiquidSdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return LiquidSdkEvent.paymentPending(details: _details)
        }
        if type == "paymentRefunded" {
            guard let detailsTmp = liquidSdkEvent["details"] as? [String: Any?] else {
                throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "LiquidSdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return LiquidSdkEvent.paymentRefunded(details: _details)
        }
        if type == "paymentRefundPending" {
            guard let detailsTmp = liquidSdkEvent["details"] as? [String: Any?] else {
                throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "LiquidSdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return LiquidSdkEvent.paymentRefundPending(details: _details)
        }
        if type == "paymentSucceed" {
            guard let detailsTmp = liquidSdkEvent["details"] as? [String: Any?] else {
                throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "LiquidSdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return LiquidSdkEvent.paymentSucceed(details: _details)
        }
        if type == "paymentWaitingConfirmation" {
            guard let detailsTmp = liquidSdkEvent["details"] as? [String: Any?] else {
                throw LiquidSdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "LiquidSdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return LiquidSdkEvent.paymentWaitingConfirmation(details: _details)
        }
        if type == "synced" {
            return LiquidSdkEvent.synced
        }

        throw LiquidSdkError.Generic(message: "Unexpected type \(type) for enum LiquidSdkEvent")
    }

    static func dictionaryOf(liquidSdkEvent: LiquidSdkEvent) -> [String: Any?] {
        switch liquidSdkEvent {
        case let .paymentFailed(
            details
        ):
            return [
                "type": "paymentFailed",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentPending(
            details
        ):
            return [
                "type": "paymentPending",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentRefunded(
            details
        ):
            return [
                "type": "paymentRefunded",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentRefundPending(
            details
        ):
            return [
                "type": "paymentRefundPending",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentSucceed(
            details
        ):
            return [
                "type": "paymentSucceed",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentWaitingConfirmation(
            details
        ):
            return [
                "type": "paymentWaitingConfirmation",
                "details": dictionaryOf(payment: details),
            ]

        case .synced:
            return [
                "type": "synced",
            ]
        }
    }

    static func arrayOf(liquidSdkEventList: [LiquidSdkEvent]) -> [Any] {
        return liquidSdkEventList.map { v -> [String: Any?] in dictionaryOf(liquidSdkEvent: v) }
    }

    static func asLiquidSdkEventList(arr: [Any]) throws -> [LiquidSdkEvent] {
        var list = [LiquidSdkEvent]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var liquidSdkEvent = try asLiquidSdkEvent(liquidSdkEvent: val)
                list.append(liquidSdkEvent)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "LiquidSdkEvent"))
            }
        }
        return list
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

    static func asPaymentState(paymentState: String) throws -> PaymentState {
        switch paymentState {
        case "created":
            return PaymentState.created

        case "pending":
            return PaymentState.pending

        case "complete":
            return PaymentState.complete

        case "failed":
            return PaymentState.failed

        default: throw LiquidSdkError.Generic(message: "Invalid variant \(paymentState) for enum PaymentState")
        }
    }

    static func valueOf(paymentState: PaymentState) -> String {
        switch paymentState {
        case .created:
            return "created"

        case .pending:
            return "pending"

        case .complete:
            return "complete"

        case .failed:
            return "failed"
        }
    }

    static func arrayOf(paymentStateList: [PaymentState]) -> [String] {
        return paymentStateList.map { v -> String in valueOf(paymentState: v) }
    }

    static func asPaymentStateList(arr: [Any]) throws -> [PaymentState] {
        var list = [PaymentState]()
        for value in arr {
            if let val = value as? String {
                var paymentState = try asPaymentState(paymentState: val)
                list.append(paymentState)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "PaymentState"))
            }
        }
        return list
    }

    static func asPaymentType(paymentType: String) throws -> PaymentType {
        switch paymentType {
        case "receive":
            return PaymentType.receive

        case "send":
            return PaymentType.send

        default: throw LiquidSdkError.Generic(message: "Invalid variant \(paymentType) for enum PaymentType")
        }
    }

    static func valueOf(paymentType: PaymentType) -> String {
        switch paymentType {
        case .receive:
            return "receive"

        case .send:
            return "send"
        }
    }

    static func arrayOf(paymentTypeList: [PaymentType]) -> [String] {
        return paymentTypeList.map { v -> String in valueOf(paymentType: v) }
    }

    static func asPaymentTypeList(arr: [Any]) throws -> [PaymentType] {
        var list = [PaymentType]()
        for value in arr {
            if let val = value as? String {
                var paymentType = try asPaymentType(paymentType: val)
                list.append(paymentType)
            } else {
                throw LiquidSdkError.Generic(message: errUnexpectedType(typeName: "PaymentType"))
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
