#import "BreezSDKLiquidPlugin.h"
#import "breez_sdk_liquid.h"
#import "breez_sdk_liquidFFI.h"
#if __has_include(<flutter_breez_liquid/flutter_breez_liquid-Swift.h>)
#import <flutter_breez_liquid/flutter_breez_liquid-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "flutter_breez_liquid-Swift.h"
#endif

@implementation BreezSDKLiquidPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {  
  dummy_method_to_enforce_bundling();
}
@end
