
export const init = async (mnemonic: string, dataDir: string = "", network: Network): Promise<void> => {
    const response = await LiquidSwapSDK.initBindingWallet(mnemonic, dataDir, network)
    return response
}