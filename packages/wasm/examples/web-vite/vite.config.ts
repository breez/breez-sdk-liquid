import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  assetsInclude: ['**/*.wasm'],
  // prevent prebundle that breaks `new URL(..., import.meta.url)` in deps
  optimizeDeps: {
    exclude: ['@breeztech/breez-sdk-liquid'],
  },
})
