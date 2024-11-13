const warningMap = new Map<string, boolean>();

export function logPrefix(): string {
  return `› ${bold('[@breeztech/react-native/expo]')}`;
}

export function warnOnce(message: string): void {
  if (!warningMap.has(message)) {
    warningMap.set(message, true);
    // eslint-disable-next-line no-console
    console.warn(yellow(`${logPrefix()} ${message}`));
  }
}

export function yellow(message: string): string {
  return `\x1b[33m${message}\x1b[0m`;
}

export function bold(message: string): string {
  return `\x1b[1m${message}\x1b[22m`;
}

export const sdkPackage: {
  name: string;
  version: string;
  // eslint-disable-next-line @typescript-eslint/no-var-requires
} = require('../../package.json');
