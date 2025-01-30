
{%- call ts::docstring(ci.get_callback_interface_definition("EventListener").unwrap(), 0, ci) %}
export type EventListener = (e: SdkEvent) => void
{% call ts::docstring(ci.get_callback_interface_definition("Logger").unwrap(), 0, ci) %}
export type Logger = (logEntry: LogEntry) => void
{% call ts::docstring(ci.get_function_definition("connect").unwrap(), 0, ci) %}
export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezSDKLiquid.connect(req)
    return response
}
{%- let obj = ci.get_object_definition("BindingLiquidSdk").unwrap() %}
{% call ts::docstring(obj.get_method("add_event_listener"), 0, ci) %}
export const addEventListener = async (listener: EventListener): Promise<string> => {
    const response = await BreezSDKLiquid.addEventListener()
    BreezSDKLiquidEmitter.addListener(`event-${response}`, listener)
    
    return response
}
{% call ts::docstring(ci.get_function_definition("set_logger").unwrap(), 0, ci) %}
export const setLogger = async (logger: Logger): Promise<EmitterSubscription> => {
    const subscription = BreezSDKLiquidEmitter.addListener("breezSdkLiquidLog", logger)

    try {
        await BreezSDKLiquid.setLogger()
    } catch {}

    return subscription
}