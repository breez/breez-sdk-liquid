Breez Liquid SDK release notes can be found at [breez-sdk-liquid/releases](https://github.com/breez/breez-sdk-liquid/releases/)

## [TBD]

### BREAKING CHANGES
- `initialize()` is now `FlutterBreezLiquid.init()`
- `BindingLiquidSdk` renamed to `BreezSdkLiquid`
- If your app uses a Notification Service Extension (NSE), you may need to re-add the SDK’s NSE source files to the Compile Sources build phase of your NSE target.

### Migration
```dart
// Before
await initialize();
BindingLiquidSdk sdk;

// After  
await FlutterBreezLiquid.init();
BreezSdkLiquid sdk;
```