Breez Liquid SDK release notes can be found at [breez-sdk-liquid/releases](https://github.com/breez/breez-sdk-liquid/releases/)

## [TBD]

### BREAKING CHANGES
- `initialize()` is now `FlutterBreezLiquid.init()`
- `BindingLiquidSdk` renamed to `BreezSdkLiquid`

### Migration
```dart
// Before
await initialize();
BindingLiquidSdk sdk;

// After  
await FlutterBreezLiquid.init();
BreezSdkLiquid sdk;
```