
export const connect = async (mnemonic: string, dataDir: string = "", network: Network): Promise<void> => {
    const response = await BreezLiquidSDK.connect(mnemonic, dataDir, network)
    return response
}
