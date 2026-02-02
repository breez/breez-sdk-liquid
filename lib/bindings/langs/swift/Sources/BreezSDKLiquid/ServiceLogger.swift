import Foundation
import os.log

#if DEBUG && true
fileprivate var logger = OSLog(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "ServiceLogger"
)
#else
fileprivate var logger = OSLog.disabled
#endif

open class ServiceLogger {
    var logStream: Logger?

    init(logStream: Logger?) {
        self.logStream = logStream
    }

    public func log(tag: String, line: String, level: String) {
        let memoryInfo = memoryUsageString()
        let lineWithMemory = "\(line) [mem: \(memoryInfo)]"

        if let logger = logStream {
            logger.log(l: LogEntry(line: lineWithMemory, level: level))
        } else {
            switch(level) {
                case "ERROR":
                    os_log("[%{public}@] %{public}@", log: logger, type: .error, tag, lineWithMemory)
                    break
                case "INFO", "WARN":
                    os_log("[%{public}@] %{public}@", log: logger, type: .info, tag, lineWithMemory)
                    break
                case "TRACE":
                    os_log("[%{public}@] %{public}@", log: logger, type: .debug, tag, lineWithMemory)
                    break
                default:
                    os_log("[%{public}@] %{public}@", log: logger, type: .default, tag, lineWithMemory)
                    return
            }
        }
    }

    private func memoryUsageString() -> String {
        // Use task_vm_info with phys_footprint - this is what iOS uses for memory limits/jetsam
        var info = task_vm_info_data_t()
        var count = mach_msg_type_number_t(MemoryLayout<task_vm_info_data_t>.size / MemoryLayout<natural_t>.size)
        let result = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: Int(count)) {
                task_info(mach_task_self_, task_flavor_t(TASK_VM_INFO), $0, &count)
            }
        }
        if result == KERN_SUCCESS {
            let footprintMB = Double(info.phys_footprint) / (1024 * 1024)
            return String(format: "%.1fMB", footprintMB)
        }
        return "?"
    }
}
