import BreezSDKLiquid
import Foundation

enum BreezSDKLiquidMapper {
    static func asAcceptPaymentProposedFeesRequest(acceptPaymentProposedFeesRequest: [String: Any?]) throws -> AcceptPaymentProposedFeesRequest {
        guard let responseTmp = acceptPaymentProposedFeesRequest["response"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "response", typeName: "AcceptPaymentProposedFeesRequest"))
        }
        let response = try asFetchPaymentProposedFeesResponse(fetchPaymentProposedFeesResponse: responseTmp)

        return AcceptPaymentProposedFeesRequest(response: response)
    }

    static func dictionaryOf(acceptPaymentProposedFeesRequest: AcceptPaymentProposedFeesRequest) -> [String: Any?] {
        return [
            "response": dictionaryOf(fetchPaymentProposedFeesResponse: acceptPaymentProposedFeesRequest.response),
        ]
    }

    static func asAcceptPaymentProposedFeesRequestList(arr: [Any]) throws -> [AcceptPaymentProposedFeesRequest] {
        var list = [AcceptPaymentProposedFeesRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var acceptPaymentProposedFeesRequest = try asAcceptPaymentProposedFeesRequest(acceptPaymentProposedFeesRequest: val)
                list.append(acceptPaymentProposedFeesRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AcceptPaymentProposedFeesRequest"))
            }
        }
        return list
    }

    static func arrayOf(acceptPaymentProposedFeesRequestList: [AcceptPaymentProposedFeesRequest]) -> [Any] {
        return acceptPaymentProposedFeesRequestList.map { v -> [String: Any?] in return dictionaryOf(acceptPaymentProposedFeesRequest: v) }
    }

    static func asAesSuccessActionData(aesSuccessActionData: [String: Any?]) throws -> AesSuccessActionData {
        guard let description = aesSuccessActionData["description"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "AesSuccessActionData"))
        }
        guard let ciphertext = aesSuccessActionData["ciphertext"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "ciphertext", typeName: "AesSuccessActionData"))
        }
        guard let iv = aesSuccessActionData["iv"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "iv", typeName: "AesSuccessActionData"))
        }

        return AesSuccessActionData(description: description, ciphertext: ciphertext, iv: iv)
    }

    static func dictionaryOf(aesSuccessActionData: AesSuccessActionData) -> [String: Any?] {
        return [
            "description": aesSuccessActionData.description,
            "ciphertext": aesSuccessActionData.ciphertext,
            "iv": aesSuccessActionData.iv,
        ]
    }

    static func asAesSuccessActionDataList(arr: [Any]) throws -> [AesSuccessActionData] {
        var list = [AesSuccessActionData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var aesSuccessActionData = try asAesSuccessActionData(aesSuccessActionData: val)
                list.append(aesSuccessActionData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AesSuccessActionData"))
            }
        }
        return list
    }

    static func arrayOf(aesSuccessActionDataList: [AesSuccessActionData]) -> [Any] {
        return aesSuccessActionDataList.map { v -> [String: Any?] in return dictionaryOf(aesSuccessActionData: v) }
    }

    static func asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: [String: Any?]) throws -> AesSuccessActionDataDecrypted {
        guard let description = aesSuccessActionDataDecrypted["description"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "AesSuccessActionDataDecrypted"))
        }
        guard let plaintext = aesSuccessActionDataDecrypted["plaintext"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "plaintext", typeName: "AesSuccessActionDataDecrypted"))
        }

        return AesSuccessActionDataDecrypted(description: description, plaintext: plaintext)
    }

    static func dictionaryOf(aesSuccessActionDataDecrypted: AesSuccessActionDataDecrypted) -> [String: Any?] {
        return [
            "description": aesSuccessActionDataDecrypted.description,
            "plaintext": aesSuccessActionDataDecrypted.plaintext,
        ]
    }

    static func asAesSuccessActionDataDecryptedList(arr: [Any]) throws -> [AesSuccessActionDataDecrypted] {
        var list = [AesSuccessActionDataDecrypted]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var aesSuccessActionDataDecrypted = try asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: val)
                list.append(aesSuccessActionDataDecrypted)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AesSuccessActionDataDecrypted"))
            }
        }
        return list
    }

    static func arrayOf(aesSuccessActionDataDecryptedList: [AesSuccessActionDataDecrypted]) -> [Any] {
        return aesSuccessActionDataDecryptedList.map { v -> [String: Any?] in return dictionaryOf(aesSuccessActionDataDecrypted: v) }
    }

    static func asAssetBalance(assetBalance: [String: Any?]) throws -> AssetBalance {
        guard let assetId = assetBalance["assetId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "assetId", typeName: "AssetBalance"))
        }
        guard let balanceSat = assetBalance["balanceSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "balanceSat", typeName: "AssetBalance"))
        }
        var name: String?
        if hasNonNilKey(data: assetBalance, key: "name") {
            guard let nameTmp = assetBalance["name"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "name"))
            }
            name = nameTmp
        }
        var ticker: String?
        if hasNonNilKey(data: assetBalance, key: "ticker") {
            guard let tickerTmp = assetBalance["ticker"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "ticker"))
            }
            ticker = tickerTmp
        }
        var balance: Double?
        if hasNonNilKey(data: assetBalance, key: "balance") {
            guard let balanceTmp = assetBalance["balance"] as? Double else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "balance"))
            }
            balance = balanceTmp
        }

        return AssetBalance(assetId: assetId, balanceSat: balanceSat, name: name, ticker: ticker, balance: balance)
    }

    static func dictionaryOf(assetBalance: AssetBalance) -> [String: Any?] {
        return [
            "assetId": assetBalance.assetId,
            "balanceSat": assetBalance.balanceSat,
            "name": assetBalance.name == nil ? nil : assetBalance.name,
            "ticker": assetBalance.ticker == nil ? nil : assetBalance.ticker,
            "balance": assetBalance.balance == nil ? nil : assetBalance.balance,
        ]
    }

    static func asAssetBalanceList(arr: [Any]) throws -> [AssetBalance] {
        var list = [AssetBalance]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var assetBalance = try asAssetBalance(assetBalance: val)
                list.append(assetBalance)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AssetBalance"))
            }
        }
        return list
    }

    static func arrayOf(assetBalanceList: [AssetBalance]) -> [Any] {
        return assetBalanceList.map { v -> [String: Any?] in return dictionaryOf(assetBalance: v) }
    }

    static func asAssetInfo(assetInfo: [String: Any?]) throws -> AssetInfo {
        guard let name = assetInfo["name"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "name", typeName: "AssetInfo"))
        }
        guard let ticker = assetInfo["ticker"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "ticker", typeName: "AssetInfo"))
        }
        guard let amount = assetInfo["amount"] as? Double else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amount", typeName: "AssetInfo"))
        }
        var fees: Double?
        if hasNonNilKey(data: assetInfo, key: "fees") {
            guard let feesTmp = assetInfo["fees"] as? Double else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "fees"))
            }
            fees = feesTmp
        }

        return AssetInfo(name: name, ticker: ticker, amount: amount, fees: fees)
    }

    static func dictionaryOf(assetInfo: AssetInfo) -> [String: Any?] {
        return [
            "name": assetInfo.name,
            "ticker": assetInfo.ticker,
            "amount": assetInfo.amount,
            "fees": assetInfo.fees == nil ? nil : assetInfo.fees,
        ]
    }

    static func asAssetInfoList(arr: [Any]) throws -> [AssetInfo] {
        var list = [AssetInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var assetInfo = try asAssetInfo(assetInfo: val)
                list.append(assetInfo)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AssetInfo"))
            }
        }
        return list
    }

    static func arrayOf(assetInfoList: [AssetInfo]) -> [Any] {
        return assetInfoList.map { v -> [String: Any?] in return dictionaryOf(assetInfo: v) }
    }

    static func asAssetMetadata(assetMetadata: [String: Any?]) throws -> AssetMetadata {
        guard let assetId = assetMetadata["assetId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "assetId", typeName: "AssetMetadata"))
        }
        guard let name = assetMetadata["name"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "name", typeName: "AssetMetadata"))
        }
        guard let ticker = assetMetadata["ticker"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "ticker", typeName: "AssetMetadata"))
        }
        guard let precision = assetMetadata["precision"] as? UInt8 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "precision", typeName: "AssetMetadata"))
        }
        var fiatId: String?
        if hasNonNilKey(data: assetMetadata, key: "fiatId") {
            guard let fiatIdTmp = assetMetadata["fiatId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "fiatId"))
            }
            fiatId = fiatIdTmp
        }

        return AssetMetadata(assetId: assetId, name: name, ticker: ticker, precision: precision, fiatId: fiatId)
    }

    static func dictionaryOf(assetMetadata: AssetMetadata) -> [String: Any?] {
        return [
            "assetId": assetMetadata.assetId,
            "name": assetMetadata.name,
            "ticker": assetMetadata.ticker,
            "precision": assetMetadata.precision,
            "fiatId": assetMetadata.fiatId == nil ? nil : assetMetadata.fiatId,
        ]
    }

    static func asAssetMetadataList(arr: [Any]) throws -> [AssetMetadata] {
        var list = [AssetMetadata]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var assetMetadata = try asAssetMetadata(assetMetadata: val)
                list.append(assetMetadata)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AssetMetadata"))
            }
        }
        return list
    }

    static func arrayOf(assetMetadataList: [AssetMetadata]) -> [Any] {
        return assetMetadataList.map { v -> [String: Any?] in return dictionaryOf(assetMetadata: v) }
    }

    static func asBackupRequest(backupRequest: [String: Any?]) throws -> BackupRequest {
        var backupPath: String?
        if hasNonNilKey(data: backupRequest, key: "backupPath") {
            guard let backupPathTmp = backupRequest["backupPath"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "backupPath"))
            }
            backupPath = backupPathTmp
        }

        return BackupRequest(backupPath: backupPath)
    }

    static func dictionaryOf(backupRequest: BackupRequest) -> [String: Any?] {
        return [
            "backupPath": backupRequest.backupPath == nil ? nil : backupRequest.backupPath,
        ]
    }

    static func asBackupRequestList(arr: [Any]) throws -> [BackupRequest] {
        var list = [BackupRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var backupRequest = try asBackupRequest(backupRequest: val)
                list.append(backupRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BackupRequest"))
            }
        }
        return list
    }

    static func arrayOf(backupRequestList: [BackupRequest]) -> [Any] {
        return backupRequestList.map { v -> [String: Any?] in return dictionaryOf(backupRequest: v) }
    }

    static func asBitcoinAddressData(bitcoinAddressData: [String: Any?]) throws -> BitcoinAddressData {
        guard let address = bitcoinAddressData["address"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "BitcoinAddressData"))
        }
        guard let networkTmp = bitcoinAddressData["network"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "BitcoinAddressData"))
        }
        let network = try asNetwork(network: networkTmp)

        var amountSat: UInt64?
        if hasNonNilKey(data: bitcoinAddressData, key: "amountSat") {
            guard let amountSatTmp = bitcoinAddressData["amountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "amountSat"))
            }
            amountSat = amountSatTmp
        }
        var label: String?
        if hasNonNilKey(data: bitcoinAddressData, key: "label") {
            guard let labelTmp = bitcoinAddressData["label"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "label"))
            }
            label = labelTmp
        }
        var message: String?
        if hasNonNilKey(data: bitcoinAddressData, key: "message") {
            guard let messageTmp = bitcoinAddressData["message"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "message"))
            }
            message = messageTmp
        }

        return BitcoinAddressData(address: address, network: network, amountSat: amountSat, label: label, message: message)
    }

    static func dictionaryOf(bitcoinAddressData: BitcoinAddressData) -> [String: Any?] {
        return [
            "address": bitcoinAddressData.address,
            "network": valueOf(network: bitcoinAddressData.network),
            "amountSat": bitcoinAddressData.amountSat == nil ? nil : bitcoinAddressData.amountSat,
            "label": bitcoinAddressData.label == nil ? nil : bitcoinAddressData.label,
            "message": bitcoinAddressData.message == nil ? nil : bitcoinAddressData.message,
        ]
    }

    static func asBitcoinAddressDataList(arr: [Any]) throws -> [BitcoinAddressData] {
        var list = [BitcoinAddressData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var bitcoinAddressData = try asBitcoinAddressData(bitcoinAddressData: val)
                list.append(bitcoinAddressData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BitcoinAddressData"))
            }
        }
        return list
    }

    static func arrayOf(bitcoinAddressDataList: [BitcoinAddressData]) -> [Any] {
        return bitcoinAddressDataList.map { v -> [String: Any?] in return dictionaryOf(bitcoinAddressData: v) }
    }

    static func asBlockchainInfo(blockchainInfo: [String: Any?]) throws -> BlockchainInfo {
        guard let liquidTip = blockchainInfo["liquidTip"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "liquidTip", typeName: "BlockchainInfo"))
        }
        guard let bitcoinTip = blockchainInfo["bitcoinTip"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bitcoinTip", typeName: "BlockchainInfo"))
        }

        return BlockchainInfo(liquidTip: liquidTip, bitcoinTip: bitcoinTip)
    }

    static func dictionaryOf(blockchainInfo: BlockchainInfo) -> [String: Any?] {
        return [
            "liquidTip": blockchainInfo.liquidTip,
            "bitcoinTip": blockchainInfo.bitcoinTip,
        ]
    }

    static func asBlockchainInfoList(arr: [Any]) throws -> [BlockchainInfo] {
        var list = [BlockchainInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var blockchainInfo = try asBlockchainInfo(blockchainInfo: val)
                list.append(blockchainInfo)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BlockchainInfo"))
            }
        }
        return list
    }

    static func arrayOf(blockchainInfoList: [BlockchainInfo]) -> [Any] {
        return blockchainInfoList.map { v -> [String: Any?] in return dictionaryOf(blockchainInfo: v) }
    }

    static func asBuyBitcoinRequest(buyBitcoinRequest: [String: Any?]) throws -> BuyBitcoinRequest {
        guard let prepareResponseTmp = buyBitcoinRequest["prepareResponse"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareResponse", typeName: "BuyBitcoinRequest"))
        }
        let prepareResponse = try asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: prepareResponseTmp)

        var redirectUrl: String?
        if hasNonNilKey(data: buyBitcoinRequest, key: "redirectUrl") {
            guard let redirectUrlTmp = buyBitcoinRequest["redirectUrl"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "redirectUrl"))
            }
            redirectUrl = redirectUrlTmp
        }

        return BuyBitcoinRequest(prepareResponse: prepareResponse, redirectUrl: redirectUrl)
    }

    static func dictionaryOf(buyBitcoinRequest: BuyBitcoinRequest) -> [String: Any?] {
        return [
            "prepareResponse": dictionaryOf(prepareBuyBitcoinResponse: buyBitcoinRequest.prepareResponse),
            "redirectUrl": buyBitcoinRequest.redirectUrl == nil ? nil : buyBitcoinRequest.redirectUrl,
        ]
    }

    static func asBuyBitcoinRequestList(arr: [Any]) throws -> [BuyBitcoinRequest] {
        var list = [BuyBitcoinRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var buyBitcoinRequest = try asBuyBitcoinRequest(buyBitcoinRequest: val)
                list.append(buyBitcoinRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BuyBitcoinRequest"))
            }
        }
        return list
    }

    static func arrayOf(buyBitcoinRequestList: [BuyBitcoinRequest]) -> [Any] {
        return buyBitcoinRequestList.map { v -> [String: Any?] in return dictionaryOf(buyBitcoinRequest: v) }
    }

    static func asCheckMessageRequest(checkMessageRequest: [String: Any?]) throws -> CheckMessageRequest {
        guard let message = checkMessageRequest["message"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "message", typeName: "CheckMessageRequest"))
        }
        guard let pubkey = checkMessageRequest["pubkey"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pubkey", typeName: "CheckMessageRequest"))
        }
        guard let signature = checkMessageRequest["signature"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "signature", typeName: "CheckMessageRequest"))
        }

        return CheckMessageRequest(message: message, pubkey: pubkey, signature: signature)
    }

    static func dictionaryOf(checkMessageRequest: CheckMessageRequest) -> [String: Any?] {
        return [
            "message": checkMessageRequest.message,
            "pubkey": checkMessageRequest.pubkey,
            "signature": checkMessageRequest.signature,
        ]
    }

    static func asCheckMessageRequestList(arr: [Any]) throws -> [CheckMessageRequest] {
        var list = [CheckMessageRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var checkMessageRequest = try asCheckMessageRequest(checkMessageRequest: val)
                list.append(checkMessageRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "CheckMessageRequest"))
            }
        }
        return list
    }

    static func arrayOf(checkMessageRequestList: [CheckMessageRequest]) -> [Any] {
        return checkMessageRequestList.map { v -> [String: Any?] in return dictionaryOf(checkMessageRequest: v) }
    }

    static func asCheckMessageResponse(checkMessageResponse: [String: Any?]) throws -> CheckMessageResponse {
        guard let isValid = checkMessageResponse["isValid"] as? Bool else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "isValid", typeName: "CheckMessageResponse"))
        }

        return CheckMessageResponse(isValid: isValid)
    }

    static func dictionaryOf(checkMessageResponse: CheckMessageResponse) -> [String: Any?] {
        return [
            "isValid": checkMessageResponse.isValid,
        ]
    }

    static func asCheckMessageResponseList(arr: [Any]) throws -> [CheckMessageResponse] {
        var list = [CheckMessageResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var checkMessageResponse = try asCheckMessageResponse(checkMessageResponse: val)
                list.append(checkMessageResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "CheckMessageResponse"))
            }
        }
        return list
    }

    static func arrayOf(checkMessageResponseList: [CheckMessageResponse]) -> [Any] {
        return checkMessageResponseList.map { v -> [String: Any?] in return dictionaryOf(checkMessageResponse: v) }
    }

    static func asConfig(config: [String: Any?]) throws -> Config {
        guard let liquidExplorerTmp = config["liquidExplorer"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "liquidExplorer", typeName: "Config"))
        }
        let liquidExplorer = try asBlockchainExplorer(blockchainExplorer: liquidExplorerTmp)

        guard let bitcoinExplorerTmp = config["bitcoinExplorer"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bitcoinExplorer", typeName: "Config"))
        }
        let bitcoinExplorer = try asBlockchainExplorer(blockchainExplorer: bitcoinExplorerTmp)

        guard let workingDir = config["workingDir"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "workingDir", typeName: "Config"))
        }
        guard let networkTmp = config["network"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "Config"))
        }
        let network = try asLiquidNetwork(liquidNetwork: networkTmp)

        guard let paymentTimeoutSec = config["paymentTimeoutSec"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentTimeoutSec", typeName: "Config"))
        }
        var syncServiceUrl: String?
        if hasNonNilKey(data: config, key: "syncServiceUrl") {
            guard let syncServiceUrlTmp = config["syncServiceUrl"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "syncServiceUrl"))
            }
            syncServiceUrl = syncServiceUrlTmp
        }
        var breezApiKey: String?
        if hasNonNilKey(data: config, key: "breezApiKey") {
            guard let breezApiKeyTmp = config["breezApiKey"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "breezApiKey"))
            }
            breezApiKey = breezApiKeyTmp
        }
        var zeroConfMaxAmountSat: UInt64?
        if hasNonNilKey(data: config, key: "zeroConfMaxAmountSat") {
            guard let zeroConfMaxAmountSatTmp = config["zeroConfMaxAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "zeroConfMaxAmountSat"))
            }
            zeroConfMaxAmountSat = zeroConfMaxAmountSatTmp
        }
        guard let useDefaultExternalInputParsers = config["useDefaultExternalInputParsers"] as? Bool else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "useDefaultExternalInputParsers", typeName: "Config"))
        }
        guard let useMagicRoutingHints = config["useMagicRoutingHints"] as? Bool else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "useMagicRoutingHints", typeName: "Config"))
        }
        var externalInputParsers: [ExternalInputParser]?
        if let externalInputParsersTmp = config["externalInputParsers"] as? [[String: Any?]] {
            externalInputParsers = try asExternalInputParserList(arr: externalInputParsersTmp)
        }

        var onchainFeeRateLeewaySat: UInt64?
        if hasNonNilKey(data: config, key: "onchainFeeRateLeewaySat") {
            guard let onchainFeeRateLeewaySatTmp = config["onchainFeeRateLeewaySat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "onchainFeeRateLeewaySat"))
            }
            onchainFeeRateLeewaySat = onchainFeeRateLeewaySatTmp
        }
        var assetMetadata: [AssetMetadata]?
        if let assetMetadataTmp = config["assetMetadata"] as? [[String: Any?]] {
            assetMetadata = try asAssetMetadataList(arr: assetMetadataTmp)
        }

        var sideswapApiKey: String?
        if hasNonNilKey(data: config, key: "sideswapApiKey") {
            guard let sideswapApiKeyTmp = config["sideswapApiKey"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "sideswapApiKey"))
            }
            sideswapApiKey = sideswapApiKeyTmp
        }

        return Config(liquidExplorer: liquidExplorer, bitcoinExplorer: bitcoinExplorer, workingDir: workingDir, network: network, paymentTimeoutSec: paymentTimeoutSec, syncServiceUrl: syncServiceUrl, breezApiKey: breezApiKey, zeroConfMaxAmountSat: zeroConfMaxAmountSat, useDefaultExternalInputParsers: useDefaultExternalInputParsers, useMagicRoutingHints: useMagicRoutingHints, externalInputParsers: externalInputParsers, onchainFeeRateLeewaySat: onchainFeeRateLeewaySat, assetMetadata: assetMetadata, sideswapApiKey: sideswapApiKey)
    }

    static func dictionaryOf(config: Config) -> [String: Any?] {
        return [
            "liquidExplorer": dictionaryOf(blockchainExplorer: config.liquidExplorer),
            "bitcoinExplorer": dictionaryOf(blockchainExplorer: config.bitcoinExplorer),
            "workingDir": config.workingDir,
            "network": valueOf(liquidNetwork: config.network),
            "paymentTimeoutSec": config.paymentTimeoutSec,
            "syncServiceUrl": config.syncServiceUrl == nil ? nil : config.syncServiceUrl,
            "breezApiKey": config.breezApiKey == nil ? nil : config.breezApiKey,
            "zeroConfMaxAmountSat": config.zeroConfMaxAmountSat == nil ? nil : config.zeroConfMaxAmountSat,
            "useDefaultExternalInputParsers": config.useDefaultExternalInputParsers,
            "useMagicRoutingHints": config.useMagicRoutingHints,
            "externalInputParsers": config.externalInputParsers == nil ? nil : arrayOf(externalInputParserList: config.externalInputParsers!),
            "onchainFeeRateLeewaySat": config.onchainFeeRateLeewaySat == nil ? nil : config.onchainFeeRateLeewaySat,
            "assetMetadata": config.assetMetadata == nil ? nil : arrayOf(assetMetadataList: config.assetMetadata!),
            "sideswapApiKey": config.sideswapApiKey == nil ? nil : config.sideswapApiKey,
        ]
    }

    static func asConfigList(arr: [Any]) throws -> [Config] {
        var list = [Config]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var config = try asConfig(config: val)
                list.append(config)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Config"))
            }
        }
        return list
    }

    static func arrayOf(configList: [Config]) -> [Any] {
        return configList.map { v -> [String: Any?] in return dictionaryOf(config: v) }
    }

    static func asConnectRequest(connectRequest: [String: Any?]) throws -> ConnectRequest {
        guard let configTmp = connectRequest["config"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "config", typeName: "ConnectRequest"))
        }
        let config = try asConfig(config: configTmp)

        var mnemonic: String?
        if hasNonNilKey(data: connectRequest, key: "mnemonic") {
            guard let mnemonicTmp = connectRequest["mnemonic"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "mnemonic"))
            }
            mnemonic = mnemonicTmp
        }
        var passphrase: String?
        if hasNonNilKey(data: connectRequest, key: "passphrase") {
            guard let passphraseTmp = connectRequest["passphrase"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "passphrase"))
            }
            passphrase = passphraseTmp
        }
        var seed: [UInt8]?
        if hasNonNilKey(data: connectRequest, key: "seed") {
            guard let seedTmp = connectRequest["seed"] as? [UInt8] else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "seed"))
            }
            seed = seedTmp
        }

        return ConnectRequest(config: config, mnemonic: mnemonic, passphrase: passphrase, seed: seed)
    }

    static func dictionaryOf(connectRequest: ConnectRequest) -> [String: Any?] {
        return [
            "config": dictionaryOf(config: connectRequest.config),
            "mnemonic": connectRequest.mnemonic == nil ? nil : connectRequest.mnemonic,
            "passphrase": connectRequest.passphrase == nil ? nil : connectRequest.passphrase,
            "seed": connectRequest.seed == nil ? nil : connectRequest.seed,
        ]
    }

    static func asConnectRequestList(arr: [Any]) throws -> [ConnectRequest] {
        var list = [ConnectRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var connectRequest = try asConnectRequest(connectRequest: val)
                list.append(connectRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ConnectRequest"))
            }
        }
        return list
    }

    static func arrayOf(connectRequestList: [ConnectRequest]) -> [Any] {
        return connectRequestList.map { v -> [String: Any?] in return dictionaryOf(connectRequest: v) }
    }

    static func asConnectWithSignerRequest(connectWithSignerRequest: [String: Any?]) throws -> ConnectWithSignerRequest {
        guard let configTmp = connectWithSignerRequest["config"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "config", typeName: "ConnectWithSignerRequest"))
        }
        let config = try asConfig(config: configTmp)

        return ConnectWithSignerRequest(config: config)
    }

    static func dictionaryOf(connectWithSignerRequest: ConnectWithSignerRequest) -> [String: Any?] {
        return [
            "config": dictionaryOf(config: connectWithSignerRequest.config),
        ]
    }

    static func asConnectWithSignerRequestList(arr: [Any]) throws -> [ConnectWithSignerRequest] {
        var list = [ConnectWithSignerRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var connectWithSignerRequest = try asConnectWithSignerRequest(connectWithSignerRequest: val)
                list.append(connectWithSignerRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ConnectWithSignerRequest"))
            }
        }
        return list
    }

    static func arrayOf(connectWithSignerRequestList: [ConnectWithSignerRequest]) -> [Any] {
        return connectWithSignerRequestList.map { v -> [String: Any?] in return dictionaryOf(connectWithSignerRequest: v) }
    }

    static func asCreateBolt12InvoiceRequest(createBolt12InvoiceRequest: [String: Any?]) throws -> CreateBolt12InvoiceRequest {
        guard let offer = createBolt12InvoiceRequest["offer"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "offer", typeName: "CreateBolt12InvoiceRequest"))
        }
        guard let invoiceRequest = createBolt12InvoiceRequest["invoiceRequest"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoiceRequest", typeName: "CreateBolt12InvoiceRequest"))
        }

        return CreateBolt12InvoiceRequest(offer: offer, invoiceRequest: invoiceRequest)
    }

    static func dictionaryOf(createBolt12InvoiceRequest: CreateBolt12InvoiceRequest) -> [String: Any?] {
        return [
            "offer": createBolt12InvoiceRequest.offer,
            "invoiceRequest": createBolt12InvoiceRequest.invoiceRequest,
        ]
    }

    static func asCreateBolt12InvoiceRequestList(arr: [Any]) throws -> [CreateBolt12InvoiceRequest] {
        var list = [CreateBolt12InvoiceRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var createBolt12InvoiceRequest = try asCreateBolt12InvoiceRequest(createBolt12InvoiceRequest: val)
                list.append(createBolt12InvoiceRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "CreateBolt12InvoiceRequest"))
            }
        }
        return list
    }

    static func arrayOf(createBolt12InvoiceRequestList: [CreateBolt12InvoiceRequest]) -> [Any] {
        return createBolt12InvoiceRequestList.map { v -> [String: Any?] in return dictionaryOf(createBolt12InvoiceRequest: v) }
    }

    static func asCreateBolt12InvoiceResponse(createBolt12InvoiceResponse: [String: Any?]) throws -> CreateBolt12InvoiceResponse {
        guard let invoice = createBolt12InvoiceResponse["invoice"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "CreateBolt12InvoiceResponse"))
        }

        return CreateBolt12InvoiceResponse(invoice: invoice)
    }

    static func dictionaryOf(createBolt12InvoiceResponse: CreateBolt12InvoiceResponse) -> [String: Any?] {
        return [
            "invoice": createBolt12InvoiceResponse.invoice,
        ]
    }

    static func asCreateBolt12InvoiceResponseList(arr: [Any]) throws -> [CreateBolt12InvoiceResponse] {
        var list = [CreateBolt12InvoiceResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var createBolt12InvoiceResponse = try asCreateBolt12InvoiceResponse(createBolt12InvoiceResponse: val)
                list.append(createBolt12InvoiceResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "CreateBolt12InvoiceResponse"))
            }
        }
        return list
    }

    static func arrayOf(createBolt12InvoiceResponseList: [CreateBolt12InvoiceResponse]) -> [Any] {
        return createBolt12InvoiceResponseList.map { v -> [String: Any?] in return dictionaryOf(createBolt12InvoiceResponse: v) }
    }

    static func asCurrencyInfo(currencyInfo: [String: Any?]) throws -> CurrencyInfo {
        guard let name = currencyInfo["name"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "name", typeName: "CurrencyInfo"))
        }
        guard let fractionSize = currencyInfo["fractionSize"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "fractionSize", typeName: "CurrencyInfo"))
        }
        var spacing: UInt32?
        if hasNonNilKey(data: currencyInfo, key: "spacing") {
            guard let spacingTmp = currencyInfo["spacing"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "spacing"))
            }
            spacing = spacingTmp
        }
        var symbol: Symbol?
        if let symbolTmp = currencyInfo["symbol"] as? [String: Any?] {
            symbol = try asSymbol(symbol: symbolTmp)
        }

        var uniqSymbol: Symbol?
        if let uniqSymbolTmp = currencyInfo["uniqSymbol"] as? [String: Any?] {
            uniqSymbol = try asSymbol(symbol: uniqSymbolTmp)
        }

        guard let localizedNameTmp = currencyInfo["localizedName"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "localizedName", typeName: "CurrencyInfo"))
        }
        let localizedName = try asLocalizedNameList(arr: localizedNameTmp)

        guard let localeOverridesTmp = currencyInfo["localeOverrides"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "localeOverrides", typeName: "CurrencyInfo"))
        }
        let localeOverrides = try asLocaleOverridesList(arr: localeOverridesTmp)

        return CurrencyInfo(name: name, fractionSize: fractionSize, spacing: spacing, symbol: symbol, uniqSymbol: uniqSymbol, localizedName: localizedName, localeOverrides: localeOverrides)
    }

    static func dictionaryOf(currencyInfo: CurrencyInfo) -> [String: Any?] {
        return [
            "name": currencyInfo.name,
            "fractionSize": currencyInfo.fractionSize,
            "spacing": currencyInfo.spacing == nil ? nil : currencyInfo.spacing,
            "symbol": currencyInfo.symbol == nil ? nil : dictionaryOf(symbol: currencyInfo.symbol!),
            "uniqSymbol": currencyInfo.uniqSymbol == nil ? nil : dictionaryOf(symbol: currencyInfo.uniqSymbol!),
            "localizedName": arrayOf(localizedNameList: currencyInfo.localizedName),
            "localeOverrides": arrayOf(localeOverridesList: currencyInfo.localeOverrides),
        ]
    }

    static func asCurrencyInfoList(arr: [Any]) throws -> [CurrencyInfo] {
        var list = [CurrencyInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var currencyInfo = try asCurrencyInfo(currencyInfo: val)
                list.append(currencyInfo)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "CurrencyInfo"))
            }
        }
        return list
    }

    static func arrayOf(currencyInfoList: [CurrencyInfo]) -> [Any] {
        return currencyInfoList.map { v -> [String: Any?] in return dictionaryOf(currencyInfo: v) }
    }

    static func asExternalInputParser(externalInputParser: [String: Any?]) throws -> ExternalInputParser {
        guard let providerId = externalInputParser["providerId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "providerId", typeName: "ExternalInputParser"))
        }
        guard let inputRegex = externalInputParser["inputRegex"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "inputRegex", typeName: "ExternalInputParser"))
        }
        guard let parserUrl = externalInputParser["parserUrl"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "parserUrl", typeName: "ExternalInputParser"))
        }

        return ExternalInputParser(providerId: providerId, inputRegex: inputRegex, parserUrl: parserUrl)
    }

    static func dictionaryOf(externalInputParser: ExternalInputParser) -> [String: Any?] {
        return [
            "providerId": externalInputParser.providerId,
            "inputRegex": externalInputParser.inputRegex,
            "parserUrl": externalInputParser.parserUrl,
        ]
    }

    static func asExternalInputParserList(arr: [Any]) throws -> [ExternalInputParser] {
        var list = [ExternalInputParser]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var externalInputParser = try asExternalInputParser(externalInputParser: val)
                list.append(externalInputParser)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ExternalInputParser"))
            }
        }
        return list
    }

    static func arrayOf(externalInputParserList: [ExternalInputParser]) -> [Any] {
        return externalInputParserList.map { v -> [String: Any?] in return dictionaryOf(externalInputParser: v) }
    }

    static func asFetchPaymentProposedFeesRequest(fetchPaymentProposedFeesRequest: [String: Any?]) throws -> FetchPaymentProposedFeesRequest {
        guard let swapId = fetchPaymentProposedFeesRequest["swapId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapId", typeName: "FetchPaymentProposedFeesRequest"))
        }

        return FetchPaymentProposedFeesRequest(swapId: swapId)
    }

    static func dictionaryOf(fetchPaymentProposedFeesRequest: FetchPaymentProposedFeesRequest) -> [String: Any?] {
        return [
            "swapId": fetchPaymentProposedFeesRequest.swapId,
        ]
    }

    static func asFetchPaymentProposedFeesRequestList(arr: [Any]) throws -> [FetchPaymentProposedFeesRequest] {
        var list = [FetchPaymentProposedFeesRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var fetchPaymentProposedFeesRequest = try asFetchPaymentProposedFeesRequest(fetchPaymentProposedFeesRequest: val)
                list.append(fetchPaymentProposedFeesRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "FetchPaymentProposedFeesRequest"))
            }
        }
        return list
    }

    static func arrayOf(fetchPaymentProposedFeesRequestList: [FetchPaymentProposedFeesRequest]) -> [Any] {
        return fetchPaymentProposedFeesRequestList.map { v -> [String: Any?] in return dictionaryOf(fetchPaymentProposedFeesRequest: v) }
    }

    static func asFetchPaymentProposedFeesResponse(fetchPaymentProposedFeesResponse: [String: Any?]) throws -> FetchPaymentProposedFeesResponse {
        guard let swapId = fetchPaymentProposedFeesResponse["swapId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapId", typeName: "FetchPaymentProposedFeesResponse"))
        }
        guard let feesSat = fetchPaymentProposedFeesResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "FetchPaymentProposedFeesResponse"))
        }
        guard let payerAmountSat = fetchPaymentProposedFeesResponse["payerAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "FetchPaymentProposedFeesResponse"))
        }
        guard let receiverAmountSat = fetchPaymentProposedFeesResponse["receiverAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "FetchPaymentProposedFeesResponse"))
        }

        return FetchPaymentProposedFeesResponse(swapId: swapId, feesSat: feesSat, payerAmountSat: payerAmountSat, receiverAmountSat: receiverAmountSat)
    }

    static func dictionaryOf(fetchPaymentProposedFeesResponse: FetchPaymentProposedFeesResponse) -> [String: Any?] {
        return [
            "swapId": fetchPaymentProposedFeesResponse.swapId,
            "feesSat": fetchPaymentProposedFeesResponse.feesSat,
            "payerAmountSat": fetchPaymentProposedFeesResponse.payerAmountSat,
            "receiverAmountSat": fetchPaymentProposedFeesResponse.receiverAmountSat,
        ]
    }

    static func asFetchPaymentProposedFeesResponseList(arr: [Any]) throws -> [FetchPaymentProposedFeesResponse] {
        var list = [FetchPaymentProposedFeesResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var fetchPaymentProposedFeesResponse = try asFetchPaymentProposedFeesResponse(fetchPaymentProposedFeesResponse: val)
                list.append(fetchPaymentProposedFeesResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "FetchPaymentProposedFeesResponse"))
            }
        }
        return list
    }

    static func arrayOf(fetchPaymentProposedFeesResponseList: [FetchPaymentProposedFeesResponse]) -> [Any] {
        return fetchPaymentProposedFeesResponseList.map { v -> [String: Any?] in return dictionaryOf(fetchPaymentProposedFeesResponse: v) }
    }

    static func asFiatCurrency(fiatCurrency: [String: Any?]) throws -> FiatCurrency {
        guard let id = fiatCurrency["id"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "id", typeName: "FiatCurrency"))
        }
        guard let infoTmp = fiatCurrency["info"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "info", typeName: "FiatCurrency"))
        }
        let info = try asCurrencyInfo(currencyInfo: infoTmp)

        return FiatCurrency(id: id, info: info)
    }

    static func dictionaryOf(fiatCurrency: FiatCurrency) -> [String: Any?] {
        return [
            "id": fiatCurrency.id,
            "info": dictionaryOf(currencyInfo: fiatCurrency.info),
        ]
    }

    static func asFiatCurrencyList(arr: [Any]) throws -> [FiatCurrency] {
        var list = [FiatCurrency]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var fiatCurrency = try asFiatCurrency(fiatCurrency: val)
                list.append(fiatCurrency)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "FiatCurrency"))
            }
        }
        return list
    }

    static func arrayOf(fiatCurrencyList: [FiatCurrency]) -> [Any] {
        return fiatCurrencyList.map { v -> [String: Any?] in return dictionaryOf(fiatCurrency: v) }
    }

    static func asGetInfoResponse(getInfoResponse: [String: Any?]) throws -> GetInfoResponse {
        guard let walletInfoTmp = getInfoResponse["walletInfo"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "walletInfo", typeName: "GetInfoResponse"))
        }
        let walletInfo = try asWalletInfo(walletInfo: walletInfoTmp)

        guard let blockchainInfoTmp = getInfoResponse["blockchainInfo"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "blockchainInfo", typeName: "GetInfoResponse"))
        }
        let blockchainInfo = try asBlockchainInfo(blockchainInfo: blockchainInfoTmp)

        return GetInfoResponse(walletInfo: walletInfo, blockchainInfo: blockchainInfo)
    }

    static func dictionaryOf(getInfoResponse: GetInfoResponse) -> [String: Any?] {
        return [
            "walletInfo": dictionaryOf(walletInfo: getInfoResponse.walletInfo),
            "blockchainInfo": dictionaryOf(blockchainInfo: getInfoResponse.blockchainInfo),
        ]
    }

    static func asGetInfoResponseList(arr: [Any]) throws -> [GetInfoResponse] {
        var list = [GetInfoResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var getInfoResponse = try asGetInfoResponse(getInfoResponse: val)
                list.append(getInfoResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "GetInfoResponse"))
            }
        }
        return list
    }

    static func arrayOf(getInfoResponseList: [GetInfoResponse]) -> [Any] {
        return getInfoResponseList.map { v -> [String: Any?] in return dictionaryOf(getInfoResponse: v) }
    }

    static func asLnInvoice(lnInvoice: [String: Any?]) throws -> LnInvoice {
        guard let bolt11 = lnInvoice["bolt11"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bolt11", typeName: "LnInvoice"))
        }
        guard let networkTmp = lnInvoice["network"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "LnInvoice"))
        }
        let network = try asNetwork(network: networkTmp)

        guard let payeePubkey = lnInvoice["payeePubkey"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payeePubkey", typeName: "LnInvoice"))
        }
        guard let paymentHash = lnInvoice["paymentHash"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentHash", typeName: "LnInvoice"))
        }
        var description: String?
        if hasNonNilKey(data: lnInvoice, key: "description") {
            guard let descriptionTmp = lnInvoice["description"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "description"))
            }
            description = descriptionTmp
        }
        var descriptionHash: String?
        if hasNonNilKey(data: lnInvoice, key: "descriptionHash") {
            guard let descriptionHashTmp = lnInvoice["descriptionHash"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "descriptionHash"))
            }
            descriptionHash = descriptionHashTmp
        }
        var amountMsat: UInt64?
        if hasNonNilKey(data: lnInvoice, key: "amountMsat") {
            guard let amountMsatTmp = lnInvoice["amountMsat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "amountMsat"))
            }
            amountMsat = amountMsatTmp
        }
        guard let timestamp = lnInvoice["timestamp"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "timestamp", typeName: "LnInvoice"))
        }
        guard let expiry = lnInvoice["expiry"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "expiry", typeName: "LnInvoice"))
        }
        guard let routingHintsTmp = lnInvoice["routingHints"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "routingHints", typeName: "LnInvoice"))
        }
        let routingHints = try asRouteHintList(arr: routingHintsTmp)

        guard let paymentSecret = lnInvoice["paymentSecret"] as? [UInt8] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentSecret", typeName: "LnInvoice"))
        }
        guard let minFinalCltvExpiryDelta = lnInvoice["minFinalCltvExpiryDelta"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minFinalCltvExpiryDelta", typeName: "LnInvoice"))
        }

        return LnInvoice(bolt11: bolt11, network: network, payeePubkey: payeePubkey, paymentHash: paymentHash, description: description, descriptionHash: descriptionHash, amountMsat: amountMsat, timestamp: timestamp, expiry: expiry, routingHints: routingHints, paymentSecret: paymentSecret, minFinalCltvExpiryDelta: minFinalCltvExpiryDelta)
    }

    static func dictionaryOf(lnInvoice: LnInvoice) -> [String: Any?] {
        return [
            "bolt11": lnInvoice.bolt11,
            "network": valueOf(network: lnInvoice.network),
            "payeePubkey": lnInvoice.payeePubkey,
            "paymentHash": lnInvoice.paymentHash,
            "description": lnInvoice.description == nil ? nil : lnInvoice.description,
            "descriptionHash": lnInvoice.descriptionHash == nil ? nil : lnInvoice.descriptionHash,
            "amountMsat": lnInvoice.amountMsat == nil ? nil : lnInvoice.amountMsat,
            "timestamp": lnInvoice.timestamp,
            "expiry": lnInvoice.expiry,
            "routingHints": arrayOf(routeHintList: lnInvoice.routingHints),
            "paymentSecret": lnInvoice.paymentSecret,
            "minFinalCltvExpiryDelta": lnInvoice.minFinalCltvExpiryDelta,
        ]
    }

    static func asLnInvoiceList(arr: [Any]) throws -> [LnInvoice] {
        var list = [LnInvoice]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnInvoice = try asLnInvoice(lnInvoice: val)
                list.append(lnInvoice)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnInvoice"))
            }
        }
        return list
    }

    static func arrayOf(lnInvoiceList: [LnInvoice]) -> [Any] {
        return lnInvoiceList.map { v -> [String: Any?] in return dictionaryOf(lnInvoice: v) }
    }

    static func asLnOffer(lnOffer: [String: Any?]) throws -> LnOffer {
        guard let offer = lnOffer["offer"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "offer", typeName: "LnOffer"))
        }
        guard let chains = lnOffer["chains"] as? [String] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "chains", typeName: "LnOffer"))
        }
        guard let pathsTmp = lnOffer["paths"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paths", typeName: "LnOffer"))
        }
        let paths = try asLnOfferBlindedPathList(arr: pathsTmp)

        var description: String?
        if hasNonNilKey(data: lnOffer, key: "description") {
            guard let descriptionTmp = lnOffer["description"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "description"))
            }
            description = descriptionTmp
        }
        var signingPubkey: String?
        if hasNonNilKey(data: lnOffer, key: "signingPubkey") {
            guard let signingPubkeyTmp = lnOffer["signingPubkey"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "signingPubkey"))
            }
            signingPubkey = signingPubkeyTmp
        }
        var minAmount: Amount?
        if let minAmountTmp = lnOffer["minAmount"] as? [String: Any?] {
            minAmount = try asAmount(amount: minAmountTmp)
        }

        var absoluteExpiry: UInt64?
        if hasNonNilKey(data: lnOffer, key: "absoluteExpiry") {
            guard let absoluteExpiryTmp = lnOffer["absoluteExpiry"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "absoluteExpiry"))
            }
            absoluteExpiry = absoluteExpiryTmp
        }
        var issuer: String?
        if hasNonNilKey(data: lnOffer, key: "issuer") {
            guard let issuerTmp = lnOffer["issuer"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "issuer"))
            }
            issuer = issuerTmp
        }

        return LnOffer(offer: offer, chains: chains, paths: paths, description: description, signingPubkey: signingPubkey, minAmount: minAmount, absoluteExpiry: absoluteExpiry, issuer: issuer)
    }

    static func dictionaryOf(lnOffer: LnOffer) -> [String: Any?] {
        return [
            "offer": lnOffer.offer,
            "chains": lnOffer.chains,
            "paths": arrayOf(lnOfferBlindedPathList: lnOffer.paths),
            "description": lnOffer.description == nil ? nil : lnOffer.description,
            "signingPubkey": lnOffer.signingPubkey == nil ? nil : lnOffer.signingPubkey,
            "minAmount": lnOffer.minAmount == nil ? nil : dictionaryOf(amount: lnOffer.minAmount!),
            "absoluteExpiry": lnOffer.absoluteExpiry == nil ? nil : lnOffer.absoluteExpiry,
            "issuer": lnOffer.issuer == nil ? nil : lnOffer.issuer,
        ]
    }

    static func asLnOfferList(arr: [Any]) throws -> [LnOffer] {
        var list = [LnOffer]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnOffer = try asLnOffer(lnOffer: val)
                list.append(lnOffer)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnOffer"))
            }
        }
        return list
    }

    static func arrayOf(lnOfferList: [LnOffer]) -> [Any] {
        return lnOfferList.map { v -> [String: Any?] in return dictionaryOf(lnOffer: v) }
    }

    static func asLightningPaymentLimitsResponse(lightningPaymentLimitsResponse: [String: Any?]) throws -> LightningPaymentLimitsResponse {
        guard let sendTmp = lightningPaymentLimitsResponse["send"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "send", typeName: "LightningPaymentLimitsResponse"))
        }
        let send = try asLimits(limits: sendTmp)

        guard let receiveTmp = lightningPaymentLimitsResponse["receive"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receive", typeName: "LightningPaymentLimitsResponse"))
        }
        let receive = try asLimits(limits: receiveTmp)

        return LightningPaymentLimitsResponse(send: send, receive: receive)
    }

    static func dictionaryOf(lightningPaymentLimitsResponse: LightningPaymentLimitsResponse) -> [String: Any?] {
        return [
            "send": dictionaryOf(limits: lightningPaymentLimitsResponse.send),
            "receive": dictionaryOf(limits: lightningPaymentLimitsResponse.receive),
        ]
    }

    static func asLightningPaymentLimitsResponseList(arr: [Any]) throws -> [LightningPaymentLimitsResponse] {
        var list = [LightningPaymentLimitsResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lightningPaymentLimitsResponse = try asLightningPaymentLimitsResponse(lightningPaymentLimitsResponse: val)
                list.append(lightningPaymentLimitsResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LightningPaymentLimitsResponse"))
            }
        }
        return list
    }

    static func arrayOf(lightningPaymentLimitsResponseList: [LightningPaymentLimitsResponse]) -> [Any] {
        return lightningPaymentLimitsResponseList.map { v -> [String: Any?] in return dictionaryOf(lightningPaymentLimitsResponse: v) }
    }

    static func asLimits(limits: [String: Any?]) throws -> Limits {
        guard let minSat = limits["minSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minSat", typeName: "Limits"))
        }
        guard let maxSat = limits["maxSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxSat", typeName: "Limits"))
        }
        guard let maxZeroConfSat = limits["maxZeroConfSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxZeroConfSat", typeName: "Limits"))
        }

        return Limits(minSat: minSat, maxSat: maxSat, maxZeroConfSat: maxZeroConfSat)
    }

    static func dictionaryOf(limits: Limits) -> [String: Any?] {
        return [
            "minSat": limits.minSat,
            "maxSat": limits.maxSat,
            "maxZeroConfSat": limits.maxZeroConfSat,
        ]
    }

    static func asLimitsList(arr: [Any]) throws -> [Limits] {
        var list = [Limits]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var limits = try asLimits(limits: val)
                list.append(limits)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Limits"))
            }
        }
        return list
    }

    static func arrayOf(limitsList: [Limits]) -> [Any] {
        return limitsList.map { v -> [String: Any?] in return dictionaryOf(limits: v) }
    }

    static func asLiquidAddressData(liquidAddressData: [String: Any?]) throws -> LiquidAddressData {
        guard let address = liquidAddressData["address"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "LiquidAddressData"))
        }
        guard let networkTmp = liquidAddressData["network"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "LiquidAddressData"))
        }
        let network = try asNetwork(network: networkTmp)

        var assetId: String?
        if hasNonNilKey(data: liquidAddressData, key: "assetId") {
            guard let assetIdTmp = liquidAddressData["assetId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "assetId"))
            }
            assetId = assetIdTmp
        }
        var amount: Double?
        if hasNonNilKey(data: liquidAddressData, key: "amount") {
            guard let amountTmp = liquidAddressData["amount"] as? Double else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "amount"))
            }
            amount = amountTmp
        }
        var amountSat: UInt64?
        if hasNonNilKey(data: liquidAddressData, key: "amountSat") {
            guard let amountSatTmp = liquidAddressData["amountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "amountSat"))
            }
            amountSat = amountSatTmp
        }
        var label: String?
        if hasNonNilKey(data: liquidAddressData, key: "label") {
            guard let labelTmp = liquidAddressData["label"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "label"))
            }
            label = labelTmp
        }
        var message: String?
        if hasNonNilKey(data: liquidAddressData, key: "message") {
            guard let messageTmp = liquidAddressData["message"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "message"))
            }
            message = messageTmp
        }

        return LiquidAddressData(address: address, network: network, assetId: assetId, amount: amount, amountSat: amountSat, label: label, message: message)
    }

    static func dictionaryOf(liquidAddressData: LiquidAddressData) -> [String: Any?] {
        return [
            "address": liquidAddressData.address,
            "network": valueOf(network: liquidAddressData.network),
            "assetId": liquidAddressData.assetId == nil ? nil : liquidAddressData.assetId,
            "amount": liquidAddressData.amount == nil ? nil : liquidAddressData.amount,
            "amountSat": liquidAddressData.amountSat == nil ? nil : liquidAddressData.amountSat,
            "label": liquidAddressData.label == nil ? nil : liquidAddressData.label,
            "message": liquidAddressData.message == nil ? nil : liquidAddressData.message,
        ]
    }

    static func asLiquidAddressDataList(arr: [Any]) throws -> [LiquidAddressData] {
        var list = [LiquidAddressData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var liquidAddressData = try asLiquidAddressData(liquidAddressData: val)
                list.append(liquidAddressData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LiquidAddressData"))
            }
        }
        return list
    }

    static func arrayOf(liquidAddressDataList: [LiquidAddressData]) -> [Any] {
        return liquidAddressDataList.map { v -> [String: Any?] in return dictionaryOf(liquidAddressData: v) }
    }

    static func asListPaymentsRequest(listPaymentsRequest: [String: Any?]) throws -> ListPaymentsRequest {
        var filters: [PaymentType]?
        if let filtersTmp = listPaymentsRequest["filters"] as? [String] {
            filters = try asPaymentTypeList(arr: filtersTmp)
        }

        var states: [PaymentState]?
        if let statesTmp = listPaymentsRequest["states"] as? [String] {
            states = try asPaymentStateList(arr: statesTmp)
        }

        var fromTimestamp: Int64?
        if hasNonNilKey(data: listPaymentsRequest, key: "fromTimestamp") {
            guard let fromTimestampTmp = listPaymentsRequest["fromTimestamp"] as? Int64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "fromTimestamp"))
            }
            fromTimestamp = fromTimestampTmp
        }
        var toTimestamp: Int64?
        if hasNonNilKey(data: listPaymentsRequest, key: "toTimestamp") {
            guard let toTimestampTmp = listPaymentsRequest["toTimestamp"] as? Int64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "toTimestamp"))
            }
            toTimestamp = toTimestampTmp
        }
        var offset: UInt32?
        if hasNonNilKey(data: listPaymentsRequest, key: "offset") {
            guard let offsetTmp = listPaymentsRequest["offset"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "offset"))
            }
            offset = offsetTmp
        }
        var limit: UInt32?
        if hasNonNilKey(data: listPaymentsRequest, key: "limit") {
            guard let limitTmp = listPaymentsRequest["limit"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "limit"))
            }
            limit = limitTmp
        }
        var details: ListPaymentDetails?
        if let detailsTmp = listPaymentsRequest["details"] as? [String: Any?] {
            details = try asListPaymentDetails(listPaymentDetails: detailsTmp)
        }

        var sortAscending: Bool?
        if hasNonNilKey(data: listPaymentsRequest, key: "sortAscending") {
            guard let sortAscendingTmp = listPaymentsRequest["sortAscending"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "sortAscending"))
            }
            sortAscending = sortAscendingTmp
        }

        return ListPaymentsRequest(filters: filters, states: states, fromTimestamp: fromTimestamp, toTimestamp: toTimestamp, offset: offset, limit: limit, details: details, sortAscending: sortAscending)
    }

    static func dictionaryOf(listPaymentsRequest: ListPaymentsRequest) -> [String: Any?] {
        return [
            "filters": listPaymentsRequest.filters == nil ? nil : arrayOf(paymentTypeList: listPaymentsRequest.filters!),
            "states": listPaymentsRequest.states == nil ? nil : arrayOf(paymentStateList: listPaymentsRequest.states!),
            "fromTimestamp": listPaymentsRequest.fromTimestamp == nil ? nil : listPaymentsRequest.fromTimestamp,
            "toTimestamp": listPaymentsRequest.toTimestamp == nil ? nil : listPaymentsRequest.toTimestamp,
            "offset": listPaymentsRequest.offset == nil ? nil : listPaymentsRequest.offset,
            "limit": listPaymentsRequest.limit == nil ? nil : listPaymentsRequest.limit,
            "details": listPaymentsRequest.details == nil ? nil : dictionaryOf(listPaymentDetails: listPaymentsRequest.details!),
            "sortAscending": listPaymentsRequest.sortAscending == nil ? nil : listPaymentsRequest.sortAscending,
        ]
    }

    static func asListPaymentsRequestList(arr: [Any]) throws -> [ListPaymentsRequest] {
        var list = [ListPaymentsRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var listPaymentsRequest = try asListPaymentsRequest(listPaymentsRequest: val)
                list.append(listPaymentsRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ListPaymentsRequest"))
            }
        }
        return list
    }

    static func arrayOf(listPaymentsRequestList: [ListPaymentsRequest]) -> [Any] {
        return listPaymentsRequestList.map { v -> [String: Any?] in return dictionaryOf(listPaymentsRequest: v) }
    }

    static func asLnOfferBlindedPath(lnOfferBlindedPath: [String: Any?]) throws -> LnOfferBlindedPath {
        guard let blindedHops = lnOfferBlindedPath["blindedHops"] as? [String] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "blindedHops", typeName: "LnOfferBlindedPath"))
        }

        return LnOfferBlindedPath(blindedHops: blindedHops)
    }

    static func dictionaryOf(lnOfferBlindedPath: LnOfferBlindedPath) -> [String: Any?] {
        return [
            "blindedHops": lnOfferBlindedPath.blindedHops,
        ]
    }

    static func asLnOfferBlindedPathList(arr: [Any]) throws -> [LnOfferBlindedPath] {
        var list = [LnOfferBlindedPath]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnOfferBlindedPath = try asLnOfferBlindedPath(lnOfferBlindedPath: val)
                list.append(lnOfferBlindedPath)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnOfferBlindedPath"))
            }
        }
        return list
    }

    static func arrayOf(lnOfferBlindedPathList: [LnOfferBlindedPath]) -> [Any] {
        return lnOfferBlindedPathList.map { v -> [String: Any?] in return dictionaryOf(lnOfferBlindedPath: v) }
    }

    static func asLnUrlAuthRequestData(lnUrlAuthRequestData: [String: Any?]) throws -> LnUrlAuthRequestData {
        guard let k1 = lnUrlAuthRequestData["k1"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "k1", typeName: "LnUrlAuthRequestData"))
        }
        guard let domain = lnUrlAuthRequestData["domain"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "domain", typeName: "LnUrlAuthRequestData"))
        }
        guard let url = lnUrlAuthRequestData["url"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "LnUrlAuthRequestData"))
        }
        var action: String?
        if hasNonNilKey(data: lnUrlAuthRequestData, key: "action") {
            guard let actionTmp = lnUrlAuthRequestData["action"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "action"))
            }
            action = actionTmp
        }

        return LnUrlAuthRequestData(k1: k1, domain: domain, url: url, action: action)
    }

    static func dictionaryOf(lnUrlAuthRequestData: LnUrlAuthRequestData) -> [String: Any?] {
        return [
            "k1": lnUrlAuthRequestData.k1,
            "domain": lnUrlAuthRequestData.domain,
            "url": lnUrlAuthRequestData.url,
            "action": lnUrlAuthRequestData.action == nil ? nil : lnUrlAuthRequestData.action,
        ]
    }

    static func asLnUrlAuthRequestDataList(arr: [Any]) throws -> [LnUrlAuthRequestData] {
        var list = [LnUrlAuthRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlAuthRequestData = try asLnUrlAuthRequestData(lnUrlAuthRequestData: val)
                list.append(lnUrlAuthRequestData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlAuthRequestData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlAuthRequestDataList: [LnUrlAuthRequestData]) -> [Any] {
        return lnUrlAuthRequestDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlAuthRequestData: v) }
    }

    static func asLnUrlErrorData(lnUrlErrorData: [String: Any?]) throws -> LnUrlErrorData {
        guard let reason = lnUrlErrorData["reason"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "reason", typeName: "LnUrlErrorData"))
        }

        return LnUrlErrorData(reason: reason)
    }

    static func dictionaryOf(lnUrlErrorData: LnUrlErrorData) -> [String: Any?] {
        return [
            "reason": lnUrlErrorData.reason,
        ]
    }

    static func asLnUrlErrorDataList(arr: [Any]) throws -> [LnUrlErrorData] {
        var list = [LnUrlErrorData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlErrorData = try asLnUrlErrorData(lnUrlErrorData: val)
                list.append(lnUrlErrorData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlErrorData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlErrorDataList: [LnUrlErrorData]) -> [Any] {
        return lnUrlErrorDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlErrorData: v) }
    }

    static func asLnUrlInfo(lnUrlInfo: [String: Any?]) throws -> LnUrlInfo {
        var lnAddress: String?
        if hasNonNilKey(data: lnUrlInfo, key: "lnAddress") {
            guard let lnAddressTmp = lnUrlInfo["lnAddress"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lnAddress"))
            }
            lnAddress = lnAddressTmp
        }
        var lnurlPayComment: String?
        if hasNonNilKey(data: lnUrlInfo, key: "lnurlPayComment") {
            guard let lnurlPayCommentTmp = lnUrlInfo["lnurlPayComment"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lnurlPayComment"))
            }
            lnurlPayComment = lnurlPayCommentTmp
        }
        var lnurlPayDomain: String?
        if hasNonNilKey(data: lnUrlInfo, key: "lnurlPayDomain") {
            guard let lnurlPayDomainTmp = lnUrlInfo["lnurlPayDomain"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lnurlPayDomain"))
            }
            lnurlPayDomain = lnurlPayDomainTmp
        }
        var lnurlPayMetadata: String?
        if hasNonNilKey(data: lnUrlInfo, key: "lnurlPayMetadata") {
            guard let lnurlPayMetadataTmp = lnUrlInfo["lnurlPayMetadata"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lnurlPayMetadata"))
            }
            lnurlPayMetadata = lnurlPayMetadataTmp
        }
        var lnurlPaySuccessAction: SuccessActionProcessed?
        if let lnurlPaySuccessActionTmp = lnUrlInfo["lnurlPaySuccessAction"] as? [String: Any?] {
            lnurlPaySuccessAction = try asSuccessActionProcessed(successActionProcessed: lnurlPaySuccessActionTmp)
        }

        var lnurlPayUnprocessedSuccessAction: SuccessAction?
        if let lnurlPayUnprocessedSuccessActionTmp = lnUrlInfo["lnurlPayUnprocessedSuccessAction"] as? [String: Any?] {
            lnurlPayUnprocessedSuccessAction = try asSuccessAction(successAction: lnurlPayUnprocessedSuccessActionTmp)
        }

        var lnurlWithdrawEndpoint: String?
        if hasNonNilKey(data: lnUrlInfo, key: "lnurlWithdrawEndpoint") {
            guard let lnurlWithdrawEndpointTmp = lnUrlInfo["lnurlWithdrawEndpoint"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lnurlWithdrawEndpoint"))
            }
            lnurlWithdrawEndpoint = lnurlWithdrawEndpointTmp
        }

        return LnUrlInfo(lnAddress: lnAddress, lnurlPayComment: lnurlPayComment, lnurlPayDomain: lnurlPayDomain, lnurlPayMetadata: lnurlPayMetadata, lnurlPaySuccessAction: lnurlPaySuccessAction, lnurlPayUnprocessedSuccessAction: lnurlPayUnprocessedSuccessAction, lnurlWithdrawEndpoint: lnurlWithdrawEndpoint)
    }

    static func dictionaryOf(lnUrlInfo: LnUrlInfo) -> [String: Any?] {
        return [
            "lnAddress": lnUrlInfo.lnAddress == nil ? nil : lnUrlInfo.lnAddress,
            "lnurlPayComment": lnUrlInfo.lnurlPayComment == nil ? nil : lnUrlInfo.lnurlPayComment,
            "lnurlPayDomain": lnUrlInfo.lnurlPayDomain == nil ? nil : lnUrlInfo.lnurlPayDomain,
            "lnurlPayMetadata": lnUrlInfo.lnurlPayMetadata == nil ? nil : lnUrlInfo.lnurlPayMetadata,
            "lnurlPaySuccessAction": lnUrlInfo.lnurlPaySuccessAction == nil ? nil : dictionaryOf(successActionProcessed: lnUrlInfo.lnurlPaySuccessAction!),
            "lnurlPayUnprocessedSuccessAction": lnUrlInfo.lnurlPayUnprocessedSuccessAction == nil ? nil : dictionaryOf(successAction: lnUrlInfo.lnurlPayUnprocessedSuccessAction!),
            "lnurlWithdrawEndpoint": lnUrlInfo.lnurlWithdrawEndpoint == nil ? nil : lnUrlInfo.lnurlWithdrawEndpoint,
        ]
    }

    static func asLnUrlInfoList(arr: [Any]) throws -> [LnUrlInfo] {
        var list = [LnUrlInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlInfo = try asLnUrlInfo(lnUrlInfo: val)
                list.append(lnUrlInfo)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlInfo"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlInfoList: [LnUrlInfo]) -> [Any] {
        return lnUrlInfoList.map { v -> [String: Any?] in return dictionaryOf(lnUrlInfo: v) }
    }

    static func asLnUrlPayErrorData(lnUrlPayErrorData: [String: Any?]) throws -> LnUrlPayErrorData {
        guard let paymentHash = lnUrlPayErrorData["paymentHash"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentHash", typeName: "LnUrlPayErrorData"))
        }
        guard let reason = lnUrlPayErrorData["reason"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "reason", typeName: "LnUrlPayErrorData"))
        }

        return LnUrlPayErrorData(paymentHash: paymentHash, reason: reason)
    }

    static func dictionaryOf(lnUrlPayErrorData: LnUrlPayErrorData) -> [String: Any?] {
        return [
            "paymentHash": lnUrlPayErrorData.paymentHash,
            "reason": lnUrlPayErrorData.reason,
        ]
    }

    static func asLnUrlPayErrorDataList(arr: [Any]) throws -> [LnUrlPayErrorData] {
        var list = [LnUrlPayErrorData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayErrorData = try asLnUrlPayErrorData(lnUrlPayErrorData: val)
                list.append(lnUrlPayErrorData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayErrorData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPayErrorDataList: [LnUrlPayErrorData]) -> [Any] {
        return lnUrlPayErrorDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayErrorData: v) }
    }

    static func asLnUrlPayRequest(lnUrlPayRequest: [String: Any?]) throws -> LnUrlPayRequest {
        guard let prepareResponseTmp = lnUrlPayRequest["prepareResponse"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareResponse", typeName: "LnUrlPayRequest"))
        }
        let prepareResponse = try asPrepareLnUrlPayResponse(prepareLnUrlPayResponse: prepareResponseTmp)

        return LnUrlPayRequest(prepareResponse: prepareResponse)
    }

    static func dictionaryOf(lnUrlPayRequest: LnUrlPayRequest) -> [String: Any?] {
        return [
            "prepareResponse": dictionaryOf(prepareLnUrlPayResponse: lnUrlPayRequest.prepareResponse),
        ]
    }

    static func asLnUrlPayRequestList(arr: [Any]) throws -> [LnUrlPayRequest] {
        var list = [LnUrlPayRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayRequest = try asLnUrlPayRequest(lnUrlPayRequest: val)
                list.append(lnUrlPayRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayRequest"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPayRequestList: [LnUrlPayRequest]) -> [Any] {
        return lnUrlPayRequestList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayRequest: v) }
    }

    static func asLnUrlPayRequestData(lnUrlPayRequestData: [String: Any?]) throws -> LnUrlPayRequestData {
        guard let callback = lnUrlPayRequestData["callback"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "callback", typeName: "LnUrlPayRequestData"))
        }
        guard let minSendable = lnUrlPayRequestData["minSendable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minSendable", typeName: "LnUrlPayRequestData"))
        }
        guard let maxSendable = lnUrlPayRequestData["maxSendable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxSendable", typeName: "LnUrlPayRequestData"))
        }
        guard let metadataStr = lnUrlPayRequestData["metadataStr"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "metadataStr", typeName: "LnUrlPayRequestData"))
        }
        guard let commentAllowed = lnUrlPayRequestData["commentAllowed"] as? UInt16 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "commentAllowed", typeName: "LnUrlPayRequestData"))
        }
        guard let domain = lnUrlPayRequestData["domain"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "domain", typeName: "LnUrlPayRequestData"))
        }
        guard let allowsNostr = lnUrlPayRequestData["allowsNostr"] as? Bool else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "allowsNostr", typeName: "LnUrlPayRequestData"))
        }
        var nostrPubkey: String?
        if hasNonNilKey(data: lnUrlPayRequestData, key: "nostrPubkey") {
            guard let nostrPubkeyTmp = lnUrlPayRequestData["nostrPubkey"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "nostrPubkey"))
            }
            nostrPubkey = nostrPubkeyTmp
        }
        var lnAddress: String?
        if hasNonNilKey(data: lnUrlPayRequestData, key: "lnAddress") {
            guard let lnAddressTmp = lnUrlPayRequestData["lnAddress"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lnAddress"))
            }
            lnAddress = lnAddressTmp
        }

        return LnUrlPayRequestData(callback: callback, minSendable: minSendable, maxSendable: maxSendable, metadataStr: metadataStr, commentAllowed: commentAllowed, domain: domain, allowsNostr: allowsNostr, nostrPubkey: nostrPubkey, lnAddress: lnAddress)
    }

    static func dictionaryOf(lnUrlPayRequestData: LnUrlPayRequestData) -> [String: Any?] {
        return [
            "callback": lnUrlPayRequestData.callback,
            "minSendable": lnUrlPayRequestData.minSendable,
            "maxSendable": lnUrlPayRequestData.maxSendable,
            "metadataStr": lnUrlPayRequestData.metadataStr,
            "commentAllowed": lnUrlPayRequestData.commentAllowed,
            "domain": lnUrlPayRequestData.domain,
            "allowsNostr": lnUrlPayRequestData.allowsNostr,
            "nostrPubkey": lnUrlPayRequestData.nostrPubkey == nil ? nil : lnUrlPayRequestData.nostrPubkey,
            "lnAddress": lnUrlPayRequestData.lnAddress == nil ? nil : lnUrlPayRequestData.lnAddress,
        ]
    }

    static func asLnUrlPayRequestDataList(arr: [Any]) throws -> [LnUrlPayRequestData] {
        var list = [LnUrlPayRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayRequestData = try asLnUrlPayRequestData(lnUrlPayRequestData: val)
                list.append(lnUrlPayRequestData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayRequestData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPayRequestDataList: [LnUrlPayRequestData]) -> [Any] {
        return lnUrlPayRequestDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayRequestData: v) }
    }

    static func asLnUrlPaySuccessData(lnUrlPaySuccessData: [String: Any?]) throws -> LnUrlPaySuccessData {
        var successAction: SuccessActionProcessed?
        if let successActionTmp = lnUrlPaySuccessData["successAction"] as? [String: Any?] {
            successAction = try asSuccessActionProcessed(successActionProcessed: successActionTmp)
        }

        guard let paymentTmp = lnUrlPaySuccessData["payment"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payment", typeName: "LnUrlPaySuccessData"))
        }
        let payment = try asPayment(payment: paymentTmp)

        return LnUrlPaySuccessData(successAction: successAction, payment: payment)
    }

    static func dictionaryOf(lnUrlPaySuccessData: LnUrlPaySuccessData) -> [String: Any?] {
        return [
            "successAction": lnUrlPaySuccessData.successAction == nil ? nil : dictionaryOf(successActionProcessed: lnUrlPaySuccessData.successAction!),
            "payment": dictionaryOf(payment: lnUrlPaySuccessData.payment),
        ]
    }

    static func asLnUrlPaySuccessDataList(arr: [Any]) throws -> [LnUrlPaySuccessData] {
        var list = [LnUrlPaySuccessData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPaySuccessData = try asLnUrlPaySuccessData(lnUrlPaySuccessData: val)
                list.append(lnUrlPaySuccessData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPaySuccessData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPaySuccessDataList: [LnUrlPaySuccessData]) -> [Any] {
        return lnUrlPaySuccessDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPaySuccessData: v) }
    }

    static func asLnUrlWithdrawRequest(lnUrlWithdrawRequest: [String: Any?]) throws -> LnUrlWithdrawRequest {
        guard let dataTmp = lnUrlWithdrawRequest["data"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawRequest"))
        }
        let data = try asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: dataTmp)

        guard let amountMsat = lnUrlWithdrawRequest["amountMsat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountMsat", typeName: "LnUrlWithdrawRequest"))
        }
        var description: String?
        if hasNonNilKey(data: lnUrlWithdrawRequest, key: "description") {
            guard let descriptionTmp = lnUrlWithdrawRequest["description"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "description"))
            }
            description = descriptionTmp
        }

        return LnUrlWithdrawRequest(data: data, amountMsat: amountMsat, description: description)
    }

    static func dictionaryOf(lnUrlWithdrawRequest: LnUrlWithdrawRequest) -> [String: Any?] {
        return [
            "data": dictionaryOf(lnUrlWithdrawRequestData: lnUrlWithdrawRequest.data),
            "amountMsat": lnUrlWithdrawRequest.amountMsat,
            "description": lnUrlWithdrawRequest.description == nil ? nil : lnUrlWithdrawRequest.description,
        ]
    }

    static func asLnUrlWithdrawRequestList(arr: [Any]) throws -> [LnUrlWithdrawRequest] {
        var list = [LnUrlWithdrawRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawRequest = try asLnUrlWithdrawRequest(lnUrlWithdrawRequest: val)
                list.append(lnUrlWithdrawRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawRequest"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawRequestList: [LnUrlWithdrawRequest]) -> [Any] {
        return lnUrlWithdrawRequestList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawRequest: v) }
    }

    static func asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: [String: Any?]) throws -> LnUrlWithdrawRequestData {
        guard let callback = lnUrlWithdrawRequestData["callback"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "callback", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let k1 = lnUrlWithdrawRequestData["k1"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "k1", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let defaultDescription = lnUrlWithdrawRequestData["defaultDescription"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "defaultDescription", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let minWithdrawable = lnUrlWithdrawRequestData["minWithdrawable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minWithdrawable", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let maxWithdrawable = lnUrlWithdrawRequestData["maxWithdrawable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxWithdrawable", typeName: "LnUrlWithdrawRequestData"))
        }

        return LnUrlWithdrawRequestData(callback: callback, k1: k1, defaultDescription: defaultDescription, minWithdrawable: minWithdrawable, maxWithdrawable: maxWithdrawable)
    }

    static func dictionaryOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData) -> [String: Any?] {
        return [
            "callback": lnUrlWithdrawRequestData.callback,
            "k1": lnUrlWithdrawRequestData.k1,
            "defaultDescription": lnUrlWithdrawRequestData.defaultDescription,
            "minWithdrawable": lnUrlWithdrawRequestData.minWithdrawable,
            "maxWithdrawable": lnUrlWithdrawRequestData.maxWithdrawable,
        ]
    }

    static func asLnUrlWithdrawRequestDataList(arr: [Any]) throws -> [LnUrlWithdrawRequestData] {
        var list = [LnUrlWithdrawRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawRequestData = try asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: val)
                list.append(lnUrlWithdrawRequestData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawRequestData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawRequestDataList: [LnUrlWithdrawRequestData]) -> [Any] {
        return lnUrlWithdrawRequestDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawRequestData: v) }
    }

    static func asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: [String: Any?]) throws -> LnUrlWithdrawSuccessData {
        guard let invoiceTmp = lnUrlWithdrawSuccessData["invoice"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "LnUrlWithdrawSuccessData"))
        }
        let invoice = try asLnInvoice(lnInvoice: invoiceTmp)

        return LnUrlWithdrawSuccessData(invoice: invoice)
    }

    static func dictionaryOf(lnUrlWithdrawSuccessData: LnUrlWithdrawSuccessData) -> [String: Any?] {
        return [
            "invoice": dictionaryOf(lnInvoice: lnUrlWithdrawSuccessData.invoice),
        ]
    }

    static func asLnUrlWithdrawSuccessDataList(arr: [Any]) throws -> [LnUrlWithdrawSuccessData] {
        var list = [LnUrlWithdrawSuccessData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawSuccessData = try asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: val)
                list.append(lnUrlWithdrawSuccessData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawSuccessData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawSuccessDataList: [LnUrlWithdrawSuccessData]) -> [Any] {
        return lnUrlWithdrawSuccessDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawSuccessData: v) }
    }

    static func asLocaleOverrides(localeOverrides: [String: Any?]) throws -> LocaleOverrides {
        guard let locale = localeOverrides["locale"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "locale", typeName: "LocaleOverrides"))
        }
        var spacing: UInt32?
        if hasNonNilKey(data: localeOverrides, key: "spacing") {
            guard let spacingTmp = localeOverrides["spacing"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "spacing"))
            }
            spacing = spacingTmp
        }
        guard let symbolTmp = localeOverrides["symbol"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "symbol", typeName: "LocaleOverrides"))
        }
        let symbol = try asSymbol(symbol: symbolTmp)

        return LocaleOverrides(locale: locale, spacing: spacing, symbol: symbol)
    }

    static func dictionaryOf(localeOverrides: LocaleOverrides) -> [String: Any?] {
        return [
            "locale": localeOverrides.locale,
            "spacing": localeOverrides.spacing == nil ? nil : localeOverrides.spacing,
            "symbol": dictionaryOf(symbol: localeOverrides.symbol),
        ]
    }

    static func asLocaleOverridesList(arr: [Any]) throws -> [LocaleOverrides] {
        var list = [LocaleOverrides]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var localeOverrides = try asLocaleOverrides(localeOverrides: val)
                list.append(localeOverrides)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LocaleOverrides"))
            }
        }
        return list
    }

    static func arrayOf(localeOverridesList: [LocaleOverrides]) -> [Any] {
        return localeOverridesList.map { v -> [String: Any?] in return dictionaryOf(localeOverrides: v) }
    }

    static func asLocalizedName(localizedName: [String: Any?]) throws -> LocalizedName {
        guard let locale = localizedName["locale"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "locale", typeName: "LocalizedName"))
        }
        guard let name = localizedName["name"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "name", typeName: "LocalizedName"))
        }

        return LocalizedName(locale: locale, name: name)
    }

    static func dictionaryOf(localizedName: LocalizedName) -> [String: Any?] {
        return [
            "locale": localizedName.locale,
            "name": localizedName.name,
        ]
    }

    static func asLocalizedNameList(arr: [Any]) throws -> [LocalizedName] {
        var list = [LocalizedName]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var localizedName = try asLocalizedName(localizedName: val)
                list.append(localizedName)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LocalizedName"))
            }
        }
        return list
    }

    static func arrayOf(localizedNameList: [LocalizedName]) -> [Any] {
        return localizedNameList.map { v -> [String: Any?] in return dictionaryOf(localizedName: v) }
    }

    static func asLogEntry(logEntry: [String: Any?]) throws -> LogEntry {
        guard let line = logEntry["line"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "line", typeName: "LogEntry"))
        }
        guard let level = logEntry["level"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "level", typeName: "LogEntry"))
        }

        return LogEntry(line: line, level: level)
    }

    static func dictionaryOf(logEntry: LogEntry) -> [String: Any?] {
        return [
            "line": logEntry.line,
            "level": logEntry.level,
        ]
    }

    static func asLogEntryList(arr: [Any]) throws -> [LogEntry] {
        var list = [LogEntry]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var logEntry = try asLogEntry(logEntry: val)
                list.append(logEntry)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LogEntry"))
            }
        }
        return list
    }

    static func arrayOf(logEntryList: [LogEntry]) -> [Any] {
        return logEntryList.map { v -> [String: Any?] in return dictionaryOf(logEntry: v) }
    }

    static func asMessageSuccessActionData(messageSuccessActionData: [String: Any?]) throws -> MessageSuccessActionData {
        guard let message = messageSuccessActionData["message"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "message", typeName: "MessageSuccessActionData"))
        }

        return MessageSuccessActionData(message: message)
    }

    static func dictionaryOf(messageSuccessActionData: MessageSuccessActionData) -> [String: Any?] {
        return [
            "message": messageSuccessActionData.message,
        ]
    }

    static func asMessageSuccessActionDataList(arr: [Any]) throws -> [MessageSuccessActionData] {
        var list = [MessageSuccessActionData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var messageSuccessActionData = try asMessageSuccessActionData(messageSuccessActionData: val)
                list.append(messageSuccessActionData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "MessageSuccessActionData"))
            }
        }
        return list
    }

    static func arrayOf(messageSuccessActionDataList: [MessageSuccessActionData]) -> [Any] {
        return messageSuccessActionDataList.map { v -> [String: Any?] in return dictionaryOf(messageSuccessActionData: v) }
    }

    static func asOnchainPaymentLimitsResponse(onchainPaymentLimitsResponse: [String: Any?]) throws -> OnchainPaymentLimitsResponse {
        guard let sendTmp = onchainPaymentLimitsResponse["send"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "send", typeName: "OnchainPaymentLimitsResponse"))
        }
        let send = try asLimits(limits: sendTmp)

        guard let receiveTmp = onchainPaymentLimitsResponse["receive"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receive", typeName: "OnchainPaymentLimitsResponse"))
        }
        let receive = try asLimits(limits: receiveTmp)

        return OnchainPaymentLimitsResponse(send: send, receive: receive)
    }

    static func dictionaryOf(onchainPaymentLimitsResponse: OnchainPaymentLimitsResponse) -> [String: Any?] {
        return [
            "send": dictionaryOf(limits: onchainPaymentLimitsResponse.send),
            "receive": dictionaryOf(limits: onchainPaymentLimitsResponse.receive),
        ]
    }

    static func asOnchainPaymentLimitsResponseList(arr: [Any]) throws -> [OnchainPaymentLimitsResponse] {
        var list = [OnchainPaymentLimitsResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var onchainPaymentLimitsResponse = try asOnchainPaymentLimitsResponse(onchainPaymentLimitsResponse: val)
                list.append(onchainPaymentLimitsResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "OnchainPaymentLimitsResponse"))
            }
        }
        return list
    }

    static func arrayOf(onchainPaymentLimitsResponseList: [OnchainPaymentLimitsResponse]) -> [Any] {
        return onchainPaymentLimitsResponseList.map { v -> [String: Any?] in return dictionaryOf(onchainPaymentLimitsResponse: v) }
    }

    static func asPayOnchainRequest(payOnchainRequest: [String: Any?]) throws -> PayOnchainRequest {
        guard let address = payOnchainRequest["address"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "PayOnchainRequest"))
        }
        guard let prepareResponseTmp = payOnchainRequest["prepareResponse"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareResponse", typeName: "PayOnchainRequest"))
        }
        let prepareResponse = try asPreparePayOnchainResponse(preparePayOnchainResponse: prepareResponseTmp)

        return PayOnchainRequest(address: address, prepareResponse: prepareResponse)
    }

    static func dictionaryOf(payOnchainRequest: PayOnchainRequest) -> [String: Any?] {
        return [
            "address": payOnchainRequest.address,
            "prepareResponse": dictionaryOf(preparePayOnchainResponse: payOnchainRequest.prepareResponse),
        ]
    }

    static func asPayOnchainRequestList(arr: [Any]) throws -> [PayOnchainRequest] {
        var list = [PayOnchainRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var payOnchainRequest = try asPayOnchainRequest(payOnchainRequest: val)
                list.append(payOnchainRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PayOnchainRequest"))
            }
        }
        return list
    }

    static func arrayOf(payOnchainRequestList: [PayOnchainRequest]) -> [Any] {
        return payOnchainRequestList.map { v -> [String: Any?] in return dictionaryOf(payOnchainRequest: v) }
    }

    static func asPayment(payment: [String: Any?]) throws -> Payment {
        guard let timestamp = payment["timestamp"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "timestamp", typeName: "Payment"))
        }
        guard let amountSat = payment["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "Payment"))
        }
        guard let feesSat = payment["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "Payment"))
        }
        guard let paymentTypeTmp = payment["paymentType"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentType", typeName: "Payment"))
        }
        let paymentType = try asPaymentType(paymentType: paymentTypeTmp)

        guard let statusTmp = payment["status"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "status", typeName: "Payment"))
        }
        let status = try asPaymentState(paymentState: statusTmp)

        guard let detailsTmp = payment["details"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "Payment"))
        }
        let details = try asPaymentDetails(paymentDetails: detailsTmp)

        var swapperFeesSat: UInt64?
        if hasNonNilKey(data: payment, key: "swapperFeesSat") {
            guard let swapperFeesSatTmp = payment["swapperFeesSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "swapperFeesSat"))
            }
            swapperFeesSat = swapperFeesSatTmp
        }
        var destination: String?
        if hasNonNilKey(data: payment, key: "destination") {
            guard let destinationTmp = payment["destination"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "destination"))
            }
            destination = destinationTmp
        }
        var txId: String?
        if hasNonNilKey(data: payment, key: "txId") {
            guard let txIdTmp = payment["txId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "txId"))
            }
            txId = txIdTmp
        }
        var unblindingData: String?
        if hasNonNilKey(data: payment, key: "unblindingData") {
            guard let unblindingDataTmp = payment["unblindingData"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "unblindingData"))
            }
            unblindingData = unblindingDataTmp
        }

        return Payment(timestamp: timestamp, amountSat: amountSat, feesSat: feesSat, paymentType: paymentType, status: status, details: details, swapperFeesSat: swapperFeesSat, destination: destination, txId: txId, unblindingData: unblindingData)
    }

    static func dictionaryOf(payment: Payment) -> [String: Any?] {
        return [
            "timestamp": payment.timestamp,
            "amountSat": payment.amountSat,
            "feesSat": payment.feesSat,
            "paymentType": valueOf(paymentType: payment.paymentType),
            "status": valueOf(paymentState: payment.status),
            "details": dictionaryOf(paymentDetails: payment.details),
            "swapperFeesSat": payment.swapperFeesSat == nil ? nil : payment.swapperFeesSat,
            "destination": payment.destination == nil ? nil : payment.destination,
            "txId": payment.txId == nil ? nil : payment.txId,
            "unblindingData": payment.unblindingData == nil ? nil : payment.unblindingData,
        ]
    }

    static func asPaymentList(arr: [Any]) throws -> [Payment] {
        var list = [Payment]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var payment = try asPayment(payment: val)
                list.append(payment)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Payment"))
            }
        }
        return list
    }

    static func arrayOf(paymentList: [Payment]) -> [Any] {
        return paymentList.map { v -> [String: Any?] in return dictionaryOf(payment: v) }
    }

    static func asPrepareBuyBitcoinRequest(prepareBuyBitcoinRequest: [String: Any?]) throws -> PrepareBuyBitcoinRequest {
        guard let providerTmp = prepareBuyBitcoinRequest["provider"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "provider", typeName: "PrepareBuyBitcoinRequest"))
        }
        let provider = try asBuyBitcoinProvider(buyBitcoinProvider: providerTmp)

        guard let amountSat = prepareBuyBitcoinRequest["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "PrepareBuyBitcoinRequest"))
        }

        return PrepareBuyBitcoinRequest(provider: provider, amountSat: amountSat)
    }

    static func dictionaryOf(prepareBuyBitcoinRequest: PrepareBuyBitcoinRequest) -> [String: Any?] {
        return [
            "provider": valueOf(buyBitcoinProvider: prepareBuyBitcoinRequest.provider),
            "amountSat": prepareBuyBitcoinRequest.amountSat,
        ]
    }

    static func asPrepareBuyBitcoinRequestList(arr: [Any]) throws -> [PrepareBuyBitcoinRequest] {
        var list = [PrepareBuyBitcoinRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareBuyBitcoinRequest = try asPrepareBuyBitcoinRequest(prepareBuyBitcoinRequest: val)
                list.append(prepareBuyBitcoinRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareBuyBitcoinRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareBuyBitcoinRequestList: [PrepareBuyBitcoinRequest]) -> [Any] {
        return prepareBuyBitcoinRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareBuyBitcoinRequest: v) }
    }

    static func asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: [String: Any?]) throws -> PrepareBuyBitcoinResponse {
        guard let providerTmp = prepareBuyBitcoinResponse["provider"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "provider", typeName: "PrepareBuyBitcoinResponse"))
        }
        let provider = try asBuyBitcoinProvider(buyBitcoinProvider: providerTmp)

        guard let amountSat = prepareBuyBitcoinResponse["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "PrepareBuyBitcoinResponse"))
        }
        guard let feesSat = prepareBuyBitcoinResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareBuyBitcoinResponse"))
        }

        return PrepareBuyBitcoinResponse(provider: provider, amountSat: amountSat, feesSat: feesSat)
    }

    static func dictionaryOf(prepareBuyBitcoinResponse: PrepareBuyBitcoinResponse) -> [String: Any?] {
        return [
            "provider": valueOf(buyBitcoinProvider: prepareBuyBitcoinResponse.provider),
            "amountSat": prepareBuyBitcoinResponse.amountSat,
            "feesSat": prepareBuyBitcoinResponse.feesSat,
        ]
    }

    static func asPrepareBuyBitcoinResponseList(arr: [Any]) throws -> [PrepareBuyBitcoinResponse] {
        var list = [PrepareBuyBitcoinResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareBuyBitcoinResponse = try asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: val)
                list.append(prepareBuyBitcoinResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareBuyBitcoinResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareBuyBitcoinResponseList: [PrepareBuyBitcoinResponse]) -> [Any] {
        return prepareBuyBitcoinResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareBuyBitcoinResponse: v) }
    }

    static func asPrepareLnUrlPayRequest(prepareLnUrlPayRequest: [String: Any?]) throws -> PrepareLnUrlPayRequest {
        guard let dataTmp = prepareLnUrlPayRequest["data"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "PrepareLnUrlPayRequest"))
        }
        let data = try asLnUrlPayRequestData(lnUrlPayRequestData: dataTmp)

        guard let amountTmp = prepareLnUrlPayRequest["amount"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amount", typeName: "PrepareLnUrlPayRequest"))
        }
        let amount = try asPayAmount(payAmount: amountTmp)

        var bip353Address: String?
        if hasNonNilKey(data: prepareLnUrlPayRequest, key: "bip353Address") {
            guard let bip353AddressTmp = prepareLnUrlPayRequest["bip353Address"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "bip353Address"))
            }
            bip353Address = bip353AddressTmp
        }
        var comment: String?
        if hasNonNilKey(data: prepareLnUrlPayRequest, key: "comment") {
            guard let commentTmp = prepareLnUrlPayRequest["comment"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "comment"))
            }
            comment = commentTmp
        }
        var validateSuccessActionUrl: Bool?
        if hasNonNilKey(data: prepareLnUrlPayRequest, key: "validateSuccessActionUrl") {
            guard let validateSuccessActionUrlTmp = prepareLnUrlPayRequest["validateSuccessActionUrl"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "validateSuccessActionUrl"))
            }
            validateSuccessActionUrl = validateSuccessActionUrlTmp
        }

        return PrepareLnUrlPayRequest(data: data, amount: amount, bip353Address: bip353Address, comment: comment, validateSuccessActionUrl: validateSuccessActionUrl)
    }

    static func dictionaryOf(prepareLnUrlPayRequest: PrepareLnUrlPayRequest) -> [String: Any?] {
        return [
            "data": dictionaryOf(lnUrlPayRequestData: prepareLnUrlPayRequest.data),
            "amount": dictionaryOf(payAmount: prepareLnUrlPayRequest.amount),
            "bip353Address": prepareLnUrlPayRequest.bip353Address == nil ? nil : prepareLnUrlPayRequest.bip353Address,
            "comment": prepareLnUrlPayRequest.comment == nil ? nil : prepareLnUrlPayRequest.comment,
            "validateSuccessActionUrl": prepareLnUrlPayRequest.validateSuccessActionUrl == nil ? nil : prepareLnUrlPayRequest.validateSuccessActionUrl,
        ]
    }

    static func asPrepareLnUrlPayRequestList(arr: [Any]) throws -> [PrepareLnUrlPayRequest] {
        var list = [PrepareLnUrlPayRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareLnUrlPayRequest = try asPrepareLnUrlPayRequest(prepareLnUrlPayRequest: val)
                list.append(prepareLnUrlPayRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareLnUrlPayRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareLnUrlPayRequestList: [PrepareLnUrlPayRequest]) -> [Any] {
        return prepareLnUrlPayRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareLnUrlPayRequest: v) }
    }

    static func asPrepareLnUrlPayResponse(prepareLnUrlPayResponse: [String: Any?]) throws -> PrepareLnUrlPayResponse {
        guard let destinationTmp = prepareLnUrlPayResponse["destination"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "destination", typeName: "PrepareLnUrlPayResponse"))
        }
        let destination = try asSendDestination(sendDestination: destinationTmp)

        guard let feesSat = prepareLnUrlPayResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareLnUrlPayResponse"))
        }
        guard let dataTmp = prepareLnUrlPayResponse["data"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "PrepareLnUrlPayResponse"))
        }
        let data = try asLnUrlPayRequestData(lnUrlPayRequestData: dataTmp)

        guard let amountTmp = prepareLnUrlPayResponse["amount"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amount", typeName: "PrepareLnUrlPayResponse"))
        }
        let amount = try asPayAmount(payAmount: amountTmp)

        var comment: String?
        if hasNonNilKey(data: prepareLnUrlPayResponse, key: "comment") {
            guard let commentTmp = prepareLnUrlPayResponse["comment"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "comment"))
            }
            comment = commentTmp
        }
        var successAction: SuccessAction?
        if let successActionTmp = prepareLnUrlPayResponse["successAction"] as? [String: Any?] {
            successAction = try asSuccessAction(successAction: successActionTmp)
        }

        return PrepareLnUrlPayResponse(destination: destination, feesSat: feesSat, data: data, amount: amount, comment: comment, successAction: successAction)
    }

    static func dictionaryOf(prepareLnUrlPayResponse: PrepareLnUrlPayResponse) -> [String: Any?] {
        return [
            "destination": dictionaryOf(sendDestination: prepareLnUrlPayResponse.destination),
            "feesSat": prepareLnUrlPayResponse.feesSat,
            "data": dictionaryOf(lnUrlPayRequestData: prepareLnUrlPayResponse.data),
            "amount": dictionaryOf(payAmount: prepareLnUrlPayResponse.amount),
            "comment": prepareLnUrlPayResponse.comment == nil ? nil : prepareLnUrlPayResponse.comment,
            "successAction": prepareLnUrlPayResponse.successAction == nil ? nil : dictionaryOf(successAction: prepareLnUrlPayResponse.successAction!),
        ]
    }

    static func asPrepareLnUrlPayResponseList(arr: [Any]) throws -> [PrepareLnUrlPayResponse] {
        var list = [PrepareLnUrlPayResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareLnUrlPayResponse = try asPrepareLnUrlPayResponse(prepareLnUrlPayResponse: val)
                list.append(prepareLnUrlPayResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareLnUrlPayResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareLnUrlPayResponseList: [PrepareLnUrlPayResponse]) -> [Any] {
        return prepareLnUrlPayResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareLnUrlPayResponse: v) }
    }

    static func asPreparePayOnchainRequest(preparePayOnchainRequest: [String: Any?]) throws -> PreparePayOnchainRequest {
        guard let amountTmp = preparePayOnchainRequest["amount"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amount", typeName: "PreparePayOnchainRequest"))
        }
        let amount = try asPayAmount(payAmount: amountTmp)

        var feeRateSatPerVbyte: UInt32?
        if hasNonNilKey(data: preparePayOnchainRequest, key: "feeRateSatPerVbyte") {
            guard let feeRateSatPerVbyteTmp = preparePayOnchainRequest["feeRateSatPerVbyte"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "feeRateSatPerVbyte"))
            }
            feeRateSatPerVbyte = feeRateSatPerVbyteTmp
        }

        return PreparePayOnchainRequest(amount: amount, feeRateSatPerVbyte: feeRateSatPerVbyte)
    }

    static func dictionaryOf(preparePayOnchainRequest: PreparePayOnchainRequest) -> [String: Any?] {
        return [
            "amount": dictionaryOf(payAmount: preparePayOnchainRequest.amount),
            "feeRateSatPerVbyte": preparePayOnchainRequest.feeRateSatPerVbyte == nil ? nil : preparePayOnchainRequest.feeRateSatPerVbyte,
        ]
    }

    static func asPreparePayOnchainRequestList(arr: [Any]) throws -> [PreparePayOnchainRequest] {
        var list = [PreparePayOnchainRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var preparePayOnchainRequest = try asPreparePayOnchainRequest(preparePayOnchainRequest: val)
                list.append(preparePayOnchainRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PreparePayOnchainRequest"))
            }
        }
        return list
    }

    static func arrayOf(preparePayOnchainRequestList: [PreparePayOnchainRequest]) -> [Any] {
        return preparePayOnchainRequestList.map { v -> [String: Any?] in return dictionaryOf(preparePayOnchainRequest: v) }
    }

    static func asPreparePayOnchainResponse(preparePayOnchainResponse: [String: Any?]) throws -> PreparePayOnchainResponse {
        guard let receiverAmountSat = preparePayOnchainResponse["receiverAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "PreparePayOnchainResponse"))
        }
        guard let claimFeesSat = preparePayOnchainResponse["claimFeesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "claimFeesSat", typeName: "PreparePayOnchainResponse"))
        }
        guard let totalFeesSat = preparePayOnchainResponse["totalFeesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "totalFeesSat", typeName: "PreparePayOnchainResponse"))
        }

        return PreparePayOnchainResponse(receiverAmountSat: receiverAmountSat, claimFeesSat: claimFeesSat, totalFeesSat: totalFeesSat)
    }

    static func dictionaryOf(preparePayOnchainResponse: PreparePayOnchainResponse) -> [String: Any?] {
        return [
            "receiverAmountSat": preparePayOnchainResponse.receiverAmountSat,
            "claimFeesSat": preparePayOnchainResponse.claimFeesSat,
            "totalFeesSat": preparePayOnchainResponse.totalFeesSat,
        ]
    }

    static func asPreparePayOnchainResponseList(arr: [Any]) throws -> [PreparePayOnchainResponse] {
        var list = [PreparePayOnchainResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var preparePayOnchainResponse = try asPreparePayOnchainResponse(preparePayOnchainResponse: val)
                list.append(preparePayOnchainResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PreparePayOnchainResponse"))
            }
        }
        return list
    }

    static func arrayOf(preparePayOnchainResponseList: [PreparePayOnchainResponse]) -> [Any] {
        return preparePayOnchainResponseList.map { v -> [String: Any?] in return dictionaryOf(preparePayOnchainResponse: v) }
    }

    static func asPrepareReceiveRequest(prepareReceiveRequest: [String: Any?]) throws -> PrepareReceiveRequest {
        guard let paymentMethodTmp = prepareReceiveRequest["paymentMethod"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentMethod", typeName: "PrepareReceiveRequest"))
        }
        let paymentMethod = try asPaymentMethod(paymentMethod: paymentMethodTmp)

        var amount: ReceiveAmount?
        if let amountTmp = prepareReceiveRequest["amount"] as? [String: Any?] {
            amount = try asReceiveAmount(receiveAmount: amountTmp)
        }

        return PrepareReceiveRequest(paymentMethod: paymentMethod, amount: amount)
    }

    static func dictionaryOf(prepareReceiveRequest: PrepareReceiveRequest) -> [String: Any?] {
        return [
            "paymentMethod": valueOf(paymentMethod: prepareReceiveRequest.paymentMethod),
            "amount": prepareReceiveRequest.amount == nil ? nil : dictionaryOf(receiveAmount: prepareReceiveRequest.amount!),
        ]
    }

    static func asPrepareReceiveRequestList(arr: [Any]) throws -> [PrepareReceiveRequest] {
        var list = [PrepareReceiveRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceiveRequest = try asPrepareReceiveRequest(prepareReceiveRequest: val)
                list.append(prepareReceiveRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveRequestList: [PrepareReceiveRequest]) -> [Any] {
        return prepareReceiveRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareReceiveRequest: v) }
    }

    static func asPrepareReceiveResponse(prepareReceiveResponse: [String: Any?]) throws -> PrepareReceiveResponse {
        guard let paymentMethodTmp = prepareReceiveResponse["paymentMethod"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentMethod", typeName: "PrepareReceiveResponse"))
        }
        let paymentMethod = try asPaymentMethod(paymentMethod: paymentMethodTmp)

        guard let feesSat = prepareReceiveResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareReceiveResponse"))
        }
        var amount: ReceiveAmount?
        if let amountTmp = prepareReceiveResponse["amount"] as? [String: Any?] {
            amount = try asReceiveAmount(receiveAmount: amountTmp)
        }

        var minPayerAmountSat: UInt64?
        if hasNonNilKey(data: prepareReceiveResponse, key: "minPayerAmountSat") {
            guard let minPayerAmountSatTmp = prepareReceiveResponse["minPayerAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "minPayerAmountSat"))
            }
            minPayerAmountSat = minPayerAmountSatTmp
        }
        var maxPayerAmountSat: UInt64?
        if hasNonNilKey(data: prepareReceiveResponse, key: "maxPayerAmountSat") {
            guard let maxPayerAmountSatTmp = prepareReceiveResponse["maxPayerAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "maxPayerAmountSat"))
            }
            maxPayerAmountSat = maxPayerAmountSatTmp
        }
        var swapperFeerate: Double?
        if hasNonNilKey(data: prepareReceiveResponse, key: "swapperFeerate") {
            guard let swapperFeerateTmp = prepareReceiveResponse["swapperFeerate"] as? Double else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "swapperFeerate"))
            }
            swapperFeerate = swapperFeerateTmp
        }

        return PrepareReceiveResponse(paymentMethod: paymentMethod, feesSat: feesSat, amount: amount, minPayerAmountSat: minPayerAmountSat, maxPayerAmountSat: maxPayerAmountSat, swapperFeerate: swapperFeerate)
    }

    static func dictionaryOf(prepareReceiveResponse: PrepareReceiveResponse) -> [String: Any?] {
        return [
            "paymentMethod": valueOf(paymentMethod: prepareReceiveResponse.paymentMethod),
            "feesSat": prepareReceiveResponse.feesSat,
            "amount": prepareReceiveResponse.amount == nil ? nil : dictionaryOf(receiveAmount: prepareReceiveResponse.amount!),
            "minPayerAmountSat": prepareReceiveResponse.minPayerAmountSat == nil ? nil : prepareReceiveResponse.minPayerAmountSat,
            "maxPayerAmountSat": prepareReceiveResponse.maxPayerAmountSat == nil ? nil : prepareReceiveResponse.maxPayerAmountSat,
            "swapperFeerate": prepareReceiveResponse.swapperFeerate == nil ? nil : prepareReceiveResponse.swapperFeerate,
        ]
    }

    static func asPrepareReceiveResponseList(arr: [Any]) throws -> [PrepareReceiveResponse] {
        var list = [PrepareReceiveResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceiveResponse = try asPrepareReceiveResponse(prepareReceiveResponse: val)
                list.append(prepareReceiveResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveResponseList: [PrepareReceiveResponse]) -> [Any] {
        return prepareReceiveResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareReceiveResponse: v) }
    }

    static func asPrepareRefundRequest(prepareRefundRequest: [String: Any?]) throws -> PrepareRefundRequest {
        guard let swapAddress = prepareRefundRequest["swapAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapAddress", typeName: "PrepareRefundRequest"))
        }
        guard let refundAddress = prepareRefundRequest["refundAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "refundAddress", typeName: "PrepareRefundRequest"))
        }
        guard let feeRateSatPerVbyte = prepareRefundRequest["feeRateSatPerVbyte"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feeRateSatPerVbyte", typeName: "PrepareRefundRequest"))
        }

        return PrepareRefundRequest(swapAddress: swapAddress, refundAddress: refundAddress, feeRateSatPerVbyte: feeRateSatPerVbyte)
    }

    static func dictionaryOf(prepareRefundRequest: PrepareRefundRequest) -> [String: Any?] {
        return [
            "swapAddress": prepareRefundRequest.swapAddress,
            "refundAddress": prepareRefundRequest.refundAddress,
            "feeRateSatPerVbyte": prepareRefundRequest.feeRateSatPerVbyte,
        ]
    }

    static func asPrepareRefundRequestList(arr: [Any]) throws -> [PrepareRefundRequest] {
        var list = [PrepareRefundRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareRefundRequest = try asPrepareRefundRequest(prepareRefundRequest: val)
                list.append(prepareRefundRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareRefundRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareRefundRequestList: [PrepareRefundRequest]) -> [Any] {
        return prepareRefundRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareRefundRequest: v) }
    }

    static func asPrepareRefundResponse(prepareRefundResponse: [String: Any?]) throws -> PrepareRefundResponse {
        guard let txVsize = prepareRefundResponse["txVsize"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "txVsize", typeName: "PrepareRefundResponse"))
        }
        guard let txFeeSat = prepareRefundResponse["txFeeSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "txFeeSat", typeName: "PrepareRefundResponse"))
        }
        var lastRefundTxId: String?
        if hasNonNilKey(data: prepareRefundResponse, key: "lastRefundTxId") {
            guard let lastRefundTxIdTmp = prepareRefundResponse["lastRefundTxId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lastRefundTxId"))
            }
            lastRefundTxId = lastRefundTxIdTmp
        }

        return PrepareRefundResponse(txVsize: txVsize, txFeeSat: txFeeSat, lastRefundTxId: lastRefundTxId)
    }

    static func dictionaryOf(prepareRefundResponse: PrepareRefundResponse) -> [String: Any?] {
        return [
            "txVsize": prepareRefundResponse.txVsize,
            "txFeeSat": prepareRefundResponse.txFeeSat,
            "lastRefundTxId": prepareRefundResponse.lastRefundTxId == nil ? nil : prepareRefundResponse.lastRefundTxId,
        ]
    }

    static func asPrepareRefundResponseList(arr: [Any]) throws -> [PrepareRefundResponse] {
        var list = [PrepareRefundResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareRefundResponse = try asPrepareRefundResponse(prepareRefundResponse: val)
                list.append(prepareRefundResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareRefundResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareRefundResponseList: [PrepareRefundResponse]) -> [Any] {
        return prepareRefundResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareRefundResponse: v) }
    }

    static func asPrepareSendRequest(prepareSendRequest: [String: Any?]) throws -> PrepareSendRequest {
        guard let destination = prepareSendRequest["destination"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "destination", typeName: "PrepareSendRequest"))
        }
        var amount: PayAmount?
        if let amountTmp = prepareSendRequest["amount"] as? [String: Any?] {
            amount = try asPayAmount(payAmount: amountTmp)
        }

        return PrepareSendRequest(destination: destination, amount: amount)
    }

    static func dictionaryOf(prepareSendRequest: PrepareSendRequest) -> [String: Any?] {
        return [
            "destination": prepareSendRequest.destination,
            "amount": prepareSendRequest.amount == nil ? nil : dictionaryOf(payAmount: prepareSendRequest.amount!),
        ]
    }

    static func asPrepareSendRequestList(arr: [Any]) throws -> [PrepareSendRequest] {
        var list = [PrepareSendRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareSendRequest = try asPrepareSendRequest(prepareSendRequest: val)
                list.append(prepareSendRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareSendRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareSendRequestList: [PrepareSendRequest]) -> [Any] {
        return prepareSendRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareSendRequest: v) }
    }

    static func asPrepareSendResponse(prepareSendResponse: [String: Any?]) throws -> PrepareSendResponse {
        guard let destinationTmp = prepareSendResponse["destination"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "destination", typeName: "PrepareSendResponse"))
        }
        let destination = try asSendDestination(sendDestination: destinationTmp)

        var amount: PayAmount?
        if let amountTmp = prepareSendResponse["amount"] as? [String: Any?] {
            amount = try asPayAmount(payAmount: amountTmp)
        }

        var feesSat: UInt64?
        if hasNonNilKey(data: prepareSendResponse, key: "feesSat") {
            guard let feesSatTmp = prepareSendResponse["feesSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "feesSat"))
            }
            feesSat = feesSatTmp
        }
        var estimatedAssetFees: Double?
        if hasNonNilKey(data: prepareSendResponse, key: "estimatedAssetFees") {
            guard let estimatedAssetFeesTmp = prepareSendResponse["estimatedAssetFees"] as? Double else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "estimatedAssetFees"))
            }
            estimatedAssetFees = estimatedAssetFeesTmp
        }
        var exchangeAmountSat: UInt64?
        if hasNonNilKey(data: prepareSendResponse, key: "exchangeAmountSat") {
            guard let exchangeAmountSatTmp = prepareSendResponse["exchangeAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "exchangeAmountSat"))
            }
            exchangeAmountSat = exchangeAmountSatTmp
        }

        return PrepareSendResponse(destination: destination, amount: amount, feesSat: feesSat, estimatedAssetFees: estimatedAssetFees, exchangeAmountSat: exchangeAmountSat)
    }

    static func dictionaryOf(prepareSendResponse: PrepareSendResponse) -> [String: Any?] {
        return [
            "destination": dictionaryOf(sendDestination: prepareSendResponse.destination),
            "amount": prepareSendResponse.amount == nil ? nil : dictionaryOf(payAmount: prepareSendResponse.amount!),
            "feesSat": prepareSendResponse.feesSat == nil ? nil : prepareSendResponse.feesSat,
            "estimatedAssetFees": prepareSendResponse.estimatedAssetFees == nil ? nil : prepareSendResponse.estimatedAssetFees,
            "exchangeAmountSat": prepareSendResponse.exchangeAmountSat == nil ? nil : prepareSendResponse.exchangeAmountSat,
        ]
    }

    static func asPrepareSendResponseList(arr: [Any]) throws -> [PrepareSendResponse] {
        var list = [PrepareSendResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareSendResponse = try asPrepareSendResponse(prepareSendResponse: val)
                list.append(prepareSendResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareSendResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareSendResponseList: [PrepareSendResponse]) -> [Any] {
        return prepareSendResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareSendResponse: v) }
    }

    static func asRate(rate: [String: Any?]) throws -> Rate {
        guard let coin = rate["coin"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "coin", typeName: "Rate"))
        }
        guard let value = rate["value"] as? Double else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "value", typeName: "Rate"))
        }

        return Rate(coin: coin, value: value)
    }

    static func dictionaryOf(rate: Rate) -> [String: Any?] {
        return [
            "coin": rate.coin,
            "value": rate.value,
        ]
    }

    static func asRateList(arr: [Any]) throws -> [Rate] {
        var list = [Rate]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var rate = try asRate(rate: val)
                list.append(rate)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Rate"))
            }
        }
        return list
    }

    static func arrayOf(rateList: [Rate]) -> [Any] {
        return rateList.map { v -> [String: Any?] in return dictionaryOf(rate: v) }
    }

    static func asReceivePaymentRequest(receivePaymentRequest: [String: Any?]) throws -> ReceivePaymentRequest {
        guard let prepareResponseTmp = receivePaymentRequest["prepareResponse"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareResponse", typeName: "ReceivePaymentRequest"))
        }
        let prepareResponse = try asPrepareReceiveResponse(prepareReceiveResponse: prepareResponseTmp)

        var description: String?
        if hasNonNilKey(data: receivePaymentRequest, key: "description") {
            guard let descriptionTmp = receivePaymentRequest["description"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "description"))
            }
            description = descriptionTmp
        }
        var useDescriptionHash: Bool?
        if hasNonNilKey(data: receivePaymentRequest, key: "useDescriptionHash") {
            guard let useDescriptionHashTmp = receivePaymentRequest["useDescriptionHash"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "useDescriptionHash"))
            }
            useDescriptionHash = useDescriptionHashTmp
        }
        var payerNote: String?
        if hasNonNilKey(data: receivePaymentRequest, key: "payerNote") {
            guard let payerNoteTmp = receivePaymentRequest["payerNote"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "payerNote"))
            }
            payerNote = payerNoteTmp
        }

        return ReceivePaymentRequest(prepareResponse: prepareResponse, description: description, useDescriptionHash: useDescriptionHash, payerNote: payerNote)
    }

    static func dictionaryOf(receivePaymentRequest: ReceivePaymentRequest) -> [String: Any?] {
        return [
            "prepareResponse": dictionaryOf(prepareReceiveResponse: receivePaymentRequest.prepareResponse),
            "description": receivePaymentRequest.description == nil ? nil : receivePaymentRequest.description,
            "useDescriptionHash": receivePaymentRequest.useDescriptionHash == nil ? nil : receivePaymentRequest.useDescriptionHash,
            "payerNote": receivePaymentRequest.payerNote == nil ? nil : receivePaymentRequest.payerNote,
        ]
    }

    static func asReceivePaymentRequestList(arr: [Any]) throws -> [ReceivePaymentRequest] {
        var list = [ReceivePaymentRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receivePaymentRequest = try asReceivePaymentRequest(receivePaymentRequest: val)
                list.append(receivePaymentRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ReceivePaymentRequest"))
            }
        }
        return list
    }

    static func arrayOf(receivePaymentRequestList: [ReceivePaymentRequest]) -> [Any] {
        return receivePaymentRequestList.map { v -> [String: Any?] in return dictionaryOf(receivePaymentRequest: v) }
    }

    static func asReceivePaymentResponse(receivePaymentResponse: [String: Any?]) throws -> ReceivePaymentResponse {
        guard let destination = receivePaymentResponse["destination"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "destination", typeName: "ReceivePaymentResponse"))
        }
        var liquidExpirationBlockheight: UInt32?
        if hasNonNilKey(data: receivePaymentResponse, key: "liquidExpirationBlockheight") {
            guard let liquidExpirationBlockheightTmp = receivePaymentResponse["liquidExpirationBlockheight"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "liquidExpirationBlockheight"))
            }
            liquidExpirationBlockheight = liquidExpirationBlockheightTmp
        }
        var bitcoinExpirationBlockheight: UInt32?
        if hasNonNilKey(data: receivePaymentResponse, key: "bitcoinExpirationBlockheight") {
            guard let bitcoinExpirationBlockheightTmp = receivePaymentResponse["bitcoinExpirationBlockheight"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "bitcoinExpirationBlockheight"))
            }
            bitcoinExpirationBlockheight = bitcoinExpirationBlockheightTmp
        }

        return ReceivePaymentResponse(destination: destination, liquidExpirationBlockheight: liquidExpirationBlockheight, bitcoinExpirationBlockheight: bitcoinExpirationBlockheight)
    }

    static func dictionaryOf(receivePaymentResponse: ReceivePaymentResponse) -> [String: Any?] {
        return [
            "destination": receivePaymentResponse.destination,
            "liquidExpirationBlockheight": receivePaymentResponse.liquidExpirationBlockheight == nil ? nil : receivePaymentResponse.liquidExpirationBlockheight,
            "bitcoinExpirationBlockheight": receivePaymentResponse.bitcoinExpirationBlockheight == nil ? nil : receivePaymentResponse.bitcoinExpirationBlockheight,
        ]
    }

    static func asReceivePaymentResponseList(arr: [Any]) throws -> [ReceivePaymentResponse] {
        var list = [ReceivePaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receivePaymentResponse = try asReceivePaymentResponse(receivePaymentResponse: val)
                list.append(receivePaymentResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ReceivePaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(receivePaymentResponseList: [ReceivePaymentResponse]) -> [Any] {
        return receivePaymentResponseList.map { v -> [String: Any?] in return dictionaryOf(receivePaymentResponse: v) }
    }

    static func asRecommendedFees(recommendedFees: [String: Any?]) throws -> RecommendedFees {
        guard let fastestFee = recommendedFees["fastestFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "fastestFee", typeName: "RecommendedFees"))
        }
        guard let halfHourFee = recommendedFees["halfHourFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "halfHourFee", typeName: "RecommendedFees"))
        }
        guard let hourFee = recommendedFees["hourFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "hourFee", typeName: "RecommendedFees"))
        }
        guard let economyFee = recommendedFees["economyFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "economyFee", typeName: "RecommendedFees"))
        }
        guard let minimumFee = recommendedFees["minimumFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minimumFee", typeName: "RecommendedFees"))
        }

        return RecommendedFees(fastestFee: fastestFee, halfHourFee: halfHourFee, hourFee: hourFee, economyFee: economyFee, minimumFee: minimumFee)
    }

    static func dictionaryOf(recommendedFees: RecommendedFees) -> [String: Any?] {
        return [
            "fastestFee": recommendedFees.fastestFee,
            "halfHourFee": recommendedFees.halfHourFee,
            "hourFee": recommendedFees.hourFee,
            "economyFee": recommendedFees.economyFee,
            "minimumFee": recommendedFees.minimumFee,
        ]
    }

    static func asRecommendedFeesList(arr: [Any]) throws -> [RecommendedFees] {
        var list = [RecommendedFees]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var recommendedFees = try asRecommendedFees(recommendedFees: val)
                list.append(recommendedFees)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RecommendedFees"))
            }
        }
        return list
    }

    static func arrayOf(recommendedFeesList: [RecommendedFees]) -> [Any] {
        return recommendedFeesList.map { v -> [String: Any?] in return dictionaryOf(recommendedFees: v) }
    }

    static func asRefundRequest(refundRequest: [String: Any?]) throws -> RefundRequest {
        guard let swapAddress = refundRequest["swapAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapAddress", typeName: "RefundRequest"))
        }
        guard let refundAddress = refundRequest["refundAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "refundAddress", typeName: "RefundRequest"))
        }
        guard let feeRateSatPerVbyte = refundRequest["feeRateSatPerVbyte"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feeRateSatPerVbyte", typeName: "RefundRequest"))
        }

        return RefundRequest(swapAddress: swapAddress, refundAddress: refundAddress, feeRateSatPerVbyte: feeRateSatPerVbyte)
    }

    static func dictionaryOf(refundRequest: RefundRequest) -> [String: Any?] {
        return [
            "swapAddress": refundRequest.swapAddress,
            "refundAddress": refundRequest.refundAddress,
            "feeRateSatPerVbyte": refundRequest.feeRateSatPerVbyte,
        ]
    }

    static func asRefundRequestList(arr: [Any]) throws -> [RefundRequest] {
        var list = [RefundRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var refundRequest = try asRefundRequest(refundRequest: val)
                list.append(refundRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RefundRequest"))
            }
        }
        return list
    }

    static func arrayOf(refundRequestList: [RefundRequest]) -> [Any] {
        return refundRequestList.map { v -> [String: Any?] in return dictionaryOf(refundRequest: v) }
    }

    static func asRefundResponse(refundResponse: [String: Any?]) throws -> RefundResponse {
        guard let refundTxId = refundResponse["refundTxId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "refundTxId", typeName: "RefundResponse"))
        }

        return RefundResponse(refundTxId: refundTxId)
    }

    static func dictionaryOf(refundResponse: RefundResponse) -> [String: Any?] {
        return [
            "refundTxId": refundResponse.refundTxId,
        ]
    }

    static func asRefundResponseList(arr: [Any]) throws -> [RefundResponse] {
        var list = [RefundResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var refundResponse = try asRefundResponse(refundResponse: val)
                list.append(refundResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RefundResponse"))
            }
        }
        return list
    }

    static func arrayOf(refundResponseList: [RefundResponse]) -> [Any] {
        return refundResponseList.map { v -> [String: Any?] in return dictionaryOf(refundResponse: v) }
    }

    static func asRefundableSwap(refundableSwap: [String: Any?]) throws -> RefundableSwap {
        guard let swapAddress = refundableSwap["swapAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapAddress", typeName: "RefundableSwap"))
        }
        guard let timestamp = refundableSwap["timestamp"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "timestamp", typeName: "RefundableSwap"))
        }
        guard let amountSat = refundableSwap["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "RefundableSwap"))
        }
        var lastRefundTxId: String?
        if hasNonNilKey(data: refundableSwap, key: "lastRefundTxId") {
            guard let lastRefundTxIdTmp = refundableSwap["lastRefundTxId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lastRefundTxId"))
            }
            lastRefundTxId = lastRefundTxIdTmp
        }

        return RefundableSwap(swapAddress: swapAddress, timestamp: timestamp, amountSat: amountSat, lastRefundTxId: lastRefundTxId)
    }

    static func dictionaryOf(refundableSwap: RefundableSwap) -> [String: Any?] {
        return [
            "swapAddress": refundableSwap.swapAddress,
            "timestamp": refundableSwap.timestamp,
            "amountSat": refundableSwap.amountSat,
            "lastRefundTxId": refundableSwap.lastRefundTxId == nil ? nil : refundableSwap.lastRefundTxId,
        ]
    }

    static func asRefundableSwapList(arr: [Any]) throws -> [RefundableSwap] {
        var list = [RefundableSwap]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var refundableSwap = try asRefundableSwap(refundableSwap: val)
                list.append(refundableSwap)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RefundableSwap"))
            }
        }
        return list
    }

    static func arrayOf(refundableSwapList: [RefundableSwap]) -> [Any] {
        return refundableSwapList.map { v -> [String: Any?] in return dictionaryOf(refundableSwap: v) }
    }

    static func asRestoreRequest(restoreRequest: [String: Any?]) throws -> RestoreRequest {
        var backupPath: String?
        if hasNonNilKey(data: restoreRequest, key: "backupPath") {
            guard let backupPathTmp = restoreRequest["backupPath"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "backupPath"))
            }
            backupPath = backupPathTmp
        }

        return RestoreRequest(backupPath: backupPath)
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
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RestoreRequest"))
            }
        }
        return list
    }

    static func arrayOf(restoreRequestList: [RestoreRequest]) -> [Any] {
        return restoreRequestList.map { v -> [String: Any?] in return dictionaryOf(restoreRequest: v) }
    }

    static func asRouteHint(routeHint: [String: Any?]) throws -> RouteHint {
        guard let hopsTmp = routeHint["hops"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "hops", typeName: "RouteHint"))
        }
        let hops = try asRouteHintHopList(arr: hopsTmp)

        return RouteHint(hops: hops)
    }

    static func dictionaryOf(routeHint: RouteHint) -> [String: Any?] {
        return [
            "hops": arrayOf(routeHintHopList: routeHint.hops),
        ]
    }

    static func asRouteHintList(arr: [Any]) throws -> [RouteHint] {
        var list = [RouteHint]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var routeHint = try asRouteHint(routeHint: val)
                list.append(routeHint)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RouteHint"))
            }
        }
        return list
    }

    static func arrayOf(routeHintList: [RouteHint]) -> [Any] {
        return routeHintList.map { v -> [String: Any?] in return dictionaryOf(routeHint: v) }
    }

    static func asRouteHintHop(routeHintHop: [String: Any?]) throws -> RouteHintHop {
        guard let srcNodeId = routeHintHop["srcNodeId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "srcNodeId", typeName: "RouteHintHop"))
        }
        guard let shortChannelId = routeHintHop["shortChannelId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "shortChannelId", typeName: "RouteHintHop"))
        }
        guard let feesBaseMsat = routeHintHop["feesBaseMsat"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesBaseMsat", typeName: "RouteHintHop"))
        }
        guard let feesProportionalMillionths = routeHintHop["feesProportionalMillionths"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesProportionalMillionths", typeName: "RouteHintHop"))
        }
        guard let cltvExpiryDelta = routeHintHop["cltvExpiryDelta"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "cltvExpiryDelta", typeName: "RouteHintHop"))
        }
        var htlcMinimumMsat: UInt64?
        if hasNonNilKey(data: routeHintHop, key: "htlcMinimumMsat") {
            guard let htlcMinimumMsatTmp = routeHintHop["htlcMinimumMsat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "htlcMinimumMsat"))
            }
            htlcMinimumMsat = htlcMinimumMsatTmp
        }
        var htlcMaximumMsat: UInt64?
        if hasNonNilKey(data: routeHintHop, key: "htlcMaximumMsat") {
            guard let htlcMaximumMsatTmp = routeHintHop["htlcMaximumMsat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "htlcMaximumMsat"))
            }
            htlcMaximumMsat = htlcMaximumMsatTmp
        }

        return RouteHintHop(srcNodeId: srcNodeId, shortChannelId: shortChannelId, feesBaseMsat: feesBaseMsat, feesProportionalMillionths: feesProportionalMillionths, cltvExpiryDelta: cltvExpiryDelta, htlcMinimumMsat: htlcMinimumMsat, htlcMaximumMsat: htlcMaximumMsat)
    }

    static func dictionaryOf(routeHintHop: RouteHintHop) -> [String: Any?] {
        return [
            "srcNodeId": routeHintHop.srcNodeId,
            "shortChannelId": routeHintHop.shortChannelId,
            "feesBaseMsat": routeHintHop.feesBaseMsat,
            "feesProportionalMillionths": routeHintHop.feesProportionalMillionths,
            "cltvExpiryDelta": routeHintHop.cltvExpiryDelta,
            "htlcMinimumMsat": routeHintHop.htlcMinimumMsat == nil ? nil : routeHintHop.htlcMinimumMsat,
            "htlcMaximumMsat": routeHintHop.htlcMaximumMsat == nil ? nil : routeHintHop.htlcMaximumMsat,
        ]
    }

    static func asRouteHintHopList(arr: [Any]) throws -> [RouteHintHop] {
        var list = [RouteHintHop]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var routeHintHop = try asRouteHintHop(routeHintHop: val)
                list.append(routeHintHop)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RouteHintHop"))
            }
        }
        return list
    }

    static func arrayOf(routeHintHopList: [RouteHintHop]) -> [Any] {
        return routeHintHopList.map { v -> [String: Any?] in return dictionaryOf(routeHintHop: v) }
    }

    static func asSendPaymentRequest(sendPaymentRequest: [String: Any?]) throws -> SendPaymentRequest {
        guard let prepareResponseTmp = sendPaymentRequest["prepareResponse"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareResponse", typeName: "SendPaymentRequest"))
        }
        let prepareResponse = try asPrepareSendResponse(prepareSendResponse: prepareResponseTmp)

        var useAssetFees: Bool?
        if hasNonNilKey(data: sendPaymentRequest, key: "useAssetFees") {
            guard let useAssetFeesTmp = sendPaymentRequest["useAssetFees"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "useAssetFees"))
            }
            useAssetFees = useAssetFeesTmp
        }
        var payerNote: String?
        if hasNonNilKey(data: sendPaymentRequest, key: "payerNote") {
            guard let payerNoteTmp = sendPaymentRequest["payerNote"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "payerNote"))
            }
            payerNote = payerNoteTmp
        }

        return SendPaymentRequest(prepareResponse: prepareResponse, useAssetFees: useAssetFees, payerNote: payerNote)
    }

    static func dictionaryOf(sendPaymentRequest: SendPaymentRequest) -> [String: Any?] {
        return [
            "prepareResponse": dictionaryOf(prepareSendResponse: sendPaymentRequest.prepareResponse),
            "useAssetFees": sendPaymentRequest.useAssetFees == nil ? nil : sendPaymentRequest.useAssetFees,
            "payerNote": sendPaymentRequest.payerNote == nil ? nil : sendPaymentRequest.payerNote,
        ]
    }

    static func asSendPaymentRequestList(arr: [Any]) throws -> [SendPaymentRequest] {
        var list = [SendPaymentRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sendPaymentRequest = try asSendPaymentRequest(sendPaymentRequest: val)
                list.append(sendPaymentRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SendPaymentRequest"))
            }
        }
        return list
    }

    static func arrayOf(sendPaymentRequestList: [SendPaymentRequest]) -> [Any] {
        return sendPaymentRequestList.map { v -> [String: Any?] in return dictionaryOf(sendPaymentRequest: v) }
    }

    static func asSendPaymentResponse(sendPaymentResponse: [String: Any?]) throws -> SendPaymentResponse {
        guard let paymentTmp = sendPaymentResponse["payment"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payment", typeName: "SendPaymentResponse"))
        }
        let payment = try asPayment(payment: paymentTmp)

        return SendPaymentResponse(payment: payment)
    }

    static func dictionaryOf(sendPaymentResponse: SendPaymentResponse) -> [String: Any?] {
        return [
            "payment": dictionaryOf(payment: sendPaymentResponse.payment),
        ]
    }

    static func asSendPaymentResponseList(arr: [Any]) throws -> [SendPaymentResponse] {
        var list = [SendPaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sendPaymentResponse = try asSendPaymentResponse(sendPaymentResponse: val)
                list.append(sendPaymentResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SendPaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(sendPaymentResponseList: [SendPaymentResponse]) -> [Any] {
        return sendPaymentResponseList.map { v -> [String: Any?] in return dictionaryOf(sendPaymentResponse: v) }
    }

    static func asSignMessageRequest(signMessageRequest: [String: Any?]) throws -> SignMessageRequest {
        guard let message = signMessageRequest["message"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "message", typeName: "SignMessageRequest"))
        }

        return SignMessageRequest(message: message)
    }

    static func dictionaryOf(signMessageRequest: SignMessageRequest) -> [String: Any?] {
        return [
            "message": signMessageRequest.message,
        ]
    }

    static func asSignMessageRequestList(arr: [Any]) throws -> [SignMessageRequest] {
        var list = [SignMessageRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var signMessageRequest = try asSignMessageRequest(signMessageRequest: val)
                list.append(signMessageRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SignMessageRequest"))
            }
        }
        return list
    }

    static func arrayOf(signMessageRequestList: [SignMessageRequest]) -> [Any] {
        return signMessageRequestList.map { v -> [String: Any?] in return dictionaryOf(signMessageRequest: v) }
    }

    static func asSignMessageResponse(signMessageResponse: [String: Any?]) throws -> SignMessageResponse {
        guard let signature = signMessageResponse["signature"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "signature", typeName: "SignMessageResponse"))
        }

        return SignMessageResponse(signature: signature)
    }

    static func dictionaryOf(signMessageResponse: SignMessageResponse) -> [String: Any?] {
        return [
            "signature": signMessageResponse.signature,
        ]
    }

    static func asSignMessageResponseList(arr: [Any]) throws -> [SignMessageResponse] {
        var list = [SignMessageResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var signMessageResponse = try asSignMessageResponse(signMessageResponse: val)
                list.append(signMessageResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SignMessageResponse"))
            }
        }
        return list
    }

    static func arrayOf(signMessageResponseList: [SignMessageResponse]) -> [Any] {
        return signMessageResponseList.map { v -> [String: Any?] in return dictionaryOf(signMessageResponse: v) }
    }

    static func asSymbol(symbol: [String: Any?]) throws -> Symbol {
        var grapheme: String?
        if hasNonNilKey(data: symbol, key: "grapheme") {
            guard let graphemeTmp = symbol["grapheme"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "grapheme"))
            }
            grapheme = graphemeTmp
        }
        var template: String?
        if hasNonNilKey(data: symbol, key: "template") {
            guard let templateTmp = symbol["template"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "template"))
            }
            template = templateTmp
        }
        var rtl: Bool?
        if hasNonNilKey(data: symbol, key: "rtl") {
            guard let rtlTmp = symbol["rtl"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "rtl"))
            }
            rtl = rtlTmp
        }
        var position: UInt32?
        if hasNonNilKey(data: symbol, key: "position") {
            guard let positionTmp = symbol["position"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "position"))
            }
            position = positionTmp
        }

        return Symbol(grapheme: grapheme, template: template, rtl: rtl, position: position)
    }

    static func dictionaryOf(symbol: Symbol) -> [String: Any?] {
        return [
            "grapheme": symbol.grapheme == nil ? nil : symbol.grapheme,
            "template": symbol.template == nil ? nil : symbol.template,
            "rtl": symbol.rtl == nil ? nil : symbol.rtl,
            "position": symbol.position == nil ? nil : symbol.position,
        ]
    }

    static func asSymbolList(arr: [Any]) throws -> [Symbol] {
        var list = [Symbol]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var symbol = try asSymbol(symbol: val)
                list.append(symbol)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Symbol"))
            }
        }
        return list
    }

    static func arrayOf(symbolList: [Symbol]) -> [Any] {
        return symbolList.map { v -> [String: Any?] in return dictionaryOf(symbol: v) }
    }

    static func asUrlSuccessActionData(urlSuccessActionData: [String: Any?]) throws -> UrlSuccessActionData {
        guard let description = urlSuccessActionData["description"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "UrlSuccessActionData"))
        }
        guard let url = urlSuccessActionData["url"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "UrlSuccessActionData"))
        }
        guard let matchesCallbackDomain = urlSuccessActionData["matchesCallbackDomain"] as? Bool else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "matchesCallbackDomain", typeName: "UrlSuccessActionData"))
        }

        return UrlSuccessActionData(description: description, url: url, matchesCallbackDomain: matchesCallbackDomain)
    }

    static func dictionaryOf(urlSuccessActionData: UrlSuccessActionData) -> [String: Any?] {
        return [
            "description": urlSuccessActionData.description,
            "url": urlSuccessActionData.url,
            "matchesCallbackDomain": urlSuccessActionData.matchesCallbackDomain,
        ]
    }

    static func asUrlSuccessActionDataList(arr: [Any]) throws -> [UrlSuccessActionData] {
        var list = [UrlSuccessActionData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var urlSuccessActionData = try asUrlSuccessActionData(urlSuccessActionData: val)
                list.append(urlSuccessActionData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "UrlSuccessActionData"))
            }
        }
        return list
    }

    static func arrayOf(urlSuccessActionDataList: [UrlSuccessActionData]) -> [Any] {
        return urlSuccessActionDataList.map { v -> [String: Any?] in return dictionaryOf(urlSuccessActionData: v) }
    }

    static func asWalletInfo(walletInfo: [String: Any?]) throws -> WalletInfo {
        guard let balanceSat = walletInfo["balanceSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "balanceSat", typeName: "WalletInfo"))
        }
        guard let pendingSendSat = walletInfo["pendingSendSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingSendSat", typeName: "WalletInfo"))
        }
        guard let pendingReceiveSat = walletInfo["pendingReceiveSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingReceiveSat", typeName: "WalletInfo"))
        }
        guard let fingerprint = walletInfo["fingerprint"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "fingerprint", typeName: "WalletInfo"))
        }
        guard let pubkey = walletInfo["pubkey"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pubkey", typeName: "WalletInfo"))
        }
        guard let assetBalancesTmp = walletInfo["assetBalances"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "assetBalances", typeName: "WalletInfo"))
        }
        let assetBalances = try asAssetBalanceList(arr: assetBalancesTmp)

        return WalletInfo(balanceSat: balanceSat, pendingSendSat: pendingSendSat, pendingReceiveSat: pendingReceiveSat, fingerprint: fingerprint, pubkey: pubkey, assetBalances: assetBalances)
    }

    static func dictionaryOf(walletInfo: WalletInfo) -> [String: Any?] {
        return [
            "balanceSat": walletInfo.balanceSat,
            "pendingSendSat": walletInfo.pendingSendSat,
            "pendingReceiveSat": walletInfo.pendingReceiveSat,
            "fingerprint": walletInfo.fingerprint,
            "pubkey": walletInfo.pubkey,
            "assetBalances": arrayOf(assetBalanceList: walletInfo.assetBalances),
        ]
    }

    static func asWalletInfoList(arr: [Any]) throws -> [WalletInfo] {
        var list = [WalletInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var walletInfo = try asWalletInfo(walletInfo: val)
                list.append(walletInfo)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "WalletInfo"))
            }
        }
        return list
    }

    static func arrayOf(walletInfoList: [WalletInfo]) -> [Any] {
        return walletInfoList.map { v -> [String: Any?] in return dictionaryOf(walletInfo: v) }
    }

    static func asAesSuccessActionDataResult(aesSuccessActionDataResult: [String: Any?]) throws -> AesSuccessActionDataResult {
        let type = aesSuccessActionDataResult["type"] as! String
        if type == "decrypted" {
            guard let dataTmp = aesSuccessActionDataResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "AesSuccessActionDataResult"))
            }
            let _data = try asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: dataTmp)

            return AesSuccessActionDataResult.decrypted(data: _data)
        }
        if type == "errorStatus" {
            guard let _reason = aesSuccessActionDataResult["reason"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "reason", typeName: "AesSuccessActionDataResult"))
            }
            return AesSuccessActionDataResult.errorStatus(reason: _reason)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum AesSuccessActionDataResult")
    }

    static func dictionaryOf(aesSuccessActionDataResult: AesSuccessActionDataResult) -> [String: Any?] {
        switch aesSuccessActionDataResult {
        case let .decrypted(
            data
        ):
            return [
                "type": "decrypted",
                "data": dictionaryOf(aesSuccessActionDataDecrypted: data),
            ]

        case let .errorStatus(
            reason
        ):
            return [
                "type": "errorStatus",
                "reason": reason,
            ]
        }
    }

    static func arrayOf(aesSuccessActionDataResultList: [AesSuccessActionDataResult]) -> [Any] {
        return aesSuccessActionDataResultList.map { v -> [String: Any?] in return dictionaryOf(aesSuccessActionDataResult: v) }
    }

    static func asAesSuccessActionDataResultList(arr: [Any]) throws -> [AesSuccessActionDataResult] {
        var list = [AesSuccessActionDataResult]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var aesSuccessActionDataResult = try asAesSuccessActionDataResult(aesSuccessActionDataResult: val)
                list.append(aesSuccessActionDataResult)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AesSuccessActionDataResult"))
            }
        }
        return list
    }

    static func asAmount(amount: [String: Any?]) throws -> Amount {
        let type = amount["type"] as! String
        if type == "bitcoin" {
            guard let _amountMsat = amount["amountMsat"] as? UInt64 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountMsat", typeName: "Amount"))
            }
            return Amount.bitcoin(amountMsat: _amountMsat)
        }
        if type == "currency" {
            guard let _iso4217Code = amount["iso4217Code"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "iso4217Code", typeName: "Amount"))
            }
            guard let _fractionalAmount = amount["fractionalAmount"] as? UInt64 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "fractionalAmount", typeName: "Amount"))
            }
            return Amount.currency(iso4217Code: _iso4217Code, fractionalAmount: _fractionalAmount)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum Amount")
    }

    static func dictionaryOf(amount: Amount) -> [String: Any?] {
        switch amount {
        case let .bitcoin(
            amountMsat
        ):
            return [
                "type": "bitcoin",
                "amountMsat": amountMsat,
            ]

        case let .currency(
            iso4217Code, fractionalAmount
        ):
            return [
                "type": "currency",
                "iso4217Code": iso4217Code,
                "fractionalAmount": fractionalAmount,
            ]
        }
    }

    static func arrayOf(amountList: [Amount]) -> [Any] {
        return amountList.map { v -> [String: Any?] in return dictionaryOf(amount: v) }
    }

    static func asAmountList(arr: [Any]) throws -> [Amount] {
        var list = [Amount]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var amount = try asAmount(amount: val)
                list.append(amount)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Amount"))
            }
        }
        return list
    }

    static func asBlockchainExplorer(blockchainExplorer: [String: Any?]) throws -> BlockchainExplorer {
        let type = blockchainExplorer["type"] as! String
        if type == "electrum" {
            guard let _url = blockchainExplorer["url"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "BlockchainExplorer"))
            }
            return BlockchainExplorer.electrum(url: _url)
        }
        if type == "esplora" {
            guard let _url = blockchainExplorer["url"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "BlockchainExplorer"))
            }
            guard let _useWaterfalls = blockchainExplorer["useWaterfalls"] as? Bool else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "useWaterfalls", typeName: "BlockchainExplorer"))
            }
            return BlockchainExplorer.esplora(url: _url, useWaterfalls: _useWaterfalls)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum BlockchainExplorer")
    }

    static func dictionaryOf(blockchainExplorer: BlockchainExplorer) -> [String: Any?] {
        switch blockchainExplorer {
        case let .electrum(
            url
        ):
            return [
                "type": "electrum",
                "url": url,
            ]

        case let .esplora(
            url, useWaterfalls
        ):
            return [
                "type": "esplora",
                "url": url,
                "useWaterfalls": useWaterfalls,
            ]
        }
    }

    static func arrayOf(blockchainExplorerList: [BlockchainExplorer]) -> [Any] {
        return blockchainExplorerList.map { v -> [String: Any?] in return dictionaryOf(blockchainExplorer: v) }
    }

    static func asBlockchainExplorerList(arr: [Any]) throws -> [BlockchainExplorer] {
        var list = [BlockchainExplorer]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var blockchainExplorer = try asBlockchainExplorer(blockchainExplorer: val)
                list.append(blockchainExplorer)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BlockchainExplorer"))
            }
        }
        return list
    }

    static func asBuyBitcoinProvider(buyBitcoinProvider: String) throws -> BuyBitcoinProvider {
        switch buyBitcoinProvider {
        case "moonpay":
            return BuyBitcoinProvider.moonpay

        default: throw SdkError.Generic(message: "Invalid variant \(buyBitcoinProvider) for enum BuyBitcoinProvider")
        }
    }

    static func valueOf(buyBitcoinProvider: BuyBitcoinProvider) -> String {
        switch buyBitcoinProvider {
        case .moonpay:
            return "moonpay"
        }
    }

    static func arrayOf(buyBitcoinProviderList: [BuyBitcoinProvider]) -> [String] {
        return buyBitcoinProviderList.map { v -> String in return valueOf(buyBitcoinProvider: v) }
    }

    static func asBuyBitcoinProviderList(arr: [Any]) throws -> [BuyBitcoinProvider] {
        var list = [BuyBitcoinProvider]()
        for value in arr {
            if let val = value as? String {
                var buyBitcoinProvider = try asBuyBitcoinProvider(buyBitcoinProvider: val)
                list.append(buyBitcoinProvider)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BuyBitcoinProvider"))
            }
        }
        return list
    }

    static func asGetPaymentRequest(getPaymentRequest: [String: Any?]) throws -> GetPaymentRequest {
        let type = getPaymentRequest["type"] as! String
        if type == "paymentHash" {
            guard let _paymentHash = getPaymentRequest["paymentHash"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentHash", typeName: "GetPaymentRequest"))
            }
            return GetPaymentRequest.paymentHash(paymentHash: _paymentHash)
        }
        if type == "swapId" {
            guard let _swapId = getPaymentRequest["swapId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapId", typeName: "GetPaymentRequest"))
            }
            return GetPaymentRequest.swapId(swapId: _swapId)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum GetPaymentRequest")
    }

    static func dictionaryOf(getPaymentRequest: GetPaymentRequest) -> [String: Any?] {
        switch getPaymentRequest {
        case let .paymentHash(
            paymentHash
        ):
            return [
                "type": "paymentHash",
                "paymentHash": paymentHash,
            ]

        case let .swapId(
            swapId
        ):
            return [
                "type": "swapId",
                "swapId": swapId,
            ]
        }
    }

    static func arrayOf(getPaymentRequestList: [GetPaymentRequest]) -> [Any] {
        return getPaymentRequestList.map { v -> [String: Any?] in return dictionaryOf(getPaymentRequest: v) }
    }

    static func asGetPaymentRequestList(arr: [Any]) throws -> [GetPaymentRequest] {
        var list = [GetPaymentRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var getPaymentRequest = try asGetPaymentRequest(getPaymentRequest: val)
                list.append(getPaymentRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "GetPaymentRequest"))
            }
        }
        return list
    }

    static func asInputType(inputType: [String: Any?]) throws -> InputType {
        let type = inputType["type"] as! String
        if type == "bitcoinAddress" {
            guard let addressTmp = inputType["address"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "InputType"))
            }
            let _address = try asBitcoinAddressData(bitcoinAddressData: addressTmp)

            return InputType.bitcoinAddress(address: _address)
        }
        if type == "liquidAddress" {
            guard let addressTmp = inputType["address"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "InputType"))
            }
            let _address = try asLiquidAddressData(liquidAddressData: addressTmp)

            return InputType.liquidAddress(address: _address)
        }
        if type == "bolt11" {
            guard let invoiceTmp = inputType["invoice"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "InputType"))
            }
            let _invoice = try asLnInvoice(lnInvoice: invoiceTmp)

            return InputType.bolt11(invoice: _invoice)
        }
        if type == "bolt12Offer" {
            guard let offerTmp = inputType["offer"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "offer", typeName: "InputType"))
            }
            let _offer = try asLnOffer(lnOffer: offerTmp)

            let _bip353Address = inputType["bip353Address"] as? String

            return InputType.bolt12Offer(offer: _offer, bip353Address: _bip353Address)
        }
        if type == "nodeId" {
            guard let _nodeId = inputType["nodeId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "nodeId", typeName: "InputType"))
            }
            return InputType.nodeId(nodeId: _nodeId)
        }
        if type == "url" {
            guard let _url = inputType["url"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "InputType"))
            }
            return InputType.url(url: _url)
        }
        if type == "lnUrlPay" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlPayRequestData(lnUrlPayRequestData: dataTmp)

            let _bip353Address = inputType["bip353Address"] as? String

            return InputType.lnUrlPay(data: _data, bip353Address: _bip353Address)
        }
        if type == "lnUrlWithdraw" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: dataTmp)

            return InputType.lnUrlWithdraw(data: _data)
        }
        if type == "lnUrlAuth" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlAuthRequestData(lnUrlAuthRequestData: dataTmp)

            return InputType.lnUrlAuth(data: _data)
        }
        if type == "lnUrlError" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return InputType.lnUrlError(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum InputType")
    }

    static func dictionaryOf(inputType: InputType) -> [String: Any?] {
        switch inputType {
        case let .bitcoinAddress(
            address
        ):
            return [
                "type": "bitcoinAddress",
                "address": dictionaryOf(bitcoinAddressData: address),
            ]

        case let .liquidAddress(
            address
        ):
            return [
                "type": "liquidAddress",
                "address": dictionaryOf(liquidAddressData: address),
            ]

        case let .bolt11(
            invoice
        ):
            return [
                "type": "bolt11",
                "invoice": dictionaryOf(lnInvoice: invoice),
            ]

        case let .bolt12Offer(
            offer, bip353Address
        ):
            return [
                "type": "bolt12Offer",
                "offer": dictionaryOf(lnOffer: offer),
                "bip353Address": bip353Address == nil ? nil : bip353Address,
            ]

        case let .nodeId(
            nodeId
        ):
            return [
                "type": "nodeId",
                "nodeId": nodeId,
            ]

        case let .url(
            url
        ):
            return [
                "type": "url",
                "url": url,
            ]

        case let .lnUrlPay(
            data, bip353Address
        ):
            return [
                "type": "lnUrlPay",
                "data": dictionaryOf(lnUrlPayRequestData: data),
                "bip353Address": bip353Address == nil ? nil : bip353Address,
            ]

        case let .lnUrlWithdraw(
            data
        ):
            return [
                "type": "lnUrlWithdraw",
                "data": dictionaryOf(lnUrlWithdrawRequestData: data),
            ]

        case let .lnUrlAuth(
            data
        ):
            return [
                "type": "lnUrlAuth",
                "data": dictionaryOf(lnUrlAuthRequestData: data),
            ]

        case let .lnUrlError(
            data
        ):
            return [
                "type": "lnUrlError",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func arrayOf(inputTypeList: [InputType]) -> [Any] {
        return inputTypeList.map { v -> [String: Any?] in return dictionaryOf(inputType: v) }
    }

    static func asInputTypeList(arr: [Any]) throws -> [InputType] {
        var list = [InputType]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var inputType = try asInputType(inputType: val)
                list.append(inputType)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "InputType"))
            }
        }
        return list
    }

    static func asLiquidNetwork(liquidNetwork: String) throws -> LiquidNetwork {
        switch liquidNetwork {
        case "mainnet":
            return LiquidNetwork.mainnet

        case "testnet":
            return LiquidNetwork.testnet

        case "regtest":
            return LiquidNetwork.regtest

        default: throw SdkError.Generic(message: "Invalid variant \(liquidNetwork) for enum LiquidNetwork")
        }
    }

    static func valueOf(liquidNetwork: LiquidNetwork) -> String {
        switch liquidNetwork {
        case .mainnet:
            return "mainnet"

        case .testnet:
            return "testnet"

        case .regtest:
            return "regtest"
        }
    }

    static func arrayOf(liquidNetworkList: [LiquidNetwork]) -> [String] {
        return liquidNetworkList.map { v -> String in return valueOf(liquidNetwork: v) }
    }

    static func asLiquidNetworkList(arr: [Any]) throws -> [LiquidNetwork] {
        var list = [LiquidNetwork]()
        for value in arr {
            if let val = value as? String {
                var liquidNetwork = try asLiquidNetwork(liquidNetwork: val)
                list.append(liquidNetwork)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LiquidNetwork"))
            }
        }
        return list
    }

    static func asListPaymentDetails(listPaymentDetails: [String: Any?]) throws -> ListPaymentDetails {
        let type = listPaymentDetails["type"] as! String
        if type == "liquid" {
            let _assetId = listPaymentDetails["assetId"] as? String

            let _destination = listPaymentDetails["destination"] as? String

            return ListPaymentDetails.liquid(assetId: _assetId, destination: _destination)
        }
        if type == "bitcoin" {
            let _address = listPaymentDetails["address"] as? String

            return ListPaymentDetails.bitcoin(address: _address)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum ListPaymentDetails")
    }

    static func dictionaryOf(listPaymentDetails: ListPaymentDetails) -> [String: Any?] {
        switch listPaymentDetails {
        case let .liquid(
            assetId, destination
        ):
            return [
                "type": "liquid",
                "assetId": assetId == nil ? nil : assetId,
                "destination": destination == nil ? nil : destination,
            ]

        case let .bitcoin(
            address
        ):
            return [
                "type": "bitcoin",
                "address": address == nil ? nil : address,
            ]
        }
    }

    static func arrayOf(listPaymentDetailsList: [ListPaymentDetails]) -> [Any] {
        return listPaymentDetailsList.map { v -> [String: Any?] in return dictionaryOf(listPaymentDetails: v) }
    }

    static func asListPaymentDetailsList(arr: [Any]) throws -> [ListPaymentDetails] {
        var list = [ListPaymentDetails]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var listPaymentDetails = try asListPaymentDetails(listPaymentDetails: val)
                list.append(listPaymentDetails)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ListPaymentDetails"))
            }
        }
        return list
    }

    static func asLnUrlCallbackStatus(lnUrlCallbackStatus: [String: Any?]) throws -> LnUrlCallbackStatus {
        let type = lnUrlCallbackStatus["type"] as! String
        if type == "ok" {
            return LnUrlCallbackStatus.ok
        }
        if type == "errorStatus" {
            guard let dataTmp = lnUrlCallbackStatus["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlCallbackStatus"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return LnUrlCallbackStatus.errorStatus(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum LnUrlCallbackStatus")
    }

    static func dictionaryOf(lnUrlCallbackStatus: LnUrlCallbackStatus) -> [String: Any?] {
        switch lnUrlCallbackStatus {
        case .ok:
            return [
                "type": "ok",
            ]

        case let .errorStatus(
            data
        ):
            return [
                "type": "errorStatus",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func arrayOf(lnUrlCallbackStatusList: [LnUrlCallbackStatus]) -> [Any] {
        return lnUrlCallbackStatusList.map { v -> [String: Any?] in return dictionaryOf(lnUrlCallbackStatus: v) }
    }

    static func asLnUrlCallbackStatusList(arr: [Any]) throws -> [LnUrlCallbackStatus] {
        var list = [LnUrlCallbackStatus]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlCallbackStatus = try asLnUrlCallbackStatus(lnUrlCallbackStatus: val)
                list.append(lnUrlCallbackStatus)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlCallbackStatus"))
            }
        }
        return list
    }

    static func asLnUrlPayResult(lnUrlPayResult: [String: Any?]) throws -> LnUrlPayResult {
        let type = lnUrlPayResult["type"] as! String
        if type == "endpointSuccess" {
            guard let dataTmp = lnUrlPayResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlPayResult"))
            }
            let _data = try asLnUrlPaySuccessData(lnUrlPaySuccessData: dataTmp)

            return LnUrlPayResult.endpointSuccess(data: _data)
        }
        if type == "endpointError" {
            guard let dataTmp = lnUrlPayResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlPayResult"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return LnUrlPayResult.endpointError(data: _data)
        }
        if type == "payError" {
            guard let dataTmp = lnUrlPayResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlPayResult"))
            }
            let _data = try asLnUrlPayErrorData(lnUrlPayErrorData: dataTmp)

            return LnUrlPayResult.payError(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum LnUrlPayResult")
    }

    static func dictionaryOf(lnUrlPayResult: LnUrlPayResult) -> [String: Any?] {
        switch lnUrlPayResult {
        case let .endpointSuccess(
            data
        ):
            return [
                "type": "endpointSuccess",
                "data": dictionaryOf(lnUrlPaySuccessData: data),
            ]

        case let .endpointError(
            data
        ):
            return [
                "type": "endpointError",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]

        case let .payError(
            data
        ):
            return [
                "type": "payError",
                "data": dictionaryOf(lnUrlPayErrorData: data),
            ]
        }
    }

    static func arrayOf(lnUrlPayResultList: [LnUrlPayResult]) -> [Any] {
        return lnUrlPayResultList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayResult: v) }
    }

    static func asLnUrlPayResultList(arr: [Any]) throws -> [LnUrlPayResult] {
        var list = [LnUrlPayResult]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayResult = try asLnUrlPayResult(lnUrlPayResult: val)
                list.append(lnUrlPayResult)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayResult"))
            }
        }
        return list
    }

    static func asLnUrlWithdrawResult(lnUrlWithdrawResult: [String: Any?]) throws -> LnUrlWithdrawResult {
        let type = lnUrlWithdrawResult["type"] as! String
        if type == "ok" {
            guard let dataTmp = lnUrlWithdrawResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawResult"))
            }
            let _data = try asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: dataTmp)

            return LnUrlWithdrawResult.ok(data: _data)
        }
        if type == "timeout" {
            guard let dataTmp = lnUrlWithdrawResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawResult"))
            }
            let _data = try asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: dataTmp)

            return LnUrlWithdrawResult.timeout(data: _data)
        }
        if type == "errorStatus" {
            guard let dataTmp = lnUrlWithdrawResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawResult"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return LnUrlWithdrawResult.errorStatus(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum LnUrlWithdrawResult")
    }

    static func dictionaryOf(lnUrlWithdrawResult: LnUrlWithdrawResult) -> [String: Any?] {
        switch lnUrlWithdrawResult {
        case let .ok(
            data
        ):
            return [
                "type": "ok",
                "data": dictionaryOf(lnUrlWithdrawSuccessData: data),
            ]

        case let .timeout(
            data
        ):
            return [
                "type": "timeout",
                "data": dictionaryOf(lnUrlWithdrawSuccessData: data),
            ]

        case let .errorStatus(
            data
        ):
            return [
                "type": "errorStatus",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func arrayOf(lnUrlWithdrawResultList: [LnUrlWithdrawResult]) -> [Any] {
        return lnUrlWithdrawResultList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawResult: v) }
    }

    static func asLnUrlWithdrawResultList(arr: [Any]) throws -> [LnUrlWithdrawResult] {
        var list = [LnUrlWithdrawResult]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawResult = try asLnUrlWithdrawResult(lnUrlWithdrawResult: val)
                list.append(lnUrlWithdrawResult)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawResult"))
            }
        }
        return list
    }

    static func asNetwork(network: String) throws -> Network {
        switch network {
        case "bitcoin":
            return Network.bitcoin

        case "testnet":
            return Network.testnet

        case "signet":
            return Network.signet

        case "regtest":
            return Network.regtest

        default: throw SdkError.Generic(message: "Invalid variant \(network) for enum Network")
        }
    }

    static func valueOf(network: Network) -> String {
        switch network {
        case .bitcoin:
            return "bitcoin"

        case .testnet:
            return "testnet"

        case .signet:
            return "signet"

        case .regtest:
            return "regtest"
        }
    }

    static func arrayOf(networkList: [Network]) -> [String] {
        return networkList.map { v -> String in return valueOf(network: v) }
    }

    static func asNetworkList(arr: [Any]) throws -> [Network] {
        var list = [Network]()
        for value in arr {
            if let val = value as? String {
                var network = try asNetwork(network: val)
                list.append(network)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Network"))
            }
        }
        return list
    }

    static func asPayAmount(payAmount: [String: Any?]) throws -> PayAmount {
        let type = payAmount["type"] as! String
        if type == "bitcoin" {
            guard let _receiverAmountSat = payAmount["receiverAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "PayAmount"))
            }
            return PayAmount.bitcoin(receiverAmountSat: _receiverAmountSat)
        }
        if type == "asset" {
            guard let _toAsset = payAmount["toAsset"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "toAsset", typeName: "PayAmount"))
            }
            guard let _receiverAmount = payAmount["receiverAmount"] as? Double else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmount", typeName: "PayAmount"))
            }
            let _estimateAssetFees = payAmount["estimateAssetFees"] as? Bool

            let _fromAsset = payAmount["fromAsset"] as? String

            return PayAmount.asset(toAsset: _toAsset, receiverAmount: _receiverAmount, estimateAssetFees: _estimateAssetFees, fromAsset: _fromAsset)
        }
        if type == "drain" {
            return PayAmount.drain
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum PayAmount")
    }

    static func dictionaryOf(payAmount: PayAmount) -> [String: Any?] {
        switch payAmount {
        case let .bitcoin(
            receiverAmountSat
        ):
            return [
                "type": "bitcoin",
                "receiverAmountSat": receiverAmountSat,
            ]

        case let .asset(
            toAsset, receiverAmount, estimateAssetFees, fromAsset
        ):
            return [
                "type": "asset",
                "toAsset": toAsset,
                "receiverAmount": receiverAmount,
                "estimateAssetFees": estimateAssetFees == nil ? nil : estimateAssetFees,
                "fromAsset": fromAsset == nil ? nil : fromAsset,
            ]

        case .drain:
            return [
                "type": "drain",
            ]
        }
    }

    static func arrayOf(payAmountList: [PayAmount]) -> [Any] {
        return payAmountList.map { v -> [String: Any?] in return dictionaryOf(payAmount: v) }
    }

    static func asPayAmountList(arr: [Any]) throws -> [PayAmount] {
        var list = [PayAmount]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var payAmount = try asPayAmount(payAmount: val)
                list.append(payAmount)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PayAmount"))
            }
        }
        return list
    }

    static func asPaymentDetails(paymentDetails: [String: Any?]) throws -> PaymentDetails {
        let type = paymentDetails["type"] as! String
        if type == "lightning" {
            guard let _swapId = paymentDetails["swapId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapId", typeName: "PaymentDetails"))
            }
            guard let _description = paymentDetails["description"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "PaymentDetails"))
            }
            guard let _liquidExpirationBlockheight = paymentDetails["liquidExpirationBlockheight"] as? UInt32 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "liquidExpirationBlockheight", typeName: "PaymentDetails"))
            }
            let _preimage = paymentDetails["preimage"] as? String

            let _invoice = paymentDetails["invoice"] as? String

            let _bolt12Offer = paymentDetails["bolt12Offer"] as? String

            let _paymentHash = paymentDetails["paymentHash"] as? String

            let _destinationPubkey = paymentDetails["destinationPubkey"] as? String

            var _lnurlInfo: LnUrlInfo?
            if let lnurlInfoTmp = paymentDetails["lnurlInfo"] as? [String: Any?] {
                _lnurlInfo = try asLnUrlInfo(lnUrlInfo: lnurlInfoTmp)
            }

            let _bip353Address = paymentDetails["bip353Address"] as? String

            let _payerNote = paymentDetails["payerNote"] as? String

            let _claimTxId = paymentDetails["claimTxId"] as? String

            let _refundTxId = paymentDetails["refundTxId"] as? String

            let _refundTxAmountSat = paymentDetails["refundTxAmountSat"] as? UInt64

            return PaymentDetails.lightning(swapId: _swapId, description: _description, liquidExpirationBlockheight: _liquidExpirationBlockheight, preimage: _preimage, invoice: _invoice, bolt12Offer: _bolt12Offer, paymentHash: _paymentHash, destinationPubkey: _destinationPubkey, lnurlInfo: _lnurlInfo, bip353Address: _bip353Address, payerNote: _payerNote, claimTxId: _claimTxId, refundTxId: _refundTxId, refundTxAmountSat: _refundTxAmountSat)
        }
        if type == "liquid" {
            guard let _assetId = paymentDetails["assetId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "assetId", typeName: "PaymentDetails"))
            }
            guard let _destination = paymentDetails["destination"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "destination", typeName: "PaymentDetails"))
            }
            guard let _description = paymentDetails["description"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "PaymentDetails"))
            }
            var _assetInfo: AssetInfo?
            if let assetInfoTmp = paymentDetails["assetInfo"] as? [String: Any?] {
                _assetInfo = try asAssetInfo(assetInfo: assetInfoTmp)
            }

            var _lnurlInfo: LnUrlInfo?
            if let lnurlInfoTmp = paymentDetails["lnurlInfo"] as? [String: Any?] {
                _lnurlInfo = try asLnUrlInfo(lnUrlInfo: lnurlInfoTmp)
            }

            let _bip353Address = paymentDetails["bip353Address"] as? String

            let _payerNote = paymentDetails["payerNote"] as? String

            return PaymentDetails.liquid(assetId: _assetId, destination: _destination, description: _description, assetInfo: _assetInfo, lnurlInfo: _lnurlInfo, bip353Address: _bip353Address, payerNote: _payerNote)
        }
        if type == "bitcoin" {
            guard let _swapId = paymentDetails["swapId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapId", typeName: "PaymentDetails"))
            }
            guard let _bitcoinAddress = paymentDetails["bitcoinAddress"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bitcoinAddress", typeName: "PaymentDetails"))
            }
            guard let _description = paymentDetails["description"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "PaymentDetails"))
            }
            guard let _autoAcceptedFees = paymentDetails["autoAcceptedFees"] as? Bool else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "autoAcceptedFees", typeName: "PaymentDetails"))
            }
            guard let _bitcoinExpirationBlockheight = paymentDetails["bitcoinExpirationBlockheight"] as? UInt32 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bitcoinExpirationBlockheight", typeName: "PaymentDetails"))
            }
            guard let _liquidExpirationBlockheight = paymentDetails["liquidExpirationBlockheight"] as? UInt32 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "liquidExpirationBlockheight", typeName: "PaymentDetails"))
            }
            let _lockupTxId = paymentDetails["lockupTxId"] as? String

            let _claimTxId = paymentDetails["claimTxId"] as? String

            let _refundTxId = paymentDetails["refundTxId"] as? String

            let _refundTxAmountSat = paymentDetails["refundTxAmountSat"] as? UInt64

            return PaymentDetails.bitcoin(swapId: _swapId, bitcoinAddress: _bitcoinAddress, description: _description, autoAcceptedFees: _autoAcceptedFees, bitcoinExpirationBlockheight: _bitcoinExpirationBlockheight, liquidExpirationBlockheight: _liquidExpirationBlockheight, lockupTxId: _lockupTxId, claimTxId: _claimTxId, refundTxId: _refundTxId, refundTxAmountSat: _refundTxAmountSat)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum PaymentDetails")
    }

    static func dictionaryOf(paymentDetails: PaymentDetails) -> [String: Any?] {
        switch paymentDetails {
        case let .lightning(
            swapId, description, liquidExpirationBlockheight, preimage, invoice, bolt12Offer, paymentHash, destinationPubkey, lnurlInfo, bip353Address, payerNote, claimTxId, refundTxId, refundTxAmountSat
        ):
            return [
                "type": "lightning",
                "swapId": swapId,
                "description": description,
                "liquidExpirationBlockheight": liquidExpirationBlockheight,
                "preimage": preimage == nil ? nil : preimage,
                "invoice": invoice == nil ? nil : invoice,
                "bolt12Offer": bolt12Offer == nil ? nil : bolt12Offer,
                "paymentHash": paymentHash == nil ? nil : paymentHash,
                "destinationPubkey": destinationPubkey == nil ? nil : destinationPubkey,
                "lnurlInfo": lnurlInfo == nil ? nil : dictionaryOf(lnUrlInfo: lnurlInfo!),
                "bip353Address": bip353Address == nil ? nil : bip353Address,
                "payerNote": payerNote == nil ? nil : payerNote,
                "claimTxId": claimTxId == nil ? nil : claimTxId,
                "refundTxId": refundTxId == nil ? nil : refundTxId,
                "refundTxAmountSat": refundTxAmountSat == nil ? nil : refundTxAmountSat,
            ]

        case let .liquid(
            assetId, destination, description, assetInfo, lnurlInfo, bip353Address, payerNote
        ):
            return [
                "type": "liquid",
                "assetId": assetId,
                "destination": destination,
                "description": description,
                "assetInfo": assetInfo == nil ? nil : dictionaryOf(assetInfo: assetInfo!),
                "lnurlInfo": lnurlInfo == nil ? nil : dictionaryOf(lnUrlInfo: lnurlInfo!),
                "bip353Address": bip353Address == nil ? nil : bip353Address,
                "payerNote": payerNote == nil ? nil : payerNote,
            ]

        case let .bitcoin(
            swapId, bitcoinAddress, description, autoAcceptedFees, bitcoinExpirationBlockheight, liquidExpirationBlockheight, lockupTxId, claimTxId, refundTxId, refundTxAmountSat
        ):
            return [
                "type": "bitcoin",
                "swapId": swapId,
                "bitcoinAddress": bitcoinAddress,
                "description": description,
                "autoAcceptedFees": autoAcceptedFees,
                "bitcoinExpirationBlockheight": bitcoinExpirationBlockheight,
                "liquidExpirationBlockheight": liquidExpirationBlockheight,
                "lockupTxId": lockupTxId == nil ? nil : lockupTxId,
                "claimTxId": claimTxId == nil ? nil : claimTxId,
                "refundTxId": refundTxId == nil ? nil : refundTxId,
                "refundTxAmountSat": refundTxAmountSat == nil ? nil : refundTxAmountSat,
            ]
        }
    }

    static func arrayOf(paymentDetailsList: [PaymentDetails]) -> [Any] {
        return paymentDetailsList.map { v -> [String: Any?] in return dictionaryOf(paymentDetails: v) }
    }

    static func asPaymentDetailsList(arr: [Any]) throws -> [PaymentDetails] {
        var list = [PaymentDetails]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var paymentDetails = try asPaymentDetails(paymentDetails: val)
                list.append(paymentDetails)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PaymentDetails"))
            }
        }
        return list
    }

    static func asPaymentMethod(paymentMethod: String) throws -> PaymentMethod {
        switch paymentMethod {
        case "lightning":
            return PaymentMethod.lightning

        case "bolt11Invoice":
            return PaymentMethod.bolt11Invoice

        case "bolt12Offer":
            return PaymentMethod.bolt12Offer

        case "bitcoinAddress":
            return PaymentMethod.bitcoinAddress

        case "liquidAddress":
            return PaymentMethod.liquidAddress

        default: throw SdkError.Generic(message: "Invalid variant \(paymentMethod) for enum PaymentMethod")
        }
    }

    static func valueOf(paymentMethod: PaymentMethod) -> String {
        switch paymentMethod {
        case .lightning:
            return "lightning"

        case .bolt11Invoice:
            return "bolt11Invoice"

        case .bolt12Offer:
            return "bolt12Offer"

        case .bitcoinAddress:
            return "bitcoinAddress"

        case .liquidAddress:
            return "liquidAddress"
        }
    }

    static func arrayOf(paymentMethodList: [PaymentMethod]) -> [String] {
        return paymentMethodList.map { v -> String in return valueOf(paymentMethod: v) }
    }

    static func asPaymentMethodList(arr: [Any]) throws -> [PaymentMethod] {
        var list = [PaymentMethod]()
        for value in arr {
            if let val = value as? String {
                var paymentMethod = try asPaymentMethod(paymentMethod: val)
                list.append(paymentMethod)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PaymentMethod"))
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

        case "timedOut":
            return PaymentState.timedOut

        case "refundable":
            return PaymentState.refundable

        case "refundPending":
            return PaymentState.refundPending

        case "waitingFeeAcceptance":
            return PaymentState.waitingFeeAcceptance

        default: throw SdkError.Generic(message: "Invalid variant \(paymentState) for enum PaymentState")
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

        case .timedOut:
            return "timedOut"

        case .refundable:
            return "refundable"

        case .refundPending:
            return "refundPending"

        case .waitingFeeAcceptance:
            return "waitingFeeAcceptance"
        }
    }

    static func arrayOf(paymentStateList: [PaymentState]) -> [String] {
        return paymentStateList.map { v -> String in return valueOf(paymentState: v) }
    }

    static func asPaymentStateList(arr: [Any]) throws -> [PaymentState] {
        var list = [PaymentState]()
        for value in arr {
            if let val = value as? String {
                var paymentState = try asPaymentState(paymentState: val)
                list.append(paymentState)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PaymentState"))
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

        default: throw SdkError.Generic(message: "Invalid variant \(paymentType) for enum PaymentType")
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
        return paymentTypeList.map { v -> String in return valueOf(paymentType: v) }
    }

    static func asPaymentTypeList(arr: [Any]) throws -> [PaymentType] {
        var list = [PaymentType]()
        for value in arr {
            if let val = value as? String {
                var paymentType = try asPaymentType(paymentType: val)
                list.append(paymentType)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PaymentType"))
            }
        }
        return list
    }

    static func asReceiveAmount(receiveAmount: [String: Any?]) throws -> ReceiveAmount {
        let type = receiveAmount["type"] as! String
        if type == "bitcoin" {
            guard let _payerAmountSat = receiveAmount["payerAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "ReceiveAmount"))
            }
            return ReceiveAmount.bitcoin(payerAmountSat: _payerAmountSat)
        }
        if type == "asset" {
            guard let _assetId = receiveAmount["assetId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "assetId", typeName: "ReceiveAmount"))
            }
            let _payerAmount = receiveAmount["payerAmount"] as? Double

            return ReceiveAmount.asset(assetId: _assetId, payerAmount: _payerAmount)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum ReceiveAmount")
    }

    static func dictionaryOf(receiveAmount: ReceiveAmount) -> [String: Any?] {
        switch receiveAmount {
        case let .bitcoin(
            payerAmountSat
        ):
            return [
                "type": "bitcoin",
                "payerAmountSat": payerAmountSat,
            ]

        case let .asset(
            assetId, payerAmount
        ):
            return [
                "type": "asset",
                "assetId": assetId,
                "payerAmount": payerAmount == nil ? nil : payerAmount,
            ]
        }
    }

    static func arrayOf(receiveAmountList: [ReceiveAmount]) -> [Any] {
        return receiveAmountList.map { v -> [String: Any?] in return dictionaryOf(receiveAmount: v) }
    }

    static func asReceiveAmountList(arr: [Any]) throws -> [ReceiveAmount] {
        var list = [ReceiveAmount]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receiveAmount = try asReceiveAmount(receiveAmount: val)
                list.append(receiveAmount)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ReceiveAmount"))
            }
        }
        return list
    }

    static func asSdkEvent(sdkEvent: [String: Any?]) throws -> SdkEvent {
        let type = sdkEvent["type"] as! String
        if type == "paymentFailed" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentFailed(details: _details)
        }
        if type == "paymentPending" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentPending(details: _details)
        }
        if type == "paymentRefundable" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentRefundable(details: _details)
        }
        if type == "paymentRefunded" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentRefunded(details: _details)
        }
        if type == "paymentRefundPending" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentRefundPending(details: _details)
        }
        if type == "paymentSucceeded" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentSucceeded(details: _details)
        }
        if type == "paymentWaitingConfirmation" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentWaitingConfirmation(details: _details)
        }
        if type == "paymentWaitingFeeAcceptance" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentWaitingFeeAcceptance(details: _details)
        }
        if type == "synced" {
            return SdkEvent.synced
        }
        if type == "dataSynced" {
            guard let _didPullNewRecords = sdkEvent["didPullNewRecords"] as? Bool else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "didPullNewRecords", typeName: "SdkEvent"))
            }
            return SdkEvent.dataSynced(didPullNewRecords: _didPullNewRecords)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum SdkEvent")
    }

    static func dictionaryOf(sdkEvent: SdkEvent) -> [String: Any?] {
        switch sdkEvent {
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

        case let .paymentRefundable(
            details
        ):
            return [
                "type": "paymentRefundable",
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

        case let .paymentSucceeded(
            details
        ):
            return [
                "type": "paymentSucceeded",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentWaitingConfirmation(
            details
        ):
            return [
                "type": "paymentWaitingConfirmation",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentWaitingFeeAcceptance(
            details
        ):
            return [
                "type": "paymentWaitingFeeAcceptance",
                "details": dictionaryOf(payment: details),
            ]

        case .synced:
            return [
                "type": "synced",
            ]

        case let .dataSynced(
            didPullNewRecords
        ):
            return [
                "type": "dataSynced",
                "didPullNewRecords": didPullNewRecords,
            ]
        }
    }

    static func arrayOf(sdkEventList: [SdkEvent]) -> [Any] {
        return sdkEventList.map { v -> [String: Any?] in return dictionaryOf(sdkEvent: v) }
    }

    static func asSdkEventList(arr: [Any]) throws -> [SdkEvent] {
        var list = [SdkEvent]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sdkEvent = try asSdkEvent(sdkEvent: val)
                list.append(sdkEvent)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SdkEvent"))
            }
        }
        return list
    }

    static func asSendDestination(sendDestination: [String: Any?]) throws -> SendDestination {
        let type = sendDestination["type"] as! String
        if type == "liquidAddress" {
            guard let addressDataTmp = sendDestination["addressData"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "addressData", typeName: "SendDestination"))
            }
            let _addressData = try asLiquidAddressData(liquidAddressData: addressDataTmp)

            let _bip353Address = sendDestination["bip353Address"] as? String

            return SendDestination.liquidAddress(addressData: _addressData, bip353Address: _bip353Address)
        }
        if type == "bolt11" {
            guard let invoiceTmp = sendDestination["invoice"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "SendDestination"))
            }
            let _invoice = try asLnInvoice(lnInvoice: invoiceTmp)

            let _bip353Address = sendDestination["bip353Address"] as? String

            return SendDestination.bolt11(invoice: _invoice, bip353Address: _bip353Address)
        }
        if type == "bolt12" {
            guard let offerTmp = sendDestination["offer"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "offer", typeName: "SendDestination"))
            }
            let _offer = try asLnOffer(lnOffer: offerTmp)

            guard let _receiverAmountSat = sendDestination["receiverAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "SendDestination"))
            }
            let _bip353Address = sendDestination["bip353Address"] as? String

            return SendDestination.bolt12(offer: _offer, receiverAmountSat: _receiverAmountSat, bip353Address: _bip353Address)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum SendDestination")
    }

    static func dictionaryOf(sendDestination: SendDestination) -> [String: Any?] {
        switch sendDestination {
        case let .liquidAddress(
            addressData, bip353Address
        ):
            return [
                "type": "liquidAddress",
                "addressData": dictionaryOf(liquidAddressData: addressData),
                "bip353Address": bip353Address == nil ? nil : bip353Address,
            ]

        case let .bolt11(
            invoice, bip353Address
        ):
            return [
                "type": "bolt11",
                "invoice": dictionaryOf(lnInvoice: invoice),
                "bip353Address": bip353Address == nil ? nil : bip353Address,
            ]

        case let .bolt12(
            offer, receiverAmountSat, bip353Address
        ):
            return [
                "type": "bolt12",
                "offer": dictionaryOf(lnOffer: offer),
                "receiverAmountSat": receiverAmountSat,
                "bip353Address": bip353Address == nil ? nil : bip353Address,
            ]
        }
    }

    static func arrayOf(sendDestinationList: [SendDestination]) -> [Any] {
        return sendDestinationList.map { v -> [String: Any?] in return dictionaryOf(sendDestination: v) }
    }

    static func asSendDestinationList(arr: [Any]) throws -> [SendDestination] {
        var list = [SendDestination]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sendDestination = try asSendDestination(sendDestination: val)
                list.append(sendDestination)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SendDestination"))
            }
        }
        return list
    }

    static func asSuccessAction(successAction: [String: Any?]) throws -> SuccessAction {
        let type = successAction["type"] as! String
        if type == "aes" {
            guard let dataTmp = successAction["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "SuccessAction"))
            }
            let _data = try asAesSuccessActionData(aesSuccessActionData: dataTmp)

            return SuccessAction.aes(data: _data)
        }
        if type == "message" {
            guard let dataTmp = successAction["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "SuccessAction"))
            }
            let _data = try asMessageSuccessActionData(messageSuccessActionData: dataTmp)

            return SuccessAction.message(data: _data)
        }
        if type == "url" {
            guard let dataTmp = successAction["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "SuccessAction"))
            }
            let _data = try asUrlSuccessActionData(urlSuccessActionData: dataTmp)

            return SuccessAction.url(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum SuccessAction")
    }

    static func dictionaryOf(successAction: SuccessAction) -> [String: Any?] {
        switch successAction {
        case let .aes(
            data
        ):
            return [
                "type": "aes",
                "data": dictionaryOf(aesSuccessActionData: data),
            ]

        case let .message(
            data
        ):
            return [
                "type": "message",
                "data": dictionaryOf(messageSuccessActionData: data),
            ]

        case let .url(
            data
        ):
            return [
                "type": "url",
                "data": dictionaryOf(urlSuccessActionData: data),
            ]
        }
    }

    static func arrayOf(successActionList: [SuccessAction]) -> [Any] {
        return successActionList.map { v -> [String: Any?] in return dictionaryOf(successAction: v) }
    }

    static func asSuccessActionList(arr: [Any]) throws -> [SuccessAction] {
        var list = [SuccessAction]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var successAction = try asSuccessAction(successAction: val)
                list.append(successAction)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SuccessAction"))
            }
        }
        return list
    }

    static func asSuccessActionProcessed(successActionProcessed: [String: Any?]) throws -> SuccessActionProcessed {
        let type = successActionProcessed["type"] as! String
        if type == "aes" {
            guard let resultTmp = successActionProcessed["result"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "result", typeName: "SuccessActionProcessed"))
            }
            let _result = try asAesSuccessActionDataResult(aesSuccessActionDataResult: resultTmp)

            return SuccessActionProcessed.aes(result: _result)
        }
        if type == "message" {
            guard let dataTmp = successActionProcessed["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "SuccessActionProcessed"))
            }
            let _data = try asMessageSuccessActionData(messageSuccessActionData: dataTmp)

            return SuccessActionProcessed.message(data: _data)
        }
        if type == "url" {
            guard let dataTmp = successActionProcessed["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "SuccessActionProcessed"))
            }
            let _data = try asUrlSuccessActionData(urlSuccessActionData: dataTmp)

            return SuccessActionProcessed.url(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum SuccessActionProcessed")
    }

    static func dictionaryOf(successActionProcessed: SuccessActionProcessed) -> [String: Any?] {
        switch successActionProcessed {
        case let .aes(
            result
        ):
            return [
                "type": "aes",
                "result": dictionaryOf(aesSuccessActionDataResult: result),
            ]

        case let .message(
            data
        ):
            return [
                "type": "message",
                "data": dictionaryOf(messageSuccessActionData: data),
            ]

        case let .url(
            data
        ):
            return [
                "type": "url",
                "data": dictionaryOf(urlSuccessActionData: data),
            ]
        }
    }

    static func arrayOf(successActionProcessedList: [SuccessActionProcessed]) -> [Any] {
        return successActionProcessedList.map { v -> [String: Any?] in return dictionaryOf(successActionProcessed: v) }
    }

    static func asSuccessActionProcessedList(arr: [Any]) throws -> [SuccessActionProcessed] {
        var list = [SuccessActionProcessed]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var successActionProcessed = try asSuccessActionProcessed(successActionProcessed: val)
                list.append(successActionProcessed)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SuccessActionProcessed"))
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
