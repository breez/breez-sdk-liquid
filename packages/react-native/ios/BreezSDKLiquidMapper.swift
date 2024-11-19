import BreezSDKLiquid
import Foundation

enum BreezSDKLiquidMapper {
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
        guard let liquidElectrumUrl = config["liquidElectrumUrl"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "liquidElectrumUrl", typeName: "Config"))
        }
        guard let bitcoinElectrumUrl = config["bitcoinElectrumUrl"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bitcoinElectrumUrl", typeName: "Config"))
        }
        guard let mempoolspaceUrl = config["mempoolspaceUrl"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "mempoolspaceUrl", typeName: "Config"))
        }
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
        guard let zeroConfMinFeeRateMsat = config["zeroConfMinFeeRateMsat"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "zeroConfMinFeeRateMsat", typeName: "Config"))
        }
        var breezApiKey: String?
        if hasNonNilKey(data: config, key: "breezApiKey") {
            guard let breezApiKeyTmp = config["breezApiKey"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "breezApiKey"))
            }
            breezApiKey = breezApiKeyTmp
        }
        var cacheDir: String?
        if hasNonNilKey(data: config, key: "cacheDir") {
            guard let cacheDirTmp = config["cacheDir"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "cacheDir"))
            }
            cacheDir = cacheDirTmp
        }
        var zeroConfMaxAmountSat: UInt64?
        if hasNonNilKey(data: config, key: "zeroConfMaxAmountSat") {
            guard let zeroConfMaxAmountSatTmp = config["zeroConfMaxAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "zeroConfMaxAmountSat"))
            }
            zeroConfMaxAmountSat = zeroConfMaxAmountSatTmp
        }

        return Config(liquidElectrumUrl: liquidElectrumUrl, bitcoinElectrumUrl: bitcoinElectrumUrl, mempoolspaceUrl: mempoolspaceUrl, workingDir: workingDir, network: network, paymentTimeoutSec: paymentTimeoutSec, zeroConfMinFeeRateMsat: zeroConfMinFeeRateMsat, breezApiKey: breezApiKey, cacheDir: cacheDir, zeroConfMaxAmountSat: zeroConfMaxAmountSat)
    }

    static func dictionaryOf(config: Config) -> [String: Any?] {
        return [
            "liquidElectrumUrl": config.liquidElectrumUrl,
            "bitcoinElectrumUrl": config.bitcoinElectrumUrl,
            "mempoolspaceUrl": config.mempoolspaceUrl,
            "workingDir": config.workingDir,
            "network": valueOf(liquidNetwork: config.network),
            "paymentTimeoutSec": config.paymentTimeoutSec,
            "zeroConfMinFeeRateMsat": config.zeroConfMinFeeRateMsat,
            "breezApiKey": config.breezApiKey == nil ? nil : config.breezApiKey,
            "cacheDir": config.cacheDir == nil ? nil : config.cacheDir,
            "zeroConfMaxAmountSat": config.zeroConfMaxAmountSat == nil ? nil : config.zeroConfMaxAmountSat,
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

        guard let mnemonic = connectRequest["mnemonic"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "mnemonic", typeName: "ConnectRequest"))
        }

        return ConnectRequest(config: config, mnemonic: mnemonic)
    }

    static func dictionaryOf(connectRequest: ConnectRequest) -> [String: Any?] {
        return [
            "config": dictionaryOf(config: connectRequest.config),
            "mnemonic": connectRequest.mnemonic,
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
        guard let balanceSat = getInfoResponse["balanceSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "balanceSat", typeName: "GetInfoResponse"))
        }
        guard let pendingSendSat = getInfoResponse["pendingSendSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingSendSat", typeName: "GetInfoResponse"))
        }
        guard let pendingReceiveSat = getInfoResponse["pendingReceiveSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingReceiveSat", typeName: "GetInfoResponse"))
        }
        guard let fingerprint = getInfoResponse["fingerprint"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "fingerprint", typeName: "GetInfoResponse"))
        }
        guard let pubkey = getInfoResponse["pubkey"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pubkey", typeName: "GetInfoResponse"))
        }

        return GetInfoResponse(balanceSat: balanceSat, pendingSendSat: pendingSendSat, pendingReceiveSat: pendingReceiveSat, fingerprint: fingerprint, pubkey: pubkey)
    }

    static func dictionaryOf(getInfoResponse: GetInfoResponse) -> [String: Any?] {
        return [
            "balanceSat": getInfoResponse.balanceSat,
            "pendingSendSat": getInfoResponse.pendingSendSat,
            "pendingReceiveSat": getInfoResponse.pendingReceiveSat,
            "fingerprint": getInfoResponse.fingerprint,
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

        return LiquidAddressData(address: address, network: network, assetId: assetId, amountSat: amountSat, label: label, message: message)
    }

    static func dictionaryOf(liquidAddressData: LiquidAddressData) -> [String: Any?] {
        return [
            "address": liquidAddressData.address,
            "network": valueOf(network: liquidAddressData.network),
            "assetId": liquidAddressData.assetId == nil ? nil : liquidAddressData.assetId,
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

        return ListPaymentsRequest(filters: filters, fromTimestamp: fromTimestamp, toTimestamp: toTimestamp, offset: offset, limit: limit, details: details)
    }

    static func dictionaryOf(listPaymentsRequest: ListPaymentsRequest) -> [String: Any?] {
        return [
            "filters": listPaymentsRequest.filters == nil ? nil : arrayOf(paymentTypeList: listPaymentsRequest.filters!),
            "fromTimestamp": listPaymentsRequest.fromTimestamp == nil ? nil : listPaymentsRequest.fromTimestamp,
            "toTimestamp": listPaymentsRequest.toTimestamp == nil ? nil : listPaymentsRequest.toTimestamp,
            "offset": listPaymentsRequest.offset == nil ? nil : listPaymentsRequest.offset,
            "limit": listPaymentsRequest.limit == nil ? nil : listPaymentsRequest.limit,
            "details": listPaymentsRequest.details == nil ? nil : dictionaryOf(listPaymentDetails: listPaymentsRequest.details!),
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

        return Payment(timestamp: timestamp, amountSat: amountSat, feesSat: feesSat, paymentType: paymentType, status: status, details: details, destination: destination, txId: txId)
    }

    static func dictionaryOf(payment: Payment) -> [String: Any?] {
        return [
            "timestamp": payment.timestamp,
            "amountSat": payment.amountSat,
            "feesSat": payment.feesSat,
            "paymentType": valueOf(paymentType: payment.paymentType),
            "status": valueOf(paymentState: payment.status),
            "details": dictionaryOf(paymentDetails: payment.details),
            "destination": payment.destination == nil ? nil : payment.destination,
            "txId": payment.txId == nil ? nil : payment.txId,
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

        guard let amountMsat = prepareLnUrlPayRequest["amountMsat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountMsat", typeName: "PrepareLnUrlPayRequest"))
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

        return PrepareLnUrlPayRequest(data: data, amountMsat: amountMsat, comment: comment, validateSuccessActionUrl: validateSuccessActionUrl)
    }

    static func dictionaryOf(prepareLnUrlPayRequest: PrepareLnUrlPayRequest) -> [String: Any?] {
        return [
            "data": dictionaryOf(lnUrlPayRequestData: prepareLnUrlPayRequest.data),
            "amountMsat": prepareLnUrlPayRequest.amountMsat,
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
        var successAction: SuccessAction?
        if let successActionTmp = prepareLnUrlPayResponse["successAction"] as? [String: Any?] {
            successAction = try asSuccessAction(successAction: successActionTmp)
        }

        return PrepareLnUrlPayResponse(destination: destination, feesSat: feesSat, successAction: successAction)
    }

    static func dictionaryOf(prepareLnUrlPayResponse: PrepareLnUrlPayResponse) -> [String: Any?] {
        return [
            "destination": dictionaryOf(sendDestination: prepareLnUrlPayResponse.destination),
            "feesSat": prepareLnUrlPayResponse.feesSat,
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

        var payerAmountSat: UInt64?
        if hasNonNilKey(data: prepareReceiveRequest, key: "payerAmountSat") {
            guard let payerAmountSatTmp = prepareReceiveRequest["payerAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "payerAmountSat"))
            }
            payerAmountSat = payerAmountSatTmp
        }

        return PrepareReceiveRequest(paymentMethod: paymentMethod, payerAmountSat: payerAmountSat)
    }

    static func dictionaryOf(prepareReceiveRequest: PrepareReceiveRequest) -> [String: Any?] {
        return [
            "paymentMethod": valueOf(paymentMethod: prepareReceiveRequest.paymentMethod),
            "payerAmountSat": prepareReceiveRequest.payerAmountSat == nil ? nil : prepareReceiveRequest.payerAmountSat,
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
        var payerAmountSat: UInt64?
        if hasNonNilKey(data: prepareReceiveResponse, key: "payerAmountSat") {
            guard let payerAmountSatTmp = prepareReceiveResponse["payerAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "payerAmountSat"))
            }
            payerAmountSat = payerAmountSatTmp
        }
        guard let paymentMethodTmp = prepareReceiveResponse["paymentMethod"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentMethod", typeName: "PrepareReceiveResponse"))
        }
        let paymentMethod = try asPaymentMethod(paymentMethod: paymentMethodTmp)

        guard let feesSat = prepareReceiveResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareReceiveResponse"))
        }

        return PrepareReceiveResponse(payerAmountSat: payerAmountSat, paymentMethod: paymentMethod, feesSat: feesSat)
    }

    static func dictionaryOf(prepareReceiveResponse: PrepareReceiveResponse) -> [String: Any?] {
        return [
            "payerAmountSat": prepareReceiveResponse.payerAmountSat == nil ? nil : prepareReceiveResponse.payerAmountSat,
            "paymentMethod": valueOf(paymentMethod: prepareReceiveResponse.paymentMethod),
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
        var refundTxId: String?
        if hasNonNilKey(data: prepareRefundResponse, key: "refundTxId") {
            guard let refundTxIdTmp = prepareRefundResponse["refundTxId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "refundTxId"))
            }
            refundTxId = refundTxIdTmp
        }

        return PrepareRefundResponse(txVsize: txVsize, txFeeSat: txFeeSat, refundTxId: refundTxId)
    }

    static func dictionaryOf(prepareRefundResponse: PrepareRefundResponse) -> [String: Any?] {
        return [
            "txVsize": prepareRefundResponse.txVsize,
            "txFeeSat": prepareRefundResponse.txFeeSat,
            "refundTxId": prepareRefundResponse.refundTxId == nil ? nil : prepareRefundResponse.refundTxId,
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

        guard let feesSat = prepareSendResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareSendResponse"))
        }

        return PrepareSendResponse(destination: destination, feesSat: feesSat)
    }

    static func dictionaryOf(prepareSendResponse: PrepareSendResponse) -> [String: Any?] {
        return [
            "destination": dictionaryOf(sendDestination: prepareSendResponse.destination),
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

        return ReceivePaymentRequest(prepareResponse: prepareResponse, description: description, useDescriptionHash: useDescriptionHash)
    }

    static func dictionaryOf(receivePaymentRequest: ReceivePaymentRequest) -> [String: Any?] {
        return [
            "prepareResponse": dictionaryOf(prepareReceiveResponse: receivePaymentRequest.prepareResponse),
            "description": receivePaymentRequest.description == nil ? nil : receivePaymentRequest.description,
            "useDescriptionHash": receivePaymentRequest.useDescriptionHash == nil ? nil : receivePaymentRequest.useDescriptionHash,
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

        return ReceivePaymentResponse(destination: destination)
    }

    static func dictionaryOf(receivePaymentResponse: ReceivePaymentResponse) -> [String: Any?] {
        return [
            "destination": receivePaymentResponse.destination,
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

        return RefundableSwap(swapAddress: swapAddress, timestamp: timestamp, amountSat: amountSat)
    }

    static func dictionaryOf(refundableSwap: RefundableSwap) -> [String: Any?] {
        return [
            "swapAddress": refundableSwap.swapAddress,
            "timestamp": refundableSwap.timestamp,
            "amountSat": refundableSwap.amountSat,
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

        return SendPaymentRequest(prepareResponse: prepareResponse)
    }

    static func dictionaryOf(sendPaymentRequest: SendPaymentRequest) -> [String: Any?] {
        return [
            "prepareResponse": dictionaryOf(prepareSendResponse: sendPaymentRequest.prepareResponse),
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
        if type == "lightning" {
            guard let _paymentHash = getPaymentRequest["paymentHash"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentHash", typeName: "GetPaymentRequest"))
            }
            return GetPaymentRequest.lightning(paymentHash: _paymentHash)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum GetPaymentRequest")
    }

    static func dictionaryOf(getPaymentRequest: GetPaymentRequest) -> [String: Any?] {
        switch getPaymentRequest {
        case let .lightning(
            paymentHash
        ):
            return [
                "type": "lightning",
                "paymentHash": paymentHash,
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

            return InputType.bolt12Offer(offer: _offer)
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

            return InputType.lnUrlPay(data: _data)
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
            offer
        ):
            return [
                "type": "bolt12Offer",
                "offer": dictionaryOf(lnOffer: offer),
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
            data
        ):
            return [
                "type": "lnUrlPay",
                "data": dictionaryOf(lnUrlPayRequestData: data),
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

        default: throw SdkError.Generic(message: "Invalid variant \(liquidNetwork) for enum LiquidNetwork")
        }
    }

    static func valueOf(liquidNetwork: LiquidNetwork) -> String {
        switch liquidNetwork {
        case .mainnet:
            return "mainnet"

        case .testnet:
            return "testnet"
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
            guard let _destination = listPaymentDetails["destination"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "destination", typeName: "ListPaymentDetails"))
            }
            return ListPaymentDetails.liquid(destination: _destination)
        }
        if type == "bitcoin" {
            guard let _address = listPaymentDetails["address"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "ListPaymentDetails"))
            }
            return ListPaymentDetails.bitcoin(address: _address)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum ListPaymentDetails")
    }

    static func dictionaryOf(listPaymentDetails: ListPaymentDetails) -> [String: Any?] {
        switch listPaymentDetails {
        case let .liquid(
            destination
        ):
            return [
                "type": "liquid",
                "destination": destination,
            ]

        case let .bitcoin(
            address
        ):
            return [
                "type": "bitcoin",
                "address": address,
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
        if type == "receiver" {
            guard let _amountSat = payAmount["amountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "PayAmount"))
            }
            return PayAmount.receiver(amountSat: _amountSat)
        }
        if type == "drain" {
            return PayAmount.drain
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum PayAmount")
    }

    static func dictionaryOf(payAmount: PayAmount) -> [String: Any?] {
        switch payAmount {
        case let .receiver(
            amountSat
        ):
            return [
                "type": "receiver",
                "amountSat": amountSat,
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
            let _preimage = paymentDetails["preimage"] as? String

            let _bolt11 = paymentDetails["bolt11"] as? String

            let _bolt12Offer = paymentDetails["bolt12Offer"] as? String

            let _paymentHash = paymentDetails["paymentHash"] as? String

            let _refundTxId = paymentDetails["refundTxId"] as? String

            let _refundTxAmountSat = paymentDetails["refundTxAmountSat"] as? UInt64

            return PaymentDetails.lightning(swapId: _swapId, description: _description, preimage: _preimage, bolt11: _bolt11, bolt12Offer: _bolt12Offer, paymentHash: _paymentHash, refundTxId: _refundTxId, refundTxAmountSat: _refundTxAmountSat)
        }
        if type == "liquid" {
            guard let _destination = paymentDetails["destination"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "destination", typeName: "PaymentDetails"))
            }
            guard let _description = paymentDetails["description"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "PaymentDetails"))
            }
            return PaymentDetails.liquid(destination: _destination, description: _description)
        }
        if type == "bitcoin" {
            guard let _swapId = paymentDetails["swapId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapId", typeName: "PaymentDetails"))
            }
            guard let _description = paymentDetails["description"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "PaymentDetails"))
            }
            let _refundTxId = paymentDetails["refundTxId"] as? String

            let _refundTxAmountSat = paymentDetails["refundTxAmountSat"] as? UInt64

            return PaymentDetails.bitcoin(swapId: _swapId, description: _description, refundTxId: _refundTxId, refundTxAmountSat: _refundTxAmountSat)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum PaymentDetails")
    }

    static func dictionaryOf(paymentDetails: PaymentDetails) -> [String: Any?] {
        switch paymentDetails {
        case let .lightning(
            swapId, description, preimage, bolt11, bolt12Offer, paymentHash, refundTxId, refundTxAmountSat
        ):
            return [
                "type": "lightning",
                "swapId": swapId,
                "description": description,
                "preimage": preimage == nil ? nil : preimage,
                "bolt11": bolt11 == nil ? nil : bolt11,
                "bolt12Offer": bolt12Offer == nil ? nil : bolt12Offer,
                "paymentHash": paymentHash == nil ? nil : paymentHash,
                "refundTxId": refundTxId == nil ? nil : refundTxId,
                "refundTxAmountSat": refundTxAmountSat == nil ? nil : refundTxAmountSat,
            ]

        case let .liquid(
            destination, description
        ):
            return [
                "type": "liquid",
                "destination": destination,
                "description": description,
            ]

        case let .bitcoin(
            swapId, description, refundTxId, refundTxAmountSat
        ):
            return [
                "type": "bitcoin",
                "swapId": swapId,
                "description": description,
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
        if type == "synced" {
            return SdkEvent.synced
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

        case .synced:
            return [
                "type": "synced",
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

            return SendDestination.liquidAddress(addressData: _addressData)
        }
        if type == "bolt11" {
            guard let invoiceTmp = sendDestination["invoice"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "SendDestination"))
            }
            let _invoice = try asLnInvoice(lnInvoice: invoiceTmp)

            return SendDestination.bolt11(invoice: _invoice)
        }
        if type == "bolt12" {
            guard let offerTmp = sendDestination["offer"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "offer", typeName: "SendDestination"))
            }
            let _offer = try asLnOffer(lnOffer: offerTmp)

            guard let _receiverAmountSat = sendDestination["receiverAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "SendDestination"))
            }
            return SendDestination.bolt12(offer: _offer, receiverAmountSat: _receiverAmountSat)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum SendDestination")
    }

    static func dictionaryOf(sendDestination: SendDestination) -> [String: Any?] {
        switch sendDestination {
        case let .liquidAddress(
            addressData
        ):
            return [
                "type": "liquidAddress",
                "addressData": dictionaryOf(liquidAddressData: addressData),
            ]

        case let .bolt11(
            invoice
        ):
            return [
                "type": "bolt11",
                "invoice": dictionaryOf(lnInvoice: invoice),
            ]

        case let .bolt12(
            offer, receiverAmountSat
        ):
            return [
                "type": "bolt12",
                "offer": dictionaryOf(lnOffer: offer),
                "receiverAmountSat": receiverAmountSat,
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
