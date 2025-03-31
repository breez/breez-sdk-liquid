/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly VITE_BREEZ_API_KEY: string
    readonly VITE_MNEMONIC: string
}

interface ImportMeta {
    readonly env: ImportMetaEnv
}
