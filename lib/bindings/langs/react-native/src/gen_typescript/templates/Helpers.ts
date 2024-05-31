
export type EventListener = (e: LiquidSdkEvent) => void

export type Logger = (logEntry: LogEntry) => void

export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezLiquidSDK.connect(req)
    return response
}

export const addEventListener = async (listener: EventListener): Promise<string> => {
    const response = await BreezLiquidSDK.addEventListener()
    BreezLiquidSDKEmitter.addListener(`event-${response}`, listener)
    
    return response
}

export const setLogger = async (logger: Logger): Promise<EmitterSubscription> => {
    const subscription = BreezLiquidSDKEmitter.addListener("breezLiquidSdkLog", logger)

    try {
        await BreezLiquidSDK.setLogger()
    } catch {}

    return subscription
}