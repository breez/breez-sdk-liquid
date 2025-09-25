package com.breez.breez_sdk_liquid

import breez_sdk_liquid.LogEntry
import breez_sdk_liquid.Logger
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

class SdkLogListener : Logger {
    private val scope = CoroutineScope(SupervisorJob())

    private val _logEvents = MutableSharedFlow<LogEntry>()
    private val logEvents: SharedFlow<LogEntry> = _logEvents.asSharedFlow()

    override fun log(l: LogEntry) {
        scope.launch {
            _logEvents.emit(l)
        }
    }

    fun subscribe(scope: CoroutineScope, block: suspend (LogEntry) -> Unit) =
        logEvents.onEach(block).launchIn(scope)

    fun unsubscribe(scope: CoroutineScope) = scope.cancel()
}