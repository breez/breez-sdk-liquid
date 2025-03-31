## Publishing

### Build
On first usage you will need to run:
```
make init
```

Then to build the Wasm code:
```
make build
```

#### Generated artifacts
* Bundle - suitable for use with bundlers like Webpack
 >* bundle/package.json
 >* bundle/breez_sdk_liquid_wasm.d.ts
 >* bundle/breez_sdk_liquid_wasm.js
 >* bundle/breez_sdk_liquid_wasm_bg.js
 >* bundle/breez_sdk_liquid_wasm_bg.wasm
 >* bundle/breez_sdk_liquid_wasm_bg.wasm.d.ts
* Deno - ES module for use with Deno
 >* deno/breez_sdk_liquid_wasm.d.ts
 >* deno/breez_sdk_liquid_wasm.js
 >* deno/breez_sdk_liquid_wasm_bg.wasm
 >* deno/breez_sdk_liquid_wasm_bg.wasm.d.ts
* Node - CommonJS module for use with Node.js
 >* node/package.json
 >* node/breez_sdk_liquid_wasm.d.ts
 >* node/breez_sdk_liquid_wasm.js
 >* node/breez_sdk_liquid_wasm_bg.wasm
 >* node/breez_sdk_liquid_wasm_bg.wasm.d.ts
* Web - ES module for use in browsers
 >* web/package.json
 >* web/breez_sdk_liquid_wasm.d.ts
 >* web/breez_sdk_liquid_wasm.js
 >* web/breez_sdk_liquid_wasm_bg.wasm
 >* web/breez_sdk_liquid_wasm_bg.wasm.d.ts

### Publish
When publishing, make sure the following are updated:
- Update the version number in `package.json`.
- Set the published version of `@breeztech/breez-sdk-liquid` in the each example's `package.json` file. 

Then login to npm:
```
npm login --@scope=@breeztech
```
Then publish:
```
npm publish --access public
```