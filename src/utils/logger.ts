import {
  warn,
  debug,
  trace,
  info,
  error,
  LogOptions,
} from "@tauri-apps/plugin-log";

export default function log(
  message: string,
  level: "warn" | "debug" | "info" | "error" | "trace" = "debug",
  options: LogOptions | undefined = undefined
) {
  const func = {
    warn: warn,
    debug: debug,
    info: info,
    error: error,
    trace: trace,
  };
  func[level](message, options);
}
